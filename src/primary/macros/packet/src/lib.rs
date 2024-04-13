use proc_macro::{TokenStream};
use proc_macro2::{Ident};
use syn::ItemStruct;
use syn::{parse_macro_input};
use quote::{quote, format_ident};

mod types;

use types::{Attributes, Imports};

#[proc_macro_derive(LoginPacket, attributes(options, dynamic_field))]
pub fn derive_login_packet(input: TokenStream) -> TokenStream {
    let ItemStruct { ident, fields, attrs, .. } = parse_macro_input!(input);
    let Imports {
        buf_read,
        binary_converter,
        byteorder_write,
        cursor,
        json_formatter,
        result,
        serialize,
        stream_reader,
        ..
    } = Imports::get();

    let mut with_async = false;
    if attrs.iter().any(|attr| attr.path().is_ident("options")) {
        let attributes = attrs.iter().next().unwrap();
        let attrs: Attributes = attributes.parse_args().unwrap();

        if let Some(_span) = attrs.with_async.span {
            with_async = true;
        }
    }

    let field_names = fields.iter().map(|f| {
        f.ident.clone()
    }).collect::<Vec<Option<Ident>>>();

    let mut dynamic_fields: Vec<Option<Ident>> = vec![];
    for field in fields.iter() {
        let ident = format_ident!("{}", field.ident.as_ref().unwrap());

        if field.attrs.iter().any(|attr| attr.path().is_ident("dynamic_field")) {
            dynamic_fields.push(Some(ident));
        }
    }

    let initializers = fields
        .iter()
        .map(|f| {
            let field_name = f.ident.clone();
            let field_type = f.ty.clone();

            if dynamic_fields.contains(&field_name) {
                quote!{ Self::#field_name(&mut reader, &mut cache) }
            } else {
                quote! {
                    {
                        let value: #field_type = #binary_converter::read_from(&mut reader)?;
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

    if with_async {
        let async_initializers = fields
            .iter()
            .map(|f| {
                let field_name = f.ident.clone();
                let field_type = f.ty.clone();

                if dynamic_fields.contains(&field_name) {
                    let async_field_name = format_ident!("async_{}", field_name.unwrap());
                    quote!{ Self::#async_field_name(&mut stream, &mut cache).await }
                } else {
                    quote! {
                    {
                        let value: #field_type = #stream_reader::read_from(&mut stream).await?;
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
                    where R: #buf_read + Unpin + Send
                {
                    let mut cache = Self {
                        #(#field_names: Default::default()),*
                    };

                    let mut instance = Self {
                        #(#field_names: #async_initializers),*
                    };

                    Ok(instance._build_body()?)
                }
            }
        }
    }

    TokenStream::from(output)
}

#[proc_macro_derive(WorldPacket, attributes(options, dynamic_field))]
pub fn derive_world_packet(input: TokenStream) -> TokenStream {
    let ItemStruct { ident, fields, attrs, .. } = parse_macro_input!(input);
    let Imports {
        binary_converter,
        byteorder_be,
        byteorder_le,
        byteorder_write,
        cursor,
        deflate_decoder,
        json_formatter,
        read,
        result,
        serialize,
        ..
    } = Imports::get();

    let mut is_compressed = quote!(false);
    if attrs.iter().any(|attr| attr.path().is_ident("options")) {
        let attributes = attrs.iter().next().unwrap();
        let attrs: Attributes = attributes.parse_args().unwrap();

        if let Some(_span) = attrs.compressed.span {
            is_compressed = quote!(true);
        }
    }

    let field_names = fields.iter().map(|f| {
        f.ident.clone()
    }).collect::<Vec<Option<Ident>>>();

    let mut dynamic_fields: Vec<Option<Ident>> = vec![];
    for field in fields.iter() {
        let ident = format_ident!("{}", field.ident.as_ref().unwrap());

        if field.attrs.iter().any(|attr| attr.path().is_ident("dynamic_field")) {
            dynamic_fields.push(Some(ident));
        }
    }

    let initializers = fields
        .iter()
        .map(|f| {
            let field_name = f.ident.clone();
            let field_type = f.ty.clone();

            if dynamic_fields.contains(&field_name) {
                quote!{ Self::#field_name(&mut reader, &mut cache) }
            } else {
                quote! {
                    {
                        let value: #field_type = #binary_converter::read_from(&mut reader)?;
                        cache.#field_name = value.clone();
                        value
                    }
                }
            }
        });

    let output = quote! {
        impl #ident {
            pub fn from_binary(buffer: &[u8]) -> #result<(Self, String)> {
                let mut buffer = match #is_compressed {
                    true => {
                        let mut internal_buffer: Vec<u8> = Vec::new();
                        // 4 bytes uncompressed + 2 bytes used by zlib
                        let omit_bytes = 6;
                        let data = &buffer[omit_bytes..];
                        let mut decoder = #deflate_decoder::new(data);
                        #read::read_to_end(&mut decoder, &mut internal_buffer)?;

                        internal_buffer.to_vec()
                    },
                    false => buffer.to_vec(),
                };

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

            pub fn to_binary_with_opcode(&mut self, opcode: u32) -> #result<Vec<u8>> {
                let body = self._build_body()?;
                let header = Self::_build_header(body.len(), opcode)?;
                Ok([header, body].concat())
            }

            pub fn unpack_with_opcode(&mut self, opcode: u32) -> #result<(u32, Vec<u8>, String)> {
                Ok((opcode, self.to_binary_with_opcode(opcode)?, self.get_json_details()?))
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

            fn _build_header(body_len: usize, opcode: u32) -> #result<Vec<u8>> {
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
        }
    };

    TokenStream::from(output)
}

#[proc_macro_derive(FieldsSerializer, attributes(dynamic_field))]
pub fn derive_fields_serializer(input: TokenStream) -> TokenStream {
    let ItemStruct { ident, fields, .. } = parse_macro_input!(input);
    let Imports {
        binary_converter,
        cursor,
        json_formatter,
        result,
        serialize,
        ..
    } = Imports::get();

    let field_names = fields.iter().map(|f| {
        f.ident.clone()
    }).collect::<Vec<Option<Ident>>>();

    let mut dynamic_fields: Vec<Option<Ident>> = vec![];
    for field in fields.iter() {
        let ident = format_ident!("{}", field.ident.as_ref().unwrap());

        if field.attrs.iter().any(|attr| attr.path().is_ident("dynamic_field")) {
            dynamic_fields.push(Some(ident));
        }
    }

    let initializers = fields
        .iter()
        .map(|f| {
            let field_name = f.ident.clone();
            let field_type = f.ty.clone();

            if dynamic_fields.contains(&field_name) {
                quote!{ Self::#field_name(&mut reader, &mut cache) }
            } else {
                quote! {
                    {
                        let value: #field_type = #binary_converter::read_from(&mut reader)?;
                        cache.#field_name = value.clone();
                        value
                    }
                }
            }
        });

    let output = quote! {
        impl #ident {
            pub fn from_binary(buffer: &[u8]) -> #result<(Self, String)> {
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

            pub fn get_json_details(&mut self) -> #result<String> {
                let mut serializer = #json_formatter::init();
                #serialize::serialize(self, &mut serializer)?;
                String::from_utf8(serializer.into_inner()).map_err(|e| e.into())
            }

            pub fn to_binary(&mut self) -> #result<Vec<u8>> {
                let mut body = Vec::new();
                #(
                    #binary_converter::write_into(
                        &mut self.#field_names,
                        &mut body
                    )?;
                )*

                Ok(body)
            }
        }
    };

    TokenStream::from(output)
}