use std::cell::RefCell;
use std::io::{Cursor};

use crate::ipc::session::types::ActionFlags;
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::utils::read_packed_guid;

pub fn handler(input: &mut HandlerInput) -> HandlerResult {
    let reader = RefCell::new(Cursor::new(input.data.as_ref().unwrap()[4..].to_vec()));
    let (_cast_item_guid, position) = read_packed_guid(RefCell::clone(&reader));
    reader.borrow_mut().set_position(position);

    let (caster_guid, position) = read_packed_guid(RefCell::clone(&reader));
    reader.borrow_mut().set_position(position);

    input.session.action_flags.set(
        ActionFlags::IS_CASTING,
        input.session.me.as_ref().unwrap().guid == caster_guid
    );

    Ok(HandlerOutput::Void)
}