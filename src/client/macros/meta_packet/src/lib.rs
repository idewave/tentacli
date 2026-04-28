use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, Error, Expr, Ident, Result, parse_macro_input};

#[proc_macro_derive(Packet, attributes(name, opcode))]
pub fn derive_packet(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let opts = match PacketOptions::from_attrs(&input.attrs) {
        Ok(o) => o,
        Err(e) => return e.to_compile_error().into(),
    };

    let ident = &input.ident;
    let generics = &input.generics;
    let where_clause = &generics.where_clause;

    let unpack_impl = if opts.has_br {
        quote! {
            impl #generics #ident #where_clause {
                pub fn unpack(
                    packet: &mut crate::client::packet::Packet
                ) -> ::anyhow::Result<Self>
                where
                    Self: ::binrw::BinRead
                        + ::serde::Serialize
                        + crate::client::packet::ExtractMetadata,
                {
                    let mut cursor =
                        ::std::io::Cursor::new(&packet.content.body);

                    let instance =
                        <Self as ::binrw::BinRead>::read(&mut cursor)?;

                    packet.set_offset_info(
                        crate::client::packet::ExtractMetadata
                            ::extract_metadata(&instance)
                    );

                    let json =
                        crate::client::packet
                            ::serialize_packet_json(&instance)?;

                    packet.set_json(json);
                    Ok(instance)
                }
            }
        }
    } else {
        quote! {}
    };

    let pack_impl = if opts.has_bw {
        let name = match &opts.name {
            Some(n) => n.clone(),
            None => {
                return Error::new_spanned(ident, "Missing #[name(...)] for outgoing packet")
                    .to_compile_error()
                    .into();
            }
        };

        let opcode = match &opts.opcode {
            Some(o) => normalize_opcode(o.clone()),
            None => {
                return Error::new_spanned(ident, "Missing #[opcode(...)] for outgoing packet")
                    .to_compile_error()
                    .into();
            }
        };

        let name_lit = syn::LitStr::new(&name.to_string(), name.span());

        quote! {
            impl #generics #ident #where_clause {
                pub const PACKET_NAME: &'static str = #name_lit;

                pub fn pack_with(
                    &self,
                    name: &str,
                    opcode: crate::client::packet::PacketOpcode,
                ) -> ::anyhow::Result<
                    crate::client::packet::Packet
                >
                where
                    Self: ::binrw::BinWrite
                        + ::serde::Serialize
                        + crate::client::packet::ExtractMetadata,
                    for<'a>
                        <Self as ::binrw::BinWrite>
                            ::Args<'a>: Default,
                {
                    let mut buffer =
                        ::std::io::Cursor::new(Vec::new());

                    ::binrw::BinWrite::write_args(
                        self,
                        &mut buffer,
                        <<Self as ::binrw::BinWrite>
                            ::Args<'_>>::default(),
                    )?;

                    let json =
                        crate::client::packet
                            ::serialize_packet_json(self)?;

                    let mut packet =
                        crate::client::packet::Packet::default();

                    packet.set_opcode(opcode);
                    packet.set_type(
                        crate::client::packet::PacketType::Outgoing
                    );
                    packet.set_offset_info(
                        crate::client::packet::ExtractMetadata
                            ::extract_metadata(self)
                    );
                    packet.set_packet_name(name.to_string());
                    packet.set_body(buffer.into_inner());
                    packet.set_json(json);

                    Ok(packet)
                }

                pub fn pack(
                    &self
                ) -> ::anyhow::Result<
                    crate::client::packet::Packet
                > {
                    self.pack_with(
                        Self::PACKET_NAME,
                        #opcode
                    )
                }
            }
        }
    } else {
        quote! {}
    };

    quote! {
        #unpack_impl
        #pack_impl
    }
    .into()
}

struct PacketOptions {
    has_br: bool,
    has_bw: bool,
    name: Option<Ident>,
    opcode: Option<Expr>,
}

impl PacketOptions {
    fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        let mut has_br = false;
        let mut has_bw = false;
        let mut name = None;
        let mut opcode = None;

        for attr in attrs {
            if attr.path().is_ident("br") {
                has_br = true;
            }

            if attr.path().is_ident("bw") {
                has_bw = true;
            }

            if attr.path().is_ident("name") {
                name = Some(attr.parse_args()?);
            }

            if attr.path().is_ident("opcode") {
                opcode = Some(attr.parse_args()?);
            }
        }

        if has_br && has_bw {
            return Err(Error::new(
                proc_macro2::Span::call_site(),
                "Packet cannot be both #[br(...)] and #[bw(...)]",
            ));
        }

        if !has_br && !has_bw {
            return Err(Error::new(
                proc_macro2::Span::call_site(),
                "Packet requires either #[br(...)] or #[bw(...)]",
            ));
        }

        Ok(Self {
            has_br,
            has_bw,
            name,
            opcode,
        })
    }
}

fn normalize_opcode(expr: Expr) -> Expr {
    if let Expr::Call(call) = &expr
        && let Expr::Path(path) = &*call.func
        && let Some(id) = path.path.get_ident()
    {
        let name = id.to_string();
        let ok = matches!(name.as_str(), "U8" | "U16" | "U32" | "U64" | "Text" | "Raw");

        if ok && call.args.len() == 1 {
            let arg = call.args.first().cloned().unwrap();
            let v = Ident::new(&name, id.span());
            return syn::parse_quote! {
                crate::client::packet
                    ::PacketOpcode::#v(#arg)
            };
        }
    }

    expr
}
