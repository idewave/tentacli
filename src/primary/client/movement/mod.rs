use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

mod detect_motion;
pub mod types;

use crate::primary::client::opcodes::Opcode;
use crate::primary::traits::processor::Processor;
use crate::primary::types::{HandlerInput, ProcessorResult};

pub struct MovementProcessor;

impl Processor for MovementProcessor {
    fn process_input(input: &mut HandlerInput) -> ProcessorResult {
        let mut reader = Cursor::new(input.data.as_ref().unwrap()[2..].to_vec());
        let opcode = reader.read_u16::<LittleEndian>().unwrap();

        let handlers: ProcessorResult = vec![
            Box::new(detect_motion::Handler),
        ];

        let handlers: ProcessorResult = match opcode {
            Opcode::MSG_MOVE_START_FORWARD => {
                input.opcode = Some(opcode);
                handlers
            },
            Opcode::MSG_MOVE_START_BACKWARD => {
                input.opcode = Some(opcode);
                handlers
            },
            Opcode::MSG_MOVE_JUMP => {
                input.opcode = Some(opcode);
                handlers
            },
            Opcode::MSG_MOVE_HEARTBEAT => {
                input.opcode = Some(opcode);
                handlers
            },
            Opcode::MSG_MOVE_START_TURN_LEFT => {
                input.opcode = Some(opcode);
                handlers
            },
            Opcode::MSG_MOVE_START_TURN_RIGHT => {
                input.opcode = Some(opcode);
                handlers
            },
            Opcode::MSG_MOVE_STOP => {
                input.opcode = Some(opcode);
                handlers
            },
            Opcode::MSG_MOVE_STOP_STRAFE => {
                input.opcode = Some(opcode);
                handlers
            },
            Opcode::MSG_MOVE_STOP_TURN => {
                input.opcode = Some(opcode);
                handlers
            },
            Opcode::MSG_MOVE_START_PITCH_UP => {
                input.opcode = Some(opcode);
                handlers
            },
            Opcode::MSG_MOVE_START_PITCH_DOWN => {
                input.opcode = Some(opcode);
                handlers
            },
            Opcode::MSG_MOVE_STOP_PITCH => {
                input.opcode = Some(opcode);
                handlers
            },
            Opcode::MSG_MOVE_FALL_LAND => {
                input.opcode = Some(opcode);
                handlers
            },
            Opcode::MSG_MOVE_SET_PITCH => {
                input.opcode = Some(opcode);
                handlers
            },
            Opcode::MSG_MOVE_START_SWIM => {
                input.opcode = Some(opcode);
                handlers
            },
            Opcode::MSG_MOVE_STOP_SWIM => {
                input.opcode = Some(opcode);
                handlers
            },
            Opcode::MSG_MOVE_SET_FACING => {
                input.opcode = Some(opcode);
                handlers
            },
            _ => {
                vec![]
            },
        };

        handlers
    }
}