use proc_macro::{TokenStream};
use proc_macro2::{Ident};
use syn::ItemStruct;
use syn::{parse_macro_input};
use quote::{quote, format_ident};

mod types;

use types::{Attributes, Imports};

#[proc_macro_derive(LoginPacket, attributes(dynamic_field))]
pub fn derive_login_packet(input: TokenStream) -> TokenStream {
    let ItemStruct { ident, fields, .. } = parse_macro_input!(input);
    let Imports {
        binary_converter,
        byteorder_write,
        cursor,
        json_formatter,
        result,
        serialize,
        types,
        ..
    } = Imports::get();

    let field_names = fields.iter().map(|f| {
        f.ident.clone()
    }).collect::<Vec<Option<Ident>>>();

    let mut dynamic_fields: Vec<Option<Ident>> = vec![];
    for field in fields.iter() {
        let ident = format_ident!("{}", field.ident.as_ref().unwrap());

        if field.attrs.iter().any(|attr| attr.path.is_ident("dynamic_field")) {
            dynamic_fields.push(Some(ident));
        }
    }

    let initial_initializers = fields
        .iter()
        .map(|f| {
            let field_name = f.ident.clone();

            if dynamic_fields.contains(&field_name) {
                quote!{ Default::default() }
            } else {
                quote! { #binary_converter::read_from(&mut initial_reader)? }
            }
        });

    let initializers = fields
        .iter()
        .map(|f| {
            let field_name = f.ident.clone();
            let field_type = f.ty.clone();

            if dynamic_fields.contains(&field_name) {
                quote!{ Self::#field_name(&mut reader, &mut initial) }
            } else {
                quote! {
                    {
                        let value: #field_type = #binary_converter::read_from(&mut reader)?;
                        initial.#field_name = value.clone();
                        value
                    }
                }
            }
        });

    let output = quote! {
        impl #ident {
            pub fn from_binary(buffer: &[u8]) -> #result<(Self, String)> {
                let mut omit_bytes = Self::opcode().to_le_bytes().len();
                let mut initial_reader = #cursor::new(buffer[omit_bytes..].to_vec());
                let mut initial = Self {
                    #(#field_names: #initial_initializers),*
                };

                let mut reader = #cursor::new(buffer[omit_bytes..].to_vec());
                let mut instance = Self {
                    #(#field_names: #initializers),*
                };
                let details = instance.get_json_details()?;

                Ok((instance, details))
            }

            pub fn to_binary(&mut self) -> #result<Vec<u8>> {
                let body = self._build_body()?;

                let header = Self::_build_header(Self::opcode())?;
                Ok([header, body].concat())
            }

            pub fn unpack(&mut self) -> #result<#types::PacketOutcome> {
                Ok((Self::opcode() as u32, self.to_binary()?, self.get_json_details()?))
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
        incoming_header_length,
        json_formatter,
        read,
        result,
        serialize,
        types,
        ..
    } = Imports::get();

    let mut is_compressed = quote!(false);
    let mut has_opcode = true;
    if attrs.iter().any(|attr| attr.path.is_ident("options")) {
        let attributes = attrs.iter().next().unwrap();
        let attrs: Attributes = attributes.parse_args().unwrap();

        if let Some(_span) = attrs.compressed.span {
            is_compressed = quote!(true);
        }

        if let Some(_span) = attrs.no_opcode.span {
            has_opcode = false;
        }
    }

    let field_names = fields.iter().map(|f| {
        f.ident.clone()
    }).collect::<Vec<Option<Ident>>>();

    let mut dynamic_fields: Vec<Option<Ident>> = vec![];
    for field in fields.iter() {
        let ident = format_ident!("{}", field.ident.as_ref().unwrap());

        if field.attrs.iter().any(|attr| attr.path.is_ident("dynamic_field")) {
            dynamic_fields.push(Some(ident));
        }
    }

    let initial_initializers = fields
        .iter()
        .map(|f| {
            let field_name = f.ident.clone();

            if dynamic_fields.contains(&field_name) {
                quote!{ Default::default() }
            } else {
                quote! { #binary_converter::read_from(&mut initial_reader)? }
            }
        });

    let initializers = fields
        .iter()
        .map(|f| {
            let field_name = f.ident.clone();
            let field_type = f.ty.clone();

            if dynamic_fields.contains(&field_name) {
                quote!{ Self::#field_name(&mut reader, &mut initial) }
            } else {
                quote! {
                    {
                        let value: #field_type = #binary_converter::read_from(&mut reader)?;
                        initial.#field_name = value.clone();
                        value
                    }
                }
            }
        });

    let mut output = quote! {
        impl #ident {
            pub fn from_binary(buffer: &[u8]) -> #result<(Self, String)> {
                let mut omit_bytes = #incoming_header_length;

                let mut buffer = match #is_compressed {
                    true => {
                        let mut internal_buffer: Vec<u8> = Vec::new();
                        // 4 bytes uncompressed + 2 bytes used by zlib
                        omit_bytes += 6;

                        let data = &buffer[omit_bytes..];
                        let mut decoder = #deflate_decoder::new(data);
                        #read::read_to_end(&mut decoder, &mut internal_buffer)?;

                        internal_buffer.to_vec()
                    },
                    false => buffer[omit_bytes..].to_vec(),
                };

                let mut initial_reader = #cursor::new(buffer.to_vec());
                let mut initial = Self {
                    #(#field_names: #initial_initializers),*
                };

                let mut reader = #cursor::new(buffer);
                let mut instance = Self {
                    #(#field_names: #initializers),*
                };
                let details = instance.get_json_details()?;

                Ok((instance, details))
            }

            // use this method in case you didn't use with_opcode! macro
            pub fn to_binary_with_opcode(&mut self, opcode: u32) -> #result<Vec<u8>> {
                let body = self._build_body()?;
                let header = Self::_build_header(body.len(), opcode)?;
                Ok([header, body].concat())
            }

            pub fn unpack_with_opcode(&mut self, opcode: u32) -> #result<#types::PacketOutcome> {
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

    if has_opcode {
        output = quote! {
            #output

            impl #ident {
                pub fn to_binary(&mut self) -> #result<Vec<u8>> {
                    let body = self._build_body()?;

                    let header = Self::_build_header(body.len(), Self::opcode())?;
                    Ok([header, body].concat())
                }

                pub fn unpack(&mut self) -> #result<#types::PacketOutcome> {
                    Ok((Self::opcode(), self.to_binary()?, self.get_json_details()?))
                }
            }
        }
    }

    TokenStream::from(output)
}