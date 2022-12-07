use async_trait::async_trait;

use crate::client::opcodes::Opcode;
use crate::ipc::session::types::{StateFlags};
use crate::types::{HandlerInput, HandlerOutput, HandlerResult, PackedGuid};
use crate::traits::packet_handler::PacketHandler;

#[derive(WorldPacket, Serialize, Deserialize, Debug)]
#[options(no_opcode)]
struct Income {
    skip: u16,
    opcode: u16,
    target_guid: PackedGuid,
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult {
        let Income { opcode, target_guid, .. } = Income::from_binary(input.data.as_ref().unwrap());

        let is_follow_me = {
            if let Some(follow_guid) = input.session.lock().unwrap().follow_target {
                follow_guid == target_guid
            } else {
                false
            }
        };

        if is_follow_me {
            input.session.lock().unwrap().state_flags.set(
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
}