use std::cell::RefCell;
use std::io::{Cursor};
use async_trait::async_trait;

use crate::ipc::session::types::ActionFlags;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::types::traits::PacketHandler;
use crate::utils::read_packed_guid;

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let reader = RefCell::new(Cursor::new(input.data.as_ref().unwrap()[4..].to_vec()));
        let (_cast_item_guid, position) = read_packed_guid(RefCell::clone(&reader));
        reader.borrow_mut().set_position(position);

        let (caster_guid, position) = read_packed_guid(RefCell::clone(&reader));
        reader.borrow_mut().set_position(position);

        let my_guid = input.session.lock().unwrap().me.as_ref().unwrap().guid;
        input.session.lock().unwrap().action_flags.set(ActionFlags::IS_CASTING, my_guid == caster_guid);

        Ok(HandlerOutput::Void)
    }
}