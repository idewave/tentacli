use std::time::{SystemTime};
use std::f32::consts::PI;
use anyhow::{Result as AnyResult};

use crate::client::{MovementFlags, MovementFlagsExtra, UnitMoveType};
use crate::client::opcodes::Opcode;
use crate::ipc::session::types::ActionFlags;
use crate::parsers::position_parser::types::Position;
use crate::types::{AIManagerInput, PackedGuid, PacketOutcome};

const MINIMAL_DISTANCE: f32 = 5.0;

#[derive(WorldPacket, Serialize, Deserialize, Debug)]
#[options(no_opcode)]
struct Outcome {
    guid: PackedGuid,
    movement_flags: u32,
    movement_flags2: u16,
    time: u32,
    x: f32,
    y: f32,
    z: f32,
    direction: f32,
    unknown: u32,
}

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

                                let packet_outcome = {
                                    let guard = input.session.lock().unwrap();
                                    let position = guard.me.as_ref().unwrap().position.unwrap();

                                    Self::build_movement_packet(
                                        u32::from(Opcode::MSG_MOVE_START_FORWARD),
                                        my_guid.unwrap(),
                                        position,
                                        MovementFlags::FORWARD,
                                        // MovementFlags::NONE,
                                    ).unwrap()
                                };

                                // input.output_queue_sender.lock().unwrap().push_back(packet);
                                input.output_sender.send(packet_outcome).await.unwrap();
                            } else {
                                let packet_outcome = {
                                    let guard = input.session.lock().unwrap();
                                    let position = guard.me.as_ref().unwrap().position.unwrap();

                                    Self::build_movement_packet(
                                        u32::from(Opcode::MSG_MOVE_HEARTBEAT),
                                        my_guid.unwrap(),
                                        position,
                                        MovementFlags::FORWARD,
                                        // MovementFlags::NONE,
                                    ).unwrap()
                                };

                                // input.output_queue_sender.lock().unwrap().push_back(packet);
                                input.output_sender.send(packet_outcome).await.unwrap();
                            }
                        }
                    } else if self.is_moving_started {
                        let packet_outcome = {
                            let guard = input.session.lock().unwrap();
                            let position = guard.me.as_ref().unwrap().position.unwrap();

                            Self::build_movement_packet(
                                u32::from(Opcode::MSG_MOVE_STOP),
                                my_guid.unwrap(),
                                position,
                                MovementFlags::NONE,
                            ).unwrap()
                        };

                        input.output_sender.send(packet_outcome).await.unwrap();
                        self.is_moving_started = false;
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
    ) -> AnyResult<PacketOutcome> {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        Outcome {
            guid: PackedGuid(guid),
            movement_flags: movement_flags.bits(),
            movement_flags2: MovementFlagsExtra::NONE.bits(),
            // TODO: need to investigate how better get u32 value from timestamp
            time: now.as_millis() as u32,
            x: new_position.x,
            y: new_position.y,
            z: new_position.z,
            direction: new_position.orientation,
            unknown: 0
        }.unpack_with_opcode(opcode)
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