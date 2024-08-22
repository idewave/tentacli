use proc_macro::{TokenStream};
use proc_macro2::{Ident};
use quote::{quote};
use std::collections::BTreeMap;
use syn::{parse_macro_input, Token, ItemStruct};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

mod types;

use types::Imports;

struct DependsOnAttribute {
    pub name: Ident,
}

impl Parse for DependsOnAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;

        Ok(Self { name })
    }
}

/// LoginPacket is a part of tentacli-based projects.
/// This proc-macro allows to send and receive packets from WoW Login server.
/// `#[depends_on]` attribute indicates that the field depends on another fields. These fields will be
/// converted into bytes and the byte-array will be used as dependency for `read_from` method of
/// BinaryConverter

#[proc_macro_derive(LoginPacket, attributes(depends_on))]
pub fn login_packet(input: TokenStream) -> TokenStream {
    let ItemStruct { ident, fields, .. } = parse_macro_input!(input);
    let Imports {
        async_buf_read,
        binary_converter,
        byteorder_write,
        cursor,
        json_formatter,
        result,
        serialize,
        stream_reader,
        ..
    } = Imports::get();

    let field_names = fields.iter().map(|f| {
        f.ident.clone()
    }).collect::<Vec<Option<Ident>>>();

    let mut depends_on: BTreeMap<Option<Ident>, Vec<Ident>> = BTreeMap::new();
    for field in fields.iter() {
        let ident = field.ident.clone();

        if field.attrs.iter().any(|attr| attr.path().is_ident("depends_on")) {
            let mut dependencies: Vec<Ident> = vec![];

            field.attrs.iter().for_each(|attr| {
                if attr.path().is_ident("depends_on") {
                    let parsed_attrs = attr.parse_args_with(
                        Punctuated::<DependsOnAttribute, Token![,]>::parse_terminated
                    ).unwrap();

                    for a in parsed_attrs {
                        dependencies.push(a.name);
                    }
                }
            });

            depends_on.insert(ident, dependencies);
        }
    }

    let initializers = fields
        .iter()
        .map(|f| {
            let field_name = f.ident.clone();
            let field_type = f.ty.clone();

            if let Some(dep_fields) = depends_on.get(&field_name) {
                quote! {
                    {
                        let mut data: Vec<u8> = vec![];
                        #(
                            #binary_converter::write_into(
                                &mut cache.#dep_fields,
                                &mut data,
                            )?;
                        )*
                        #binary_converter::read_from(&mut reader, &mut data)?
                    }
                }
            } else {
                quote! {
                    {
                        let value: #field_type = #binary_converter::read_from(&mut reader, &mut vec![])?;
                        cache.#field_name = value.clone();
                        value
                    }
                }
            }
        });

    let mut output = quote! {
        impl #ident {
            pub fn from_binary(buffer: &[u8]) -> #result<(Self, String)> {
                let mut cache = Self {
                    #(#field_names: Default::default()),*
                };

                let mut reader = #cursor::new(buffer.to_vec());
                let mut instance = Self {
                    #(#field_names: #initializers),*
                };
                let details = instance.get_json_details()?;

                Ok((instance, details))
            }

            pub fn to_binary_with_opcode(&mut self, opcode: u8) -> #result<Vec<u8>> {
                let body = self._build_body()?;
                let header = Self::_build_header(opcode)?;
                Ok([header, body].concat())
            }

            pub fn unpack_with_opcode(&mut self, opcode: u8) -> #result<(u32, Vec<u8>, String)> {
                Ok((opcode as u32, self.to_binary_with_opcode(opcode)?, self.get_json_details()?))
            }

            pub fn get_json_details(&mut self) -> #result<String> {
                let mut serializer = #json_formatter::init();
                #serialize::serialize(self, &mut serializer)?;
                String::from_utf8(serializer.into_inner()).map_err(|e| e.into())
            }

            fn _build_body(&mut self) -> #result<Vec<u8>> {
                let mut body = Vec::new();
                #(
                    #binary_converter::write_into(
                        &mut self.#field_names,
                        &mut body
                    )?;
                )*

                Ok(body)
            }

            fn _build_header(opcode: u8) -> #result<Vec<u8>> {
                let mut header: Vec<u8> = Vec::new();
                #byteorder_write::write_u8(
                    &mut header,
                    opcode,
                )?;

                Ok(header)
           }
        }
    };

    let async_initializers = fields
        .iter()
        .map(|f| {
            let field_name = f.ident.clone();
            let field_type = f.ty.clone();

            if let Some(dep_fields) = depends_on.get(&field_name) {
                quote! {
                    {
                        let mut data: Vec<u8> = vec![];
                        #(
                            #binary_converter::write_into(
                                &mut cache.#dep_fields,
                                &mut data,
                            )?;
                        )*
                        #stream_reader::read_from(&mut stream, &mut data).await?
                    }
                }
            } else {
                quote! {
                    {
                        let value: #field_type = #stream_reader::read_from(&mut stream, &mut vec![]).await?;
                        cache.#field_name = value.clone();
                        value
                    }
                }
            }
        });

    output = quote! {
        #output

        impl #ident {
            pub async fn from_stream<R>(mut stream: &mut R) -> #result<Vec<u8>>
                where R: #async_buf_read + Unpin + Send
            {
                let mut cache = Self {
                    #(#field_names: Default::default()),*
                };

                let mut instance = Self {
                    #(#field_names: #async_initializers),*
                };

                instance._build_body()
            }
        }
    };

    TokenStream::from(output)
}

