#[macro_export]
macro_rules! packet {
    (
        $(@option[login_opcode=$login_opcode_value:expr])?
        $(@option[world_opcode=$world_opcode_value:expr])?
        $(@option[compressed:$compressed_value:expr])?

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
            // income
            pub fn from_binary(buffer: &Vec<u8>) -> Self {
                let mut omit_bytes: usize = $crate::crypto::decryptor::INCOMING_HEADER_LENGTH;
                $(
                    omit_bytes = $login_opcode_value.to_le_bytes().len();
                )?
                $(
                    if $compressed_value {
                        // 4 bytes uncompressed + 2 bytes used by zlib
                        omit_bytes += 6;
                    }
                )?

                let mut internal_buffer: Vec<u8> = Vec::new();
                $(
                    if $compressed_value {
                        let data = &buffer[omit_bytes..];
                        let mut decoder = flate2::read::DeflateDecoder::new(data);
                        std::io::Read::read_to_end(&mut decoder, &mut internal_buffer).unwrap();
                    }
                )?

                let buffer = if internal_buffer.is_empty() {
                    buffer[omit_bytes..].to_vec()
                } else {
                    internal_buffer
                };

                let mut reader = std::io::Cursor::new(&buffer);

                Self {
                    $(
                        $field_name: $crate::traits::binary_converter::BinaryConverter::read_from(
                            &mut reader
                        ).unwrap()
                    ),*
                }
            }

            // outcome
            pub fn to_binary(&mut self) -> Vec<u8> {
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

            fn _build_header(body: &Vec<u8>) -> Vec<u8> {
                let mut header: Vec<u8> = Vec::new();
                $(
                    byteorder::WriteBytesExt::write_u8(
                        &mut header,
                        $login_opcode_value as u8
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
                        $world_opcode_value as u32
                    ).unwrap();
                )?

                header
           }

            $(
                pub fn unpack(&mut self) -> $crate::types::PacketOutcome {
                    ($login_opcode_value as u32, self.to_binary())
                }
            )?

            $(
                pub fn unpack(&mut self) -> $crate::types::PacketOutcome {
                    ($world_opcode_value as u32, self.to_binary())
                }
            )?
        }
    };
}