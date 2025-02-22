use tentacli_traits::Processor;
use tentacli_traits::types::opcodes::Opcode;
use tentacli_traits::types::ProcessorResult;

pub use item_name_query::ItemNameQuery;

mod update_object;
mod destroy_object;
mod item_name_query;

pub struct ObjectProcessor;

impl Processor for ObjectProcessor {
    fn get_handlers(opcode: u16) -> ProcessorResult {
        let handlers: ProcessorResult = match opcode {
            Opcode::SMSG_COMPRESSED_UPDATE_OBJECT |
            Opcode::SMSG_UPDATE_OBJECT => {
                vec![Box::new(update_object::Handler)]
            }
            Opcode::SMSG_DESTROY_OBJECT => {
                vec![Box::new(destroy_object::Handler)]
            }
            Opcode::SMSG_ITEM_NAME_QUERY_RESPONSE => {
                vec![Box::new(item_name_query::Handler)]
            }
            _ => vec![],
        };

        handlers
    }
}
