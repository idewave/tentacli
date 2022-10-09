use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

pub mod ai;

mod detect_motion;
mod handle_follow;
pub mod parsers;
pub mod types;

use crate::client::opcodes::Opcode;
use crate::types::traits::{Processor};
use crate::types::{HandlerInput, ProcessorResult};

pub struct MovementProcessor;

impl Processor for MovementProcessor {
    fn process_input(input: &mut HandlerInput) -> ProcessorResult {
        let mut reader = Cursor::new(input.data.as_ref().unwrap()[2..].to_vec());
        let opcode = reader.read_u16::<LittleEndian>().unwrap();

        let handlers: ProcessorResult = vec![
            Box::new(detect_motion::Handler),
            Box::new(handle_follow::Handler),
        ];

        let mut message = String::new();

        let handlers: ProcessorResult = match opcode {
            Opcode::MSG_MOVE_START_FORWARD => {
                message = String::from("MSG_MOVE_START_FORWARD");
                handlers
            },
            Opcode::MSG_MOVE_START_BACKWARD => {
                message = String::from("MSG_MOVE_START_BACKWARD");
                handlers
            },
            Opcode::MSG_MOVE_JUMP => {
                message = String::from("MSG_MOVE_JUMP");
                handlers
            },
            Opcode::MSG_MOVE_HEARTBEAT => {
                message = String::from("MSG_MOVE_HEARTBEAT");
                handlers
            },
            Opcode::MSG_MOVE_START_TURN_LEFT => {
                message = String::from("MSG_MOVE_START_TURN_LEFT");
                handlers
            },
            Opcode::MSG_MOVE_START_TURN_RIGHT => {
                message = String::from("MSG_MOVE_START_TURN_RIGHT");
                handlers
            },
            Opcode::MSG_MOVE_STOP => {
                message = String::from("MSG_MOVE_STOP");
                handlers
            },
            Opcode::MSG_MOVE_STOP_STRAFE => {
                message = String::from("MSG_MOVE_STOP_STRAFE");
                handlers
            },
            Opcode::MSG_MOVE_STOP_TURN => {
                message = String::from("MSG_MOVE_STOP_TURN");
                handlers
            },
            Opcode::MSG_MOVE_START_PITCH_UP => {
                message = String::from("MSG_MOVE_START_PITCH_UP");
                handlers
            },
            Opcode::MSG_MOVE_START_PITCH_DOWN => {
                message = String::from("MSG_MOVE_START_PITCH_DOWN");
                handlers
            },
            Opcode::MSG_MOVE_STOP_PITCH => {
                message = String::from("MSG_MOVE_STOP_PITCH");
                handlers
            },
            Opcode::MSG_MOVE_FALL_LAND => {
                message = String::from("MSG_MOVE_FALL_LAND");
                handlers
            },
            Opcode::MSG_MOVE_SET_PITCH => {
                message = String::from("MSG_MOVE_SET_PITCH");
                handlers
            },
            Opcode::MSG_MOVE_START_SWIM => {
                message = String::from("MSG_MOVE_START_SWIM");
                handlers
            },
            Opcode::MSG_MOVE_STOP_SWIM => {
                message = String::from("MSG_MOVE_STOP_SWIM");
                handlers
            },
            Opcode::MSG_MOVE_SET_FACING => {
                message = String::from("MSG_MOVE_SET_FACING");
                handlers
            },
            _ => {
                vec![]
            },
        };

        input.message_income.send_server_message(message);

        handlers
    }
}