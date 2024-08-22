use proc_macro2::{TokenStream as TokenStream2};
use quote::{quote};

pub struct Imports {
    pub async_buf_read: TokenStream2,
    pub binary_converter: TokenStream2,
    pub buf_read: TokenStream2,
    pub byteorder_be: TokenStream2,
    pub byteorder_le: TokenStream2,
    pub byteorder_write: TokenStream2,
    pub cursor: TokenStream2,
    pub json_formatter: TokenStream2,
    pub result: TokenStream2,
    pub serialize: TokenStream2,
    pub stream_reader: TokenStream2,
    pub utils: TokenStream2,
}

impl Imports {
    pub fn get() -> Self {
        Self {
            async_buf_read: quote!(tokio::io::AsyncBufRead),
            binary_converter: quote!(tentacli_traits::BinaryConverter),
            buf_read: quote!(std::io::BufRead),
            byteorder_be: quote!(byteorder::BigEndian),
            byteorder_le: quote!(byteorder::LittleEndian),
            byteorder_write: quote!(byteorder::WriteBytesExt),
            cursor: quote!(std::io::Cursor),
            json_formatter: quote!(tentacli_formatters::JsonFormatter),
            result: quote!(anyhow::Result),
            serialize: quote!(serde::Serialize),
            stream_reader: quote!(tentacli_traits::StreamReader),
            utils: quote!(tentacli_utils),
        }
    }
}