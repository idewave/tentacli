use std::collections::HashMap;

use async_trait::async_trait;
use tentacli_traits::PacketHandler;
use tentacli_traits::types::{HandlerInput, HandlerOutput, HandlerResult};
use tentacli_traits::types::opcodes::Opcode;
use tentacli_traits::types::shared::Object;
use tentacli_traits::types::update_fields::{FieldValue, ObjectField};

#[derive(WorldPacket, Serialize, Debug)]
pub struct ItemNameQuery {
    pub entry: i32,
    pub guid: u64,
}

#[derive(WorldPacket, Serialize, Debug)]
pub struct Incoming {
    entry: i32,
    name: String,
    inventory_type: u32,
}

pub struct Handler;

#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let mut response = Vec::new();

        let (Incoming { entry, name, .. }, json) = Incoming::from_binary(&input.data)?;

        response.push(HandlerOutput::ResponseMessage(
            Opcode::get_opcode_name(input.opcode as u32)
                .unwrap_or(format!("Unknown opcode: {}", input.opcode)),
            Some(json),
        ));

        let mut guard = input.data_storage.lock().await;
        Self::update_or_insert(&mut guard.items_map, entry, name);

        Ok(response)
    }
}

impl Handler {
    fn update_or_insert(map: &mut HashMap<u64, Object>, target_entry: i32, name: String) {
        let item = map.values_mut().find(|item| {
            match item.update_data.object_fields.get(&ObjectField::Entry) {
                Some(FieldValue::Integer(entry)) => *entry == target_entry,
                _ => false,
            }
        });

        if let Some(item) = item {
            item.name = name;
        }
    }
}