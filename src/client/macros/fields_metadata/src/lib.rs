use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Index, parse_macro_input};

#[proc_macro_derive(FieldsMetadata)]
pub fn fields_metadata(input: TokenStream) -> TokenStream {
    let extract_metadata = quote!(crate::client::packet::ExtractMetadata);
    let metadata_context = quote!(crate::client::packet::MetadataContext);
    let calculate_metadata = quote!(crate::client::packet::CalculateMetadata);

    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let handlers = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(named) => {
                let per_field = named.named.iter().filter_map(|f| {
                        if has_serde_skip(&f.attrs) {
                            return None;
                        }

                        let field_name = &f.ident;
                        let field_ty   = &f.ty;

                        Some(quote! {
                            {
                                let prev_key = context.current_key.clone();
                                context.current_key = if prev_key.is_empty() {
                                    stringify!(#field_name).to_string()
                                } else {
                                    format!("{}/{}", prev_key, stringify!(#field_name))
                                };

                                <#field_ty as #calculate_metadata>::calculate(&self.#field_name, context);
                                context.current_key = prev_key;
                            }
                        })
                    });

                quote! { #(#per_field)* }
            }

            Fields::Unnamed(unnamed) => {
                let len = unnamed.unnamed.len();
                if len == 0 {
                    quote! {}
                } else if len == 1 {
                    let ty0 = &unnamed.unnamed[0].ty;
                    quote! {
                        <#ty0 as #calculate_metadata>::calculate(&self.0, context);
                    }
                } else {
                    let per_field = unnamed.unnamed.iter().enumerate().filter_map(|(i, f)| {
                        if has_serde_skip(&f.attrs) {
                            return None;
                        }

                        let idx = Index::from(i);
                        let f_ty = &f.ty;
                        let i_str = i.to_string();

                        Some(quote! {
                            {
                                let prev_key = context.current_key.clone();
                                context.current_key = if prev_key.is_empty() {
                                    #i_str.to_string()
                                } else {
                                    format!("{}/{}", prev_key, #i_str)
                                };

                                <#f_ty as #calculate_metadata>::calculate(&self.#idx, context);
                                context.current_key = prev_key;
                            }
                        })
                    });

                    quote! { #(#per_field)* }
                }
            }

            Fields::Unit => {
                quote! {}
            }
        },
        _ => {
            return syn::Error::new_spanned(&input.ident, "Only structs are supported")
                .to_compile_error()
                .into();
        }
    };

    let expanded = quote! {
        impl #calculate_metadata for #struct_name {
            fn calculate<'a>(&self, context: &'a mut #metadata_context) -> &'a mut #metadata_context {
                #handlers
                context
            }
        }

        impl #extract_metadata for #struct_name {}
    };

    expanded.into()
}

fn has_serde_skip(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if !attr.path().is_ident("serde") {
            return false;
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("skip") || meta.path.is_ident("skip_serializing") {
                Ok(())
            } else {
                Err(meta.error("not skip"))
            }
        })
        .is_ok()
    })
}
