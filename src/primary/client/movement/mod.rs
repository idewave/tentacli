mod detect_motion;
pub mod types;

use crate::primary::client::opcodes::Opcode;
use crate::primary::traits::processor::Processor;
use crate::primary::types::{HandlerInput, ProcessorResult};

pub struct MovementProcessor;

impl Processor for MovementProcessor {
    fn get_handlers(input: &mut HandlerInput) -> ProcessorResult {
        let handlers: ProcessorResult = vec![
            Box::new(detect_motion::Handler),
        ];

        let handlers: ProcessorResult = match input.opcode {
            Opcode::MSG_MOVE_START_FORWARD |
            Opcode::MSG_MOVE_START_BACKWARD |
            Opcode::MSG_MOVE_JUMP |
            Opcode::MSG_MOVE_HEARTBEAT |
            Opcode::MSG_MOVE_START_TURN_LEFT |
            Opcode::MSG_MOVE_START_TURN_RIGHT |
            Opcode::MSG_MOVE_STOP |
            Opcode::MSG_MOVE_STOP_STRAFE |
            Opcode::MSG_MOVE_STOP_TURN |
            Opcode::MSG_MOVE_START_PITCH_UP |
            Opcode::MSG_MOVE_START_PITCH_DOWN |
            Opcode::MSG_MOVE_STOP_PITCH |
            Opcode::MSG_MOVE_FALL_LAND |
            Opcode::MSG_MOVE_SET_PITCH |
            Opcode::MSG_MOVE_START_SWIM |
            Opcode::MSG_MOVE_STOP_SWIM |
            Opcode::MSG_MOVE_SET_FACING => {
                handlers
            },
            _ => {
                vec![]
            },
        };

        handlers
    }
}

pub mod packet {
    use crate::primary::client::Opcode;
    use crate::primary::types::PackedGuid;

    #[non_exhaustive]
    pub struct MovementOpcodes;

    #[allow(dead_code)]
    impl MovementOpcodes {
        pub const MSG_MOVE_START_FORWARD: u16 = Opcode::MSG_MOVE_START_FORWARD;
        pub const MSG_MOVE_START_BACKWARD: u16 = Opcode::MSG_MOVE_START_BACKWARD;
        pub const MSG_MOVE_STOP: u16 = Opcode::MSG_MOVE_STOP;
        pub const MSG_MOVE_START_STRAFE_LEFT: u16 = Opcode::MSG_MOVE_START_STRAFE_LEFT;
        pub const MSG_MOVE_START_STRAFE_RIGHT: u16 = Opcode::MSG_MOVE_START_STRAFE_RIGHT;
        pub const MSG_MOVE_STOP_STRAFE: u16 = Opcode::MSG_MOVE_STOP_STRAFE;
        pub const MSG_MOVE_JUMP: u16 = Opcode::MSG_MOVE_JUMP;
        pub const MSG_MOVE_START_TURN_LEFT: u16 = Opcode::MSG_MOVE_START_TURN_LEFT;
        pub const MSG_MOVE_START_TURN_RIGHT: u16 = Opcode::MSG_MOVE_START_TURN_RIGHT;
        pub const MSG_MOVE_STOP_TURN: u16 = Opcode::MSG_MOVE_STOP_TURN;
        pub const MSG_MOVE_START_PITCH_UP: u16 = Opcode::MSG_MOVE_START_PITCH_UP;
        pub const MSG_MOVE_START_PITCH_DOWN: u16 = Opcode::MSG_MOVE_START_PITCH_DOWN;
        pub const MSG_MOVE_STOP_PITCH: u16 = Opcode::MSG_MOVE_STOP_PITCH;
        pub const MSG_MOVE_FALL_LAND: u16 = Opcode::MSG_MOVE_FALL_LAND;
        pub const MSG_MOVE_START_SWIM: u16 = Opcode::MSG_MOVE_START_SWIM;
        pub const MSG_MOVE_STOP_SWIM: u16 = Opcode::MSG_MOVE_STOP_SWIM;
        pub const MSG_MOVE_SET_FACING: u16 = Opcode::MSG_MOVE_SET_FACING;
        pub const MSG_MOVE_SET_PITCH: u16 = Opcode::MSG_MOVE_SET_PITCH;
        pub const MSG_MOVE_WORLDPORT_ACK: u16 = Opcode::MSG_MOVE_WORLDPORT_ACK;
        pub const MSG_MOVE_HEARTBEAT: u16 = Opcode::MSG_MOVE_HEARTBEAT;
    }

    #[derive(WorldPacket, Serialize, Deserialize, Debug)]
    #[options(no_opcode)]
    pub struct MovementOutcome {
        pub guid: PackedGuid,
        pub movement_flags: u32,
        pub movement_flags2: u16,
        pub time: u32,
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub direction: f32,
        pub unknown: u32,
    }
}