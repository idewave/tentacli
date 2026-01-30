mod null_terminated;

pub use null_terminated::NullTerminated;

#[macro_export]
macro_rules! enum_field {
    (
        $vis:vis enum $name:ident : $repr:ty {
            $(
                $variant:ident = $value:expr
            ),* $(,)?
        }
    ) => {
        #[repr($repr)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        $vis enum $name {
            $(
                $variant = $value,
            )*
            Unknown($repr),
        }

        impl ::binrw::BinRead for $name {
            type Args<'a> = ();

            fn read_options<R: ::std::io::Read + ::std::io::Seek>(
                reader: &mut R,
                endian: ::binrw::Endian,
                _args: Self::Args<'_>,
            ) -> ::binrw::BinResult<Self> {
                let raw: $repr =
                    <$repr as ::binrw::BinRead>::read_options(reader, endian, ())?;

                Ok(match raw {
                    $(
                        $value => $name::$variant,
                    )*
                    other => $name::Unknown(other),
                })
            }
        }

        impl $crate::client::packet::CalculateMetadata for $name {
            fn calculate<'a>(
                &self,
                ctx: &'a mut $crate::client::packet::MetadataContext,
            ) -> &'a mut $crate::client::packet::MetadataContext {
                let size = ::std::mem::size_of::<$repr>();
                ctx.metadata.insert(
                    ctx.current_key.clone(),
                    $crate::client::packet::MetadataValue {
                        size,
                        offset: ctx.offset,
                    },
                );
                ctx.offset += size;
                ctx
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                fn to_screaming_snake(input: &str) -> String {
                    let mut out = String::with_capacity(input.len() + 8);

                    for (i, ch) in input.chars().enumerate() {
                        if ch.is_uppercase() && i > 0 {
                            out.push('_');
                        }
                        for up in ch.to_uppercase() {
                            out.push(up);
                        }
                    }

                    out
                }

                let name = match self {
                    $(
                        $name::$variant => stringify!($variant),
                    )*
                    $name::Unknown(_) => "UNKNOWN",
                };

                let formatted = to_screaming_snake(name);
                serializer.serialize_str(&formatted)
            }
        }
    };
}