/// WorldPacket is a part of tentacli-based projects.
/// This proc-macro allows to send and receive packets from WoW World server.
/// `#[depends_on]` attribute indicates that the field depends on another fields. These fields will be
/// converted into bytes and the byte-array will be used as dependency for `read_from` method of
/// `BinaryConverter`.
/// `#[conditional]` attribute indicates that the field's value will be parsed only if condition
/// is true. The result for condition will be parsed from implemented method with same name as
/// field name.

#[proc_macro_derive(WorldPacket, attributes(depends_on, conditional))]
pub fn world_packet(input: TokenStream) -> TokenStream {
    let ItemStruct { ident, fields, .. } = parse_macro_input!(input);
    let Imports {
        binary_converter,
        byteorder_be,
        byteorder_le,
        byteorder_write,
        cursor,
        json_formatter,
        result,
        serialize,
        utils,
        ..
    } = Imports::get();

    let field_names = fields.iter().map(|f| {
        f.ident.clone()
    }).collect::<Vec<Option<Ident>>>();

    let mut depends_on: BTreeMap<Option<Ident>, Vec<Ident>> = BTreeMap::new();
    let mut conditional: Vec<Option<Ident>> = vec![];
    for field in fields.iter() {
        let ident = field.ident.clone();

        if field.attrs.iter().any(|attr| attr.path().is_ident("depends_on")) {
            let mut dependencies: Vec<Ident> = vec![];

            field.attrs.iter().for_each(|attr| {
                if attr.path().is_ident("depends_on") {
                    let parsed_attrs = attr.parse_args_with(
                        Punctuated::<DependsOnAttribute, Token![,]>::parse_terminated
                    ).unwrap();

                    for a in parsed_attrs {
                        dependencies.push(a.name);
                    }
                }
            });

            depends_on.insert(ident.clone(), dependencies);
        }

        if field.attrs.iter().any(|attr| attr.path().is_ident("conditional")) {
            conditional.push(ident);
        }
    }

    let initializers = fields
        .iter()
        .map(|f| {
            let field_name = f.ident.clone();
            let field_type = f.ty.clone();

            let output = if let Some(dep_fields) = depends_on.get(&field_name) {
                quote! {
                    {
                        let mut data: Vec<u8> = vec![];
                        #(
                            #binary_converter::write_into(
                                &mut cache.#dep_fields,
                                &mut data,
                            )?;
                        )*
                        #binary_converter::read_from(&mut reader, &mut data)?
                    }
                }
            } else {
                quote! {
                    {
                        let value: #field_type = #binary_converter::read_from(
                            &mut reader, &mut vec![]
                        ).unwrap_or_default();

                        cache.#field_name = value.clone();
                        value
                    }
                }
            };

            if conditional.contains(&field_name) {
                quote! {
                    {
                        if Self::#field_name(&mut cache) {
                            #output
                        } else {
                            Default::default()
                        }
                    }
                }
            } else {
                output
            }
        });

    let writable_fields = fields.iter().map(|f| {
        let field_name = f.ident.clone();

        if conditional.contains(&field_name) {
            quote! {
                if Self::#field_name(self) {
                    #binary_converter::write_into(&mut self.#field_name, &mut body)?;
                }
            }
        } else {
            quote! {
                #binary_converter::write_into(&mut self.#field_name, &mut body)?;
            }
        }
    });

    let output = quote! {
        impl #ident {
            pub fn from_binary(buffer: &[u8]) -> #result<(Self, String)> {
                Self::build_instance(buffer)
            }

            pub fn from_compressed_binary(buffer: &[u8]) -> #result<(Self, String)> {
                let mut buffer = #utils::deflate_decompress(&buffer[6..])?;
                Self::build_instance(&buffer)
            }

            pub fn to_binary_with_server_opcode(
                &mut self,
                opcode: u16
            ) -> #result<Vec<u8>> {
                let body = self._build_body()?;
                let header = Self::_build_header_for_server_packet(body.len(), opcode)?;
                Ok([header, body].concat())
            }

            pub fn to_binary_with_client_opcode(&mut self, opcode: u32) -> #result<Vec<u8>> {
                let body = self._build_body()?;
                let header = Self::_build_header_for_client_packet(body.len(), opcode)?;
                Ok([header, body].concat())
            }

            pub fn unpack_with_server_opcode(
                &mut self,
                opcode: u16
            ) -> #result<(u16, Vec<u8>, String)> {
                Ok((opcode,
                    self.to_binary_with_server_opcode(opcode)?,
                    self.get_json_details()?))
            }

            pub fn unpack_with_client_opcode(&mut self, opcode: u32) -> #result<(u32, Vec<u8>, String)> {
                Ok((opcode, self.to_binary_with_client_opcode(opcode)?, self.get_json_details()?))
            }

            pub fn get_json_details(&mut self) -> #result<String> {
                let mut serializer = #json_formatter::init();
                #serialize::serialize(self, &mut serializer)?;
                String::from_utf8(serializer.into_inner()).map_err(|e| e.into())
            }

            fn _build_body(&mut self) -> #result<Vec<u8>> {
                let mut body = Vec::new();
                #(#writable_fields)*

                Ok(body)
            }

            fn _build_header_for_server_packet(body_len: usize, opcode: u16) -> #result<Vec<u8>> {
                let mut header: Vec<u8> = Vec::new();

                let is_large_packet = body_len > 0x7FFF;

                #byteorder_write::write_u16::<#byteorder_be>(
                    &mut header,
                    // header is 2 bytes packet size + 2 bytes outcoming opcode size
                    (body_len as u16) + 2,
                )?;

                #byteorder_write::write_u16::<#byteorder_le>(
                    &mut header,
                    opcode,
                )?;

                if is_large_packet {
                    header.insert(0, 128);
                }

                Ok(header)
           }

            fn _build_header_for_client_packet(body_len: usize, opcode: u32) -> #result<Vec<u8>> {
                let mut header: Vec<u8> = Vec::new();
                #byteorder_write::write_u16::<#byteorder_be>(
                    &mut header,
                    // header is 2 bytes packet size + 4 bytes outcoming opcode size
                    (body_len as u16) + 4,
                )?;

                #byteorder_write::write_u32::<#byteorder_le>(
                    &mut header,
                    opcode,
                )?;

                Ok(header)
           }

            fn build_instance(buffer: &[u8]) -> #result<(Self, String)> {
                let mut cache = Self {
                    #(#field_names: Default::default()),*
                };

                let mut reader = #cursor::new(buffer);
                let mut instance = Self {
                    #(#field_names: #initializers),*
                };

                let details = instance.get_json_details()?;

                Ok((instance, details))
            }
        }
    };

    TokenStream::from(output)
}

