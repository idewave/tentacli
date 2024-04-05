use proc_macro2::{TokenStream as TokenStream2};
use quote::{quote};
use structmeta::{Flag, StructMeta};

pub struct Imports {
    pub async_read: TokenStream2,
    pub binary_converter: TokenStream2,
    pub buf_read: TokenStream2,
    pub byteorder_be: TokenStream2,
    pub byteorder_le: TokenStream2,
    pub byteorder_write: TokenStream2,
    pub cursor: TokenStream2,
    pub deflate_decoder: TokenStream2,
    pub json_formatter: TokenStream2,
    pub read: TokenStream2,
    pub result: TokenStream2,
    pub serialize: TokenStream2,
    pub stream_reader: TokenStream2,
}

impl Imports {
    pub fn get() -> Self {
        Self {
            async_read: quote!(tokio::io::AsyncRead),
            binary_converter: quote!(crate::traits::BinaryConverter),
            buf_read: quote!(tokio::io::AsyncBufRead),
            byteorder_be: quote!(byteorder::BigEndian),
            byteorder_le: quote!(byteorder::LittleEndian),
            byteorder_write: quote!(byteorder::WriteBytesExt),
            cursor: quote!(std::io::Cursor),
            deflate_decoder: quote!(flate2::read::DeflateDecoder),
            json_formatter: quote!(idewave_formatters::JsonFormatter),
            // TODO: need to reorganize constants
            read: quote!(std::io::Read),
            result: quote!(anyhow::Result),
            serialize: quote!(serde::Serialize),
            stream_reader: quote!(crate::primary::traits::StreamReader),
        }
    }
}

#[derive(StructMeta, Debug)]
pub struct Attributes {
    pub compressed: Flag,
    pub no_opcode: Flag,
}