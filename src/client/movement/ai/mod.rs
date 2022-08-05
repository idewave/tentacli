use std::time::{SystemTime};
use std::f32::consts::PI;
use std::io::Write;
use byteorder::{LittleEndian, WriteBytesExt};

mod handle_follow;

use crate::client::movement::parsers::types::Position;
use crate::client::{MovementFlags, MovementFlagsExtra, UnitMoveType};
use crate::client::opcodes::Opcode;
use crate::network::packet::OutcomePacket;
use crate::ipc::session::types::ActionFlags;
use crate::types::AIManagerInput;
use crate::utils::pack_guid;

const MINIMAL_DISTANCE: f32 = 5.0;

pub struct AI {
    is_moving_started: bool,
}

impl AI {
    pub fn new() -> Self {
        Self {
            is_moving_started: false,
        }
    }

    pub async fn manage(&mut self, input: AIManagerInput) {
        let mut output_queue = input.output_queue.lock().unwrap();
        let mut session = input.session.lock().unwrap();
        let data_storage = input.data_storage.lock().unwrap();

        if session.me.is_some() {
            if session.action_flags.contains(ActionFlags::IS_FOLLOWING) {
                let target_guid = session.follow_target.unwrap();
                let me = session.me.as_mut().unwrap();

                if let Some(target) = data_storage.players_map.get(&target_guid) {
                    let mut source_position = me.position.unwrap();

                    if let Some(target_position) = target.position {
                        let distance = Self::calculate_distance(
                            source_position,
                            target_position,
                        );

                        if distance > MINIMAL_DISTANCE {
                            if let Some(speed) = me.movement_speed.get(&UnitMoveType::MOVE_RUN) {
                                let velocity = Self::calculate_velocity(
                                    source_position,
                                    target_position,
                                    *speed
                                );

                                source_position.x += velocity.x;
                                source_position.y += velocity.y;
                                source_position.z += velocity.z;

                                if !self.is_moving_started {
                                    self.is_moving_started = true;

                                    source_position.orientation = Self::calculate_orientation(
                                        source_position,
                                        target_position,
                                    );

                                    let (_, header, body) = Self::build_movement_packet(
                                        u32::from(Opcode::MSG_MOVE_START_FORWARD),
                                        me.guid,
                                        source_position,
                                        MovementFlags::FORWARD,
                                        // MovementFlags::NONE,
                                    );
                                    let packet = [header, body].concat();

                                    output_queue.push_back(packet);
                                } else {
                                    let (_, header, body) = Self::build_movement_packet(
                                        u32::from(Opcode::MSG_MOVE_HEARTBEAT),
                                        me.guid,
                                        source_position,
                                        MovementFlags::FORWARD,
                                        // MovementFlags::NONE,
                                    );
                                    let packet = [header, body].concat();

                                    output_queue.push_back(packet);
                                }
                            }
                        } else if self.is_moving_started {
                            let (_, header, body) = Self::build_movement_packet(
                                u32::from(Opcode::MSG_MOVE_STOP),
                                me.guid,
                                source_position,
                                MovementFlags::NONE,
                            );
                            let packet = [header, body].concat();

                            output_queue.push_back(packet);
                            self.is_moving_started = false;
                        }

                        me.position = Some(source_position);
                    }
                }
            }
        }
    }

    fn build_movement_packet(
        opcode: u32,
        guid: u64,
        new_position: Position,
        movement_flags: MovementFlags
    ) -> (u32, Vec<u8>, Vec<u8>) {
        let mut body: Vec<u8> = Vec::new();

        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

        body.write_all(&pack_guid(guid)).unwrap();
        body.write_u32::<LittleEndian>(movement_flags.bits()).unwrap();
        body.write_u16::<LittleEndian>(MovementFlagsExtra::NONE.bits()).unwrap();
        // TODO: need to investigate how better get u32 value from timestamp
        body.write_u32::<LittleEndian>(now.as_millis() as u32).unwrap();
        body.write_f32::<LittleEndian>(new_position.x).unwrap();
        body.write_f32::<LittleEndian>(new_position.y).unwrap();
        body.write_f32::<LittleEndian>(new_position.z).unwrap();
        body.write_f32::<LittleEndian>(new_position.orientation).unwrap();
        body.write_u32::<LittleEndian>(0).unwrap();

        OutcomePacket::from(opcode, Some(body))
    }

    fn calculate_velocity(from: Position, to: Position, speed: f32) -> Position {
        let distance = Self::calculate_distance(from, to);
        // TODO: need to investigate what is correct const value here
        let total_time = (distance / speed) * 60.5;

        Position::new(
            (to.x - from.x) / total_time,
            (to.y - from.y) / total_time,
            (to.z - from.z) / total_time,
            0.0,
        )
    }

    fn calculate_distance(from: Position, to: Position) -> f32 {
        let dx = to.x - from.x;
        let dy = to.y - from.y;
        let dz = to.z - from.z;

        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    fn calculate_orientation(from: Position, to: Position) -> f32 {
        let dx = to.x - from.x;
        let dy = to.y - from.y;
        let angle = dy.atan2(dx);

        if angle >= 0.0 { angle } else { 2.0 * PI + angle }
    }
}