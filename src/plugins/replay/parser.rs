use std::collections::HashSet;
use std::fs::File;

use anyhow::Result;
use chrono::{NaiveDateTime, TimeZone, Utc};
use memchr::memchr;
use memchr::memmem::find;
use memmap2::Mmap;

pub struct WorldLogReader {
    mmap: Mmap,
    offset: usize,
    opcode_filter: Vec<Vec<u8>>,
    dedup_enabled: bool,
    payload_buffer: Vec<u8>,
    current_timestamp: Option<i64>,
    seen_in_second: HashSet<u64>,
}

pub struct LogPacket {
    pub timestamp: Option<i64>,
    pub opcode: u16,
    pub payload: Vec<u8>,
}

impl WorldLogReader {
    pub fn open(path: &str, opcode_filter: &[String], dedup_enabled: bool) -> Result<Self> {
        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };

        Ok(Self {
            mmap,
            offset: 0,
            opcode_filter: opcode_filter.iter().map(|opcode| opcode.as_bytes().to_vec()).collect(),
            dedup_enabled,
            payload_buffer: Vec::new(),
            current_timestamp: None,
            seen_in_second: HashSet::new(),
        })
    }

    pub fn next_packet(&mut self) -> Option<LogPacket> {
        loop {
            if self.offset >= self.mmap.len() {
                return None;
            }

            let entry_start_offset = self.offset;
            let next_entry_start_offset =
                find_next_entry_start(&self.mmap, entry_start_offset + 1).unwrap_or(self.mmap.len());

            let block = &self.mmap[entry_start_offset..next_entry_start_offset];

            let (opcode_name_bytes, opcode_code) = match Self::extract_opcode(block) {
                Some(value) => value,
                None => {
                    self.offset = next_entry_start_offset;
                    continue;
                }
            };

            if !self.opcode_filter.is_empty()
                && !self
                .opcode_filter
                .iter()
                .any(|filter| filter.as_slice() == opcode_name_bytes)
            {
                self.offset = next_entry_start_offset;
                continue;
            }

            if !Self::extract_bytes(block, &mut self.payload_buffer) {
                self.offset = next_entry_start_offset;
                continue;
            }

            let timestamp = Self::extract_timestamp(block);

            if self.dedup_enabled {
                let should_keep = match timestamp {
                    Some(ts) => {
                        if self.current_timestamp != Some(ts) {
                            self.current_timestamp = Some(ts);
                            self.seen_in_second.clear();
                        }

                        let signature = dedup_signature_fnv1a(ts, opcode_code, &self.payload_buffer);
                        self.seen_in_second.insert(signature)
                    }
                    None => true,
                };

                if !should_keep {
                    self.offset = next_entry_start_offset;
                    continue;
                }
            }

            self.offset = next_entry_start_offset;

            return Some(LogPacket {
                timestamp,
                opcode: opcode_code,
                payload: std::mem::take(&mut self.payload_buffer),
            });
        }
    }

    #[inline]
    fn hex_to_bytes(input: &[u8], out: &mut Vec<u8>) -> bool {
        out.clear();

        let mut high_nibble: u8 = 0;
        let mut have_high: bool = false;

        for &byte in input {
            let value = match byte {
                b'0'..=b'9' => byte - b'0',
                b'a'..=b'f' => byte - b'a' + 10,
                b'A'..=b'F' => byte - b'A' + 10,
                b' ' | b'\n' | b'\r' | b'\t' => continue,
                _ => return false,
            };

            if have_high {
                out.push((high_nibble << 4) | value);
                have_high = false;
            } else {
                high_nibble = value;
                have_high = true;
            }
        }

        !have_high
    }

    #[inline]
    fn extract_opcode(block: &[u8]) -> Option<(&[u8], u16)> {
        const PREFIX: &[u8] = b"OPCODE: ";

        let start = find(block, PREFIX)?;
        let rest = &block[start + PREFIX.len()..];

        let open_paren = find(rest, b" (")?;
        let opcode_name = &rest[..open_paren];

        let after_paren = &rest[open_paren + 2..];
        let close_paren = memchr(b')', after_paren)?;

        let hex_part = &after_paren[..close_paren];
        if !hex_part.starts_with(b"0x") {
            return None;
        }

        let hex_digits = std::str::from_utf8(&hex_part[2..]).ok()?;
        let opcode_code = u16::from_str_radix(hex_digits, 16).ok()?;

        Some((opcode_name, opcode_code))
    }

    #[inline]
    fn extract_bytes(block: &[u8], out: &mut Vec<u8>) -> bool {
        const DATA_LF: &[u8] = b"DATA:\n";
        const DATA_CRLF: &[u8] = b"DATA:\r\n";

        if let Some(pos) = find(block, DATA_LF) {
            let payload = &block[pos + DATA_LF.len()..];
            return Self::hex_to_bytes(payload, out);
        }

        if let Some(pos) = find(block, DATA_CRLF) {
            let payload = &block[pos + DATA_CRLF.len()..];
            return Self::hex_to_bytes(payload, out);
        }

        false
    }

    #[inline]
    fn extract_timestamp(block: &[u8]) -> Option<i64> {
        let mut start = 0;
        while start < block.len() && matches!(block[start], b'\n' | b'\r') {
            start += 1;
        }

        let trimmed_block = &block[start..];
        let newline_pos = memchr(b'\n', trimmed_block)?;
        let mut ts_bytes = &trimmed_block[..newline_pos];

        while ts_bytes.last().is_some_and(|b| *b == b'\r' || *b == b' ' || *b == b'\t') {
            ts_bytes = &ts_bytes[..ts_bytes.len() - 1];
        }

        let ts_str = std::str::from_utf8(ts_bytes).ok()?;
        let naive = NaiveDateTime::parse_from_str(ts_str, "%Y-%m-%d %H:%M:%S").ok()?;

        Some(Utc.from_utc_datetime(&naive).timestamp())
    }
}

#[inline]
fn find_next_entry_start(haystack: &[u8], search_from: usize) -> Option<usize> {
    let slice = &haystack[search_from..];

    for i in 0..slice.len().saturating_sub(19) {
        if slice[i].is_ascii_digit()
            && &slice[i+4..i+5] == b"-"
            && &slice[i+7..i+8] == b"-"
            && &slice[i+10..i+11] == b" "
            && &slice[i+13..i+14] == b":"
            && &slice[i+16..i+17] == b":"
        {
            return Some(search_from + i);
        }
    }
    None
}

#[inline]
fn dedup_signature_fnv1a(timestamp: i64, opcode_code: u16, payload: &[u8]) -> u64 {
    const FNV_OFFSET_BASIS: u64 = 14695981039346656037;
    const FNV_PRIME: u64 = 1099511628211;

    let mut hash: u64 = FNV_OFFSET_BASIS;

    for byte in timestamp.to_le_bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }

    for byte in opcode_code.to_le_bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }

    for &byte in payload {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }

    hash
}