/// Segment is a part of tentacli-based projects.
/// This proc-macro is used mostly for serialization, when there's a need to serialize set of fields
/// into byte-array. Can be useful for sharing duplicated parts between different packets.
/// `#[depends_on]` attribute indicates that the field depends on another fields. These fields will be
/// converted into bytes and the byte-array will be used as dependency for `read_from` method of
/// `BinaryConverter`.
/// `#[conditional]` attribute indicates that the field's value will be parsed only if condition
/// is true. The result for condition will be parsed from implemented method with same name as
/// field name.

#[proc_macro_derive(Segment, attributes(depends_on, conditional))]
pub fn segment(input: TokenStream) -> TokenStream {
    let ItemStruct { ident, fields, .. } = parse_macro_input!(input);
    let Imports {
        binary_converter,
        buf_read,
        result,
        ..
    } = Imports::get();

    let field_names = fields.iter().map(|f| {
        f.ident.clone()
    }).collect::<Vec<Option<Ident>>>();

    let mut depends_on: BTreeMap<Option<Ident>, Vec<Ident>> = BTreeMap::new();
    let mut conditional: Vec<Option<Ident>> = vec![];
    for field in fields.iter() {
        let ident = field.ident.clone();

        if field.attrs.iter().any(|attr| attr.path().is_ident("depends_on")) {
            let mut dependencies: Vec<Ident> = vec![];

            field.attrs.iter().for_each(|attr| {
                if attr.path().is_ident("depends_on") {
                    let parsed_attrs = attr.parse_args_with(
                        Punctuated::<DependsOnAttribute, Token![,]>::parse_terminated
                    ).unwrap();

                    for a in parsed_attrs {
                        dependencies.push(a.name);
                    }
                }
            });

            depends_on.insert(ident.clone(), dependencies);
        }

        if field.attrs.iter().any(|attr| attr.path().is_ident("conditional")) {
            conditional.push(ident);
        }
    }

    let initializers = fields
        .iter()
        .map(|f| {
            let field_name = f.ident.clone();
            let field_type = f.ty.clone();

            let output = if let Some(dep_fields) = depends_on.get(&field_name) {
                quote! {
                    {
                        let mut data: Vec<u8> = vec![];
                        #(
                            #binary_converter::write_into(
                                &mut cache.#dep_fields,
                                &mut data,
                            )?;
                        )*
                        #binary_converter::read_from(&mut reader, &mut data)?
                    }
                }
            } else {
                quote! {
                    {
                        let value: #field_type = #binary_converter::read_from(&mut reader, &mut vec![])?;
                        cache.#field_name = value.clone();
                        value
                    }
                }
            };

            if conditional.contains(&field_name) {
                quote! {
                    {
                        if Self::#field_name(&mut cache) {
                            #output
                        } else {
                            Default::default()
                        }
                    }
                }
            } else {
                output
            }
        });

    let writable_fields = fields.iter().map(|f| {
        let field_name = f.ident.clone();

        if conditional.contains(&field_name) {
            quote! {
                if Self::#field_name(self) {
                    #binary_converter::write_into(&mut self.#field_name, &mut body)?;
                }
            }
        } else {
            quote! {
                #binary_converter::write_into(&mut self.#field_name, &mut body)?;
            }
        }
    });

    let output = quote! {
        impl #ident {
            pub fn read_from<R: #buf_read>(mut reader: &mut R) -> #result<Self> {
                let mut cache = Self {
                    #(#field_names: Default::default()),*
                };

                let mut instance = Self {
                    #(#field_names: #initializers),*
                };

                Ok(instance)
            }

            pub fn to_binary(&mut self) -> #result<Vec<u8>> {
                let mut body = Vec::new();
                #(#writable_fields)*

                Ok(body)
            }
        }

        impl #binary_converter for #ident {
            fn write_into(&mut self, buffer: &mut Vec<u8>) -> #result<()> {
                buffer.extend(self.to_binary()?);
                Ok(())
            }

            fn read_from<R: #buf_read>(reader: &mut R, _: &mut Vec<u8>) -> #result<Self> {
                Self::read_from(reader)
            }
        }
    };

    TokenStream::from(output)
}