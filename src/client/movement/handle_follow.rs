use std::cell::RefCell;
use std::io::{Cursor};
use byteorder::{LittleEndian, ReadBytesExt};

use crate::client::opcodes::Opcode;
use crate::network::session::types::{StateFlags};
use crate::types::{HandlerInput, HandlerOutput, HandlerResult};
use crate::utils::{read_packed_guid};

pub fn handler(input: &mut HandlerInput) -> HandlerResult {
    let reader = RefCell::new(Cursor::new(input.data.as_ref().unwrap()[2..].to_vec()));
    let opcode = reader.borrow_mut().read_u16::<LittleEndian>()?;

    let (target_guid, position) = read_packed_guid(RefCell::clone(&reader));
    reader.borrow_mut().set_position(position);

    if let Some(follow_guid) = input.session.follow_target {
        if follow_guid != target_guid {
            return Ok(HandlerOutput::Void);
        }

        input.session.state_flags.set(
            StateFlags::IS_MOVEMENT_STARTED,
            opcode == Opcode::MSG_MOVE_STOP
                || opcode == Opcode::MSG_MOVE_STOP_TURN
                || opcode == Opcode::MSG_MOVE_STOP_STRAFE
        );
    }

    Ok(HandlerOutput::Void)

    // let (movement_info, _) = MovementParser::parse(RefCell::clone(&reader));

    // let me = input.session.me.as_mut().unwrap();
    // let (movement_info, _) = MovementParser::parse(RefCell::clone(&reader));
    // let position = movement_info.position;
    //
    // let mut body: Vec<u8> = Vec::new();
    // body.write_all(&pack_guid(me.guid))?;
    // body.write_u32::<LittleEndian>(movement_info.movement_flags)?;
    // body.write_u16::<LittleEndian>(movement_info.movement_flags2 as u16)?;
    // body.write_u32::<LittleEndian>(movement_info.time)?;
    // body.write_f32::<LittleEndian>(position.x)?;
    // body.write_f32::<LittleEndian>(position.y)?;
    // body.write_f32::<LittleEndian>(position.z)?;
    // body.write_f32::<LittleEndian>(position.orientation)?;
    // body.write_u32::<LittleEndian>(movement_info.fall_time)?;

    // println!(
    //     "MOVING: {:?}:{:?}:{:?} ({:?}) --- [{:?} - {:?}] + {:?}",
    //     movement_info.x,
    //     movement_info.y,
    //     movement_info.z,
    //     movement_info.orientation,
    //     movement_info.movement_flags,
    //     movement_info.movement_flags2,
    //     movement_info.fall_time,
    // );

    // if movement_info.movement_flags & MovementFlags::MOVEMENTFLAG_JUMPING != 0 {
    //     body.write_f32::<LittleEndian>(movement_info.jump_vertical_speed)?;
    //     body.write_f32::<LittleEndian>(movement_info.jump_sin_angle)?;
    //     body.write_f32::<LittleEndian>(movement_info.jump_cos_angle)?;
    //     body.write_f32::<LittleEndian>(movement_info.jump_horizontal_speed)?;
    // }
    //
    // Ok(HandlerOutput::Data(OutcomePacket::new(opcode as u32, Some(body))))
}