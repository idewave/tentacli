use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

pub mod ai;

mod detect_motion;
mod handle_follow;
mod monster_move;
pub mod parsers;
pub mod types;

use crate::client::opcodes::Opcode;
use crate::traits::Processor;
use crate::types::{HandlerFunction, HandlerInput, ProcessorResult};

pub struct MovementProcessor;

impl Processor for MovementProcessor {
    fn process_input(input: HandlerInput) -> ProcessorResult {
        let mut reader = Cursor::new(input.data.as_ref().unwrap()[2..].to_vec());
        let opcode = reader.read_u16::<LittleEndian>().unwrap();

        let handlers: Vec<HandlerFunction> = vec![
            Box::new(detect_motion::handler),
            Box::new(handle_follow::handler),
        ];

        let handlers: Vec<HandlerFunction> = match opcode {
            Opcode::MSG_MOVE_START_FORWARD => {
                println!("RECEIVE MSG_MOVE_START_FORWARD");
                handlers
            },
            Opcode::MSG_MOVE_START_BACKWARD => {
                println!("RECEIVE MSG_MOVE_START_BACKWARD");
                handlers
            },
            Opcode::MSG_MOVE_JUMP => {
                println!("RECEIVE MSG_MOVE_JUMP");
                handlers
            },
            Opcode::MSG_MOVE_HEARTBEAT => {
                println!("RECEIVE MSG_MOVE_HEARTBEAT");
                handlers
            },
            Opcode::MSG_MOVE_START_TURN_LEFT => {
                println!("RECEIVE MSG_MOVE_START_TURN_LEFT");
                handlers
            },
            Opcode::MSG_MOVE_START_TURN_RIGHT => {
                println!("RECEIVE MSG_MOVE_START_TURN_RIGHT");
                handlers
            },
            Opcode::MSG_MOVE_STOP => {
                println!("RECEIVE MSG_MOVE_STOP");
                handlers
            },
            Opcode::MSG_MOVE_STOP_STRAFE => {
                println!("RECEIVE MSG_MOVE_STOP_STRAFE");
                handlers
            },
            Opcode::MSG_MOVE_STOP_TURN => {
                println!("RECEIVE MSG_MOVE_STOP_TURN");
                handlers
            },
            Opcode::MSG_MOVE_START_PITCH_UP => {
                println!("RECEIVE MSG_MOVE_START_PITCH_UP");
                handlers
            },
            Opcode::MSG_MOVE_START_PITCH_DOWN => {
                println!("RECEIVE MSG_MOVE_START_PITCH_DOWN");
                handlers
            },
            Opcode::MSG_MOVE_STOP_PITCH => {
                println!("RECEIVE MSG_MOVE_STOP_PITCH");
                handlers
            },
            Opcode::MSG_MOVE_FALL_LAND => {
                println!("RECEIVE MSG_MOVE_FALL_LAND");
                handlers
            },
            Opcode::MSG_MOVE_SET_PITCH => {
                println!("RECEIVE MSG_MOVE_SET_PITCH");
                handlers
            },
            Opcode::MSG_MOVE_START_SWIM => {
                println!("RECEIVE MSG_MOVE_START_SWIM");
                handlers
            },
            Opcode::MSG_MOVE_STOP_SWIM => {
                println!("RECEIVE MSG_MOVE_STOP_SWIM");
                handlers
            },
            Opcode::MSG_MOVE_SET_FACING => {
                println!("RECEIVE MSG_MOVE_SET_FACING");
                handlers
            },
            // Opcode::SMSG_MONSTER_MOVE => {
            //     println!("RECEIVE SMSG_MONSTER_MOVE");
            //     vec![]
            // },
            _ => {
                vec![]
            },
        };

        Self::collect_responses(handlers, input)
    }
}