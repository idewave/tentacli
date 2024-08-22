#[macro_use]
extern crate thiserror;

mod binary_converter;
mod feature;
mod packet_handler;
mod processor;
mod stream_reader;
pub mod types;

pub use binary_converter::BinaryConverter;
pub use types::errors::{
    FieldError, ConfigError, CharacterListError, RealmListError, FeatureError, MutexError
};
pub use feature::Feature;
pub use packet_handler::PacketHandler;
pub use processor::Processor;
pub use stream_reader::StreamReader;

#[macro_export]
macro_rules! impl_serialize_for_flags {
    ($flag_struct:ident) => {
        impl serde::Serialize for $flag_struct {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let mut str_flags = String::new();
                let mut first = true;
                for flag in self.iter() {
                    if first {
                        first = false;
                    } else {
                        str_flags += "|";
                    }
                    str_flags += &format!("{:?}", flag)
                        .replace(stringify!($flag_struct), "")
                        .replace('(', "")
                        .replace(')', "");
                }
                serializer.serialize_str(&str_flags)
            }
        }
    };
}