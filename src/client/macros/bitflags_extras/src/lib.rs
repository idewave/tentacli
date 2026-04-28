use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(BitflagExtras, attributes(bitflags_repr))]
pub fn derive_bitflag_extras(input: TokenStream) -> TokenStream {
    let input_ast = parse_macro_input!(input as DeriveInput);
    let flag_name = input_ast.ident;

    // Parse #[bitflags_repr(u32)] as syn::Type
    let mut repr_type: Option<syn::Type> = None;

    for attribute in input_ast.attrs {
        if attribute.path().is_ident("bitflags_repr") {
            let parsed_type = attribute
                .parse_args::<syn::Type>()
                .expect("Invalid #[bitflags_repr(...)] syntax. Example: #[bitflags_repr(u32)]");
            repr_type = Some(parsed_type);
            break;
        }
    }

    let repr_type = repr_type.expect("Missing #[bitflags_repr(u8|u16|u32|u64)]");

    let expanded = quote! {
        impl binrw::BinRead for #flag_name {
            type Args<'a> = ();

            fn read_options<R: std::io::Read + std::io::Seek>(
                reader: &mut R,
                endian: binrw::Endian,
                _: Self::Args<'_>,
            ) -> binrw::BinResult<Self> {
                let bits = <#repr_type as binrw::BinRead>::read_options(reader, endian, ())?;
                Ok(Self::from_bits_truncate(bits))
            }
        }

        impl binrw::BinWrite for #flag_name {
            type Args<'a> = ();

            fn write_options<W: std::io::Write>(
                &self,
                writer: &mut W,
                endian: binrw::Endian,
                _: Self::Args<'_>,
            ) -> binrw::BinResult<()> {
                let bits: #repr_type = self.bits();
                match endian {
                    binrw::Endian::Little => writer.write_all(&bits.to_le_bytes())?,
                    binrw::Endian::Big => writer.write_all(&bits.to_be_bytes())?,
                }
                Ok(())
            }
        }

        impl serde::Serialize for #flag_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut string_flags = String::new();
                let mut is_first = true;

                for flag in self.iter() {
                    if !is_first {
                        string_flags.push('|');
                    }
                    is_first = false;

                    string_flags.push_str(
                        &format!("{:?}", flag)
                            .replace(stringify!(#flag_name), "")
                            .replace('(', "")
                            .replace(')', ""),
                    );
                }

                serializer.serialize_str(&string_flags)
            }
        }

        impl crate::client::CalculateMetadata for #flag_name {
            fn calculate<'a>(
                &self,
                context: &'a mut crate::client::MetadataContext,
            ) -> &'a mut crate::client::MetadataContext {
                self.bits().calculate(context)
            }
        }
    };

    TokenStream::from(expanded)
}
