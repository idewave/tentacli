#[macro_export]
macro_rules! with_opcode {
    (
        $(@login_opcode($login_opcode:expr))?
        $(@world_opcode($world_opcode:expr))?

        $(#[$outer:meta])*
        $vis:vis struct $PacketStruct:ident {
            $(
                $(#[$field_attr: meta])? $field_vis:vis $field_name:ident: $field_type:ty
            ),*$(,)?
        }

        $($PacketStructImpl: item)*
    ) => {
        $(#[$outer])*
        $vis struct $PacketStruct {
            $($(#[$field_attr])? $field_vis $field_name: $field_type),*
        }

         $($PacketStructImpl)*

        impl $PacketStruct {
            $(
                fn opcode() -> u8 {
                    $login_opcode as u8
                }
            )?

            $(
                fn opcode() -> u32 {
                    $world_opcode as u32
                }
            )?
        }
    };
}