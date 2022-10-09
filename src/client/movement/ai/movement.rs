use std::time::{SystemTime};
use std::f32::consts::PI;
use std::io::Write;
use byteorder::{LittleEndian, WriteBytesExt};

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
        let is_following = {
            let guard = input.session.lock().unwrap();
            guard.action_flags.contains(ActionFlags::IS_FOLLOWING)
        };

        if !is_following {
            return;
        }

        let players_map = {
            let guard = input.data_storage.lock().unwrap();
            guard.players_map.clone()
        };

        let my_guid = {
            let guard = input.session.lock().unwrap();
            if guard.me.is_some() { Some(guard.me.as_ref().unwrap().guid) } else { None }
        };

        let my_movement_speed = {
            let guard = input.session.lock().unwrap();
            guard.me.as_ref().unwrap().movement_speed.clone()
        };

        let target_guid = {
            let guard = input.session.lock().unwrap();
            guard.follow_target.unwrap()
        };

        if my_guid.is_some() {
            if let Some(target) = players_map.get(&target_guid) {
                let initial_position = {
                    let guard = input.session.lock().unwrap();
                    let position = guard.me.as_ref().unwrap().position.unwrap();
                    position
                };

                if let Some(target_position) = target.position {
                    let distance = Self::calculate_distance(
                        initial_position,
                        target_position,
                    );

                    if distance > MINIMAL_DISTANCE {
                        if let Some(speed) = my_movement_speed.get(&UnitMoveType::MOVE_RUN) {
                            let velocity = Self::calculate_velocity(
                                initial_position,
                                target_position,
                                *speed
                            );

                            {
                                let guard = input.session.lock().unwrap();
                                let mut position = guard.me.as_ref().unwrap().position.unwrap();

                                position.x += velocity.x;
                                position.y += velocity.y;
                                position.z += velocity.z;
                            }

                            if !self.is_moving_started {
                                self.is_moving_started = true;

                                {
                                    let guard = input.session.lock().unwrap();
                                    let mut position = guard.me.as_ref().unwrap().position.unwrap();

                                    position.orientation = Self::calculate_orientation(
                                        position,
                                        target_position,
                                    );
                                }

                                let packet = {
                                    let guard = input.session.lock().unwrap();
                                    let position = guard.me.as_ref().unwrap().position.unwrap();

                                    let (_, header, body) = Self::build_movement_packet(
                                        u32::from(Opcode::MSG_MOVE_START_FORWARD),
                                        my_guid.unwrap(),
                                        position,
                                        MovementFlags::FORWARD,
                                        // MovementFlags::NONE,
                                    );
                                    [header, body].concat()
                                };

                                // input.output_queue_sender.lock().unwrap().push_back(packet);
                                input.output_sender.send(packet).await.unwrap();
                            } else {
                                let packet = {
                                    let guard = input.session.lock().unwrap();
                                    let position = guard.me.as_ref().unwrap().position.unwrap();

                                    let (_, header, body) = Self::build_movement_packet(
                                        u32::from(Opcode::MSG_MOVE_HEARTBEAT),
                                        my_guid.unwrap(),
                                        position,
                                        MovementFlags::FORWARD,
                                        // MovementFlags::NONE,
                                    );
                                    [header, body].concat()
                                };

                                // input.output_queue_sender.lock().unwrap().push_back(packet);
                                input.output_sender.send(packet).await.unwrap();
                            }
                        }
                    } else if self.is_moving_started {
                        let packet = {
                            let guard = input.session.lock().unwrap();
                            let position = guard.me.as_ref().unwrap().position.unwrap();

                            let (_, header, body) = Self::build_movement_packet(
                                u32::from(Opcode::MSG_MOVE_STOP),
                                my_guid.unwrap(),
                                position,
                                MovementFlags::NONE,
                            );
                            [header, body].concat()
                        };

                        // input.output_queue_sender.lock().unwrap().push_back(packet);
                        input.output_sender.send(packet).await.unwrap();
                        self.is_moving_started = false;
                    }

                    // let guard = input.session.lock().unwrap();
                    // guard.me.unwrap().position = Some(position);
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