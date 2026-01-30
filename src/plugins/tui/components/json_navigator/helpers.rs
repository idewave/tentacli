use std::collections::HashMap;
use ratatui::prelude::{Color, Line, Span, Style};
use serde_json::Value;

use crate::client::MetadataValue;
use crate::plugins::tui::components::json_navigator::cursor::ValueSpan;
use crate::plugins::tui::theme::{
    HEX_HIGHLIGHT_BG, HEX_HIGHLIGHT_FG, JSON_HIGHLIGHT_BG, JSON_HIGHLIGHT_FG
};

pub fn format_json(raw: &str) -> String {
    let Ok(value) = serde_json::from_str::<Value>(raw) else {
        return raw.to_string();
    };

    format_value(&value, 0)
}

fn format_value(value: &Value, indent: usize) -> String {
    let pad = "  ".repeat(indent);

    match value {
        Value::Object(map) => {
            if map.is_empty() {
                return "{}".to_string();
            }

            let mut out = String::from("{\n");
            let len = map.len();

            for (i, (k, v)) in map.iter().enumerate() {
                out.push_str(&format!(
                    "{}  \"{}\": {}",
                    pad,
                    k,
                    format_value(v, indent + 1)
                ));
                if i + 1 < len {
                    out.push(',');
                }
                out.push('\n');
            }

            out.push_str(&format!("{pad}}}"));
            out
        }

        Value::Array(arr) => {
            if arr.is_empty() {
                return "[]".to_string();
            }

            if arr.iter().all(|v| v.is_number() || v.is_boolean()) {
                format_array(arr, indent)
            } else {
                let mut out = String::from("[\n");
                let len = arr.len();

                for (i, v) in arr.iter().enumerate() {
                    out.push_str(&format!(
                        "{}  {}",
                        pad,
                        format_value(v, indent + 1)
                    ));
                    if i + 1 < len {
                        out.push(',');
                    }
                    out.push('\n');
                }

                out.push_str(&format!("{pad}]"));
                out
            }
        }

        _ => serde_json::to_string(value).unwrap_or_default(),
    }
}

fn format_array(arr: &[Value], indent: usize) -> String {
    const PER_LINE: usize = 8;

    let pad = "  ".repeat(indent);
    let mut out = String::from("[\n");

    for (i, chunk) in arr.chunks(PER_LINE).enumerate() {
        out.push_str(&format!(
            "{}  {}",
            pad,
            chunk
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ));

        if i + 1 < (arr.len() + PER_LINE - 1) / PER_LINE {
            out.push(',');
        }
        out.push('\n');
    }

    out.push_str(&format!("{pad}]"));
    out
}

pub fn hex_with_highlight(
    data: &[u8],
    offset: Option<MetadataValue>,
) -> Vec<Line<'_>> {
    const BYTES_PER_LINE: usize = 16;

    let (hl_start, hl_end) = offset
        .map(|h| (h.offset, h.offset + h.size))
        .unwrap_or((usize::MAX, usize::MAX));

    let mut lines = Vec::new();

    for (line_idx, chunk) in data.chunks(BYTES_PER_LINE).enumerate() {
        let line_offset = line_idx * BYTES_PER_LINE;
        let mut spans = Vec::new();

        spans.push(Span::styled(
            format!("{:04x}: ", line_offset),
            Style::new()
                .fg(Color::Gray)
                .add_modifier(ratatui::prelude::Modifier::BOLD),
        ));

        for (i, byte) in chunk.iter().enumerate() {
            let byte_pos = line_offset + i;
            let is_byte_highlighted = byte_pos >= hl_start && byte_pos < hl_end;

            let highlight_style = Style::new()
                .fg(HEX_HIGHLIGHT_FG)
                .bg(HEX_HIGHLIGHT_BG)
                .add_modifier(ratatui::prelude::Modifier::BOLD);

            let normal_style = Style::new().fg(Color::LightCyan);

            // Render byte (highlighted or normal)
            spans.push(Span::styled(
                format!("{:02x}", byte),
                if is_byte_highlighted { highlight_style } else { normal_style },
            ));

            // Render space AFTER the byte:
            // - keep space between bytes
            // - highlight the space only if it sits "inside" the highlighted range
            //   (i.e. current byte highlighted AND there is another highlighted byte after it)
            // - never highlight trailing space at end of line / after last highlighted byte
            if i + 1 < chunk.len() {
                let next_byte_pos = byte_pos + 1;
                let is_next_byte_highlighted = next_byte_pos >= hl_start && next_byte_pos < hl_end;

                let space_style = if is_byte_highlighted && is_next_byte_highlighted {
                    highlight_style
                } else {
                    Style::default()
                };

                spans.push(Span::styled(" ", space_style));
            }
        }

        lines.push(Line::from(spans));
    }

    lines
}

pub fn aggregate_field_span(
    offsets: &HashMap<String, MetadataValue>,
    path: &str,
) -> Option<MetadataValue> {
    let prefix = if path.is_empty() {
        "/".to_string()
    } else {
        format!("{}/", path)
    };

    let mut min_offset: Option<usize> = None;
    let mut max_end: Option<usize> = None;

    for (k, v) in offsets {
        if k.starts_with(&prefix) {
            let start = v.offset;
            let end = v.offset + v.size;

            min_offset = Some(min_offset.map_or(start, |m| m.min(start)));
            max_end = Some(max_end.map_or(end, |m| m.max(end)));
        }
    }

    match (min_offset, max_end) {
        (Some(start), Some(end)) if end > start => Some(MetadataValue {
            offset: start,
            size: end - start,
        }),
        _ => None,
    }
}

pub fn json_with_highlight(
    json: &str,
    highlight: Option<ValueSpan>,
) -> (Vec<Line<'_>>, Option<(usize, usize)>) {
    let mut lines = Vec::new();

    let mut highlight_start: Option<usize> = None;
    let mut highlight_end: Option<usize> = None;

    let (hl_start, hl_end) = highlight
        .map(|s| (s.offset, s.offset + s.size))
        .unwrap_or((usize::MAX, usize::MAX));

    let mut global_pos = 0;

    for (line_idx, raw_line) in json.lines().enumerate() {
        let line_len = raw_line.len();
        let line_start = global_pos;
        let line_end = global_pos + line_len;

        let mut spans = Vec::new();

        let intersects = !(hl_end <= line_start || hl_start >= line_end);

        if !intersects {
            spans.push(Span::raw(raw_line.to_string()));
        } else {
            highlight_start.get_or_insert(line_idx);
            highlight_end = Some(line_idx);

            let local_hl_start =
                hl_start.saturating_sub(line_start).min(line_len);
            let local_hl_end =
                hl_end.saturating_sub(line_start).min(line_len);

            if local_hl_start > 0 {
                spans.push(Span::raw(&raw_line[..local_hl_start]));
            }

            spans.push(Span::styled(
                &raw_line[local_hl_start..local_hl_end],
                Style::new()
                    .fg(JSON_HIGHLIGHT_FG)
                    .bg(JSON_HIGHLIGHT_BG)
                    .add_modifier(ratatui::prelude::Modifier::BOLD),
            ));

            if local_hl_end < line_len {
                spans.push(Span::raw(&raw_line[local_hl_end..]));
            }
        }

        lines.push(Line::from(spans));

        global_pos += line_len + 1;
    }

    let highlight_range = match (highlight_start, highlight_end) {
        (Some(start), Some(end)) => Some((start, end)),
        _ => None,
    };

    (lines, highlight_range)
}

pub fn ensure_visible(
    scroll: &mut u16,
    start: usize,
    end: usize,
    view_height: usize,
) {
    if view_height == 0 {
        return;
    }

    let scroll_usize = *scroll as usize;

    let span_h = end.saturating_sub(start) + 1;
    if span_h > view_height {
        if start < scroll_usize || start >= scroll_usize + view_height {
            *scroll = start as u16;
        }
        return;
    }

    if start < scroll_usize {
        *scroll = start as u16;
    } else if end >= scroll_usize + view_height {
        *scroll = (end + 1 - view_height) as u16;
    }
}

pub fn clamp_scroll(
    scroll: &mut u16,
    node_start: usize,
    node_end: usize,
    view_height: usize,
) {
    if view_height == 0 {
        return;
    }

    let min = node_start;
    let max = node_end.saturating_sub(view_height.saturating_sub(1));

    let max = max.max(min);

    let s = *scroll as usize;
    let clamped = s.clamp(min, max);

    *scroll = clamped as u16;
}