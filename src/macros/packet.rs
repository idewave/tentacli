#[macro_export]
macro_rules! packet {
    (
        $(@option[login_opcode=$login_opcode:expr])?
        $(@option[world_opcode=$world_opcode:expr])?
        $(@option[compressed_if:$compressed_if:expr])?
        $(@option[dynamic_fields:$($dynamic_fields:expr),*])?

        $(#[$outer:meta])*
        $vis:vis struct $PacketStruct:ident {
            $($field_vis:vis $field_name:ident: $field_type:ty),*$(,)?
        }

        $($PacketStructImpl: item)*
    ) => {
        $(#[$outer])*
        #[derive(Clone, Debug, Default)]
        $vis struct $PacketStruct {
            $($field_vis $field_name: $field_type),*
        }

        $($PacketStructImpl)*

        impl $PacketStruct {
            #[allow(dead_code)]
            pub fn from_binary(buffer: &Vec<u8>) -> Self {
                #![allow(unused_mut)]
                #![allow(unused_variables)]
                #![allow(unused_assignments)]
                let mut omit_bytes: usize = $crate::crypto::decryptor::INCOMING_HEADER_LENGTH;
                $(
                    omit_bytes = ($login_opcode as u8).to_le_bytes().len();
                )?

                let mut is_compressed = false;
                $(
                    let mut reader = std::io::Cursor::new(buffer[2..].to_vec());
                    let opcode = byteorder::ReadBytesExt::read_u16::<byteorder::LittleEndian>(
                        &mut reader
                    ).unwrap();

                    if $compressed_if == opcode {
                        // 4 bytes uncompressed + 2 bytes used by zlib
                        omit_bytes += 6;
                        is_compressed = true;
                    }
                )?

                let mut internal_buffer: Vec<u8> = Vec::new();
                if is_compressed {
                    let data = &buffer[omit_bytes..];
                    let mut decoder = flate2::read::DeflateDecoder::new(data);
                    std::io::Read::read_to_end(&mut decoder, &mut internal_buffer).unwrap();
                }

                let buffer = if internal_buffer.is_empty() {
                    buffer[omit_bytes..].to_vec()
                } else {
                    internal_buffer
                };

                let mut dynamic_fields: Vec<&str> = vec![];
                $(
                    $(
                        for index in 0..$dynamic_fields.len() {
                            dynamic_fields.push($dynamic_fields[index]);
                        }
                    )*
                )?

                let mut reader = std::io::Cursor::new(&buffer);

                let initial = Self {
                    $(
                        $field_name: if dynamic_fields.contains(&stringify!($field_name)) {
                            // ... do calculations
                            <$field_type>::default()
                        } else {
                            $crate::traits::binary_converter::BinaryConverter::read_from(
                                &mut reader
                            ).unwrap()
                        }
                    ),*
                };

                initial
            }

            #[allow(dead_code)]
            pub fn to_binary(&mut self) -> Vec<u8> {
                #![allow(unused_mut)]
                let mut body = Vec::new();
                $(
                    $crate::traits::binary_converter::BinaryConverter::write_into(
                        &mut self.$field_name,
                        &mut body
                    ).unwrap();
                )*
                let header = Self::_build_header(&body);
                [header, body].concat()
            }

            #[allow(unused_variables)]
            fn _build_header(body: &Vec<u8>) -> Vec<u8> {
                #![allow(unused_mut)]
                let mut header: Vec<u8> = Vec::new();
                $(
                    byteorder::WriteBytesExt::write_u8(
                        &mut header,
                        $login_opcode as u8
                    ).unwrap();
                )?
                $(
                    let size = body.len() + $crate::crypto::encryptor::OUTCOMING_OPCODE_LENGTH;

                    byteorder::WriteBytesExt::write_u16::<byteorder::BigEndian>(
                        &mut header,
                        size as u16,
                    ).unwrap();
                    byteorder::WriteBytesExt::write_u32::<byteorder::LittleEndian>(
                        &mut header,
                        $world_opcode as u32
                    ).unwrap();
                )?

                header
           }

            $(
                #[allow(dead_code)]
                pub fn unpack(&mut self) -> $crate::types::PacketOutcome {
                    ($login_opcode as u32, self.to_binary())
                }
            )?

            $(
                #[allow(dead_code)]
                pub fn unpack(&mut self) -> $crate::types::PacketOutcome {
                    ($world_opcode as u32, self.to_binary())
                }
            )?
        }
    };
}