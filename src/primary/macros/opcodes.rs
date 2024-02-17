#[macro_export]
macro_rules! opcodes {
    (
        pub struct $struct_name:ident {
            $(
                $(#[$attrs:meta])*
                pub const $const_name:ident : $const_type:ty = $const_value:expr;
            )*
        }
    ) => {
        pub struct $struct_name;

        impl $struct_name {
            $(
                $(#[$attrs])*
                #[allow(dead_code)]
                pub const $const_name: $const_type = $const_value;
            )*

            pub fn get_opcode_name(index: u32) -> Option<String> {
                match index {
                    $(
                        $const_value if $const_value == $const_value as u32 => Some(stringify!($const_name).to_string()),
                    )*
                    _ => None,
                }
            }
        }
    };
}