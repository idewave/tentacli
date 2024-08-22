use std::collections::BTreeMap;
use anyhow::{Result as AnyResult};
use async_broadcast::{Sender as BroadcastSender, Receiver as BroadcastReceiver};
use tentacli_traits::{Feature, FeatureError, Processor};
use tentacli_traits::types::{HandlerOutput, ProcessorFunction, ProcessorResult};
use tentacli_traits::types::opcodes::Opcode;
use tokio::task::JoinHandle;

mod auth;
mod realm;

pub use auth::{LoginChallengeResponse, LoginProofResponse, RealmlistResponse};

use auth::AuthProcessor;
use crate::features::wotlk_login::realm::packet::LogoutOutcoming;
use crate::features::wotlk_login::realm::RealmProcessor;

pub struct WotlkLogin {
    _receiver: Option<BroadcastReceiver<HandlerOutput>>,
    _sender: Option<BroadcastSender<HandlerOutput>>,
}

impl Feature for WotlkLogin {
    fn new() -> Self where Self: Sized {
        Self {
            _receiver: None,
            _sender: None,
        }
    }

    fn set_broadcast_channel(
        &mut self,
        sender: BroadcastSender<HandlerOutput>,
        receiver: BroadcastReceiver<HandlerOutput>
    ) {
        self._sender = Some(sender);
        self._receiver = Some(receiver);
    }

    fn get_tasks(&mut self) -> AnyResult<Vec<JoinHandle<()>>> {
        let sender = self._sender.as_ref().ok_or(FeatureError::SenderNotFound)?.clone();
        let mut receiver = self._receiver.as_mut().ok_or(FeatureError::ReceiverNotFound)?.clone();

        let handle_exit = || {
            tokio::spawn(async move {
                loop {
                    if let Ok(output) = receiver.recv().await {
                        match output {
                            HandlerOutput::ExitRequest => {
                                sender.broadcast(
                                    HandlerOutput::Data(
                                        LogoutOutcoming::default()
                                            .unpack_with_client_opcode(Opcode::CMSG_LOGOUT_REQUEST)
                                            .unwrap()
                                    )
                                ).await.unwrap();
                            },
                            _ => {},
                        }
                    }
                }
            })
        };

        Ok(vec![handle_exit()])
    }

    fn get_login_processors(&self) -> Vec<ProcessorFunction> {
        vec![
            Box::new(AuthProcessor::get_handlers),
        ]
    }

    fn get_one_time_handler_maps(&self) -> Vec<BTreeMap<u16, ProcessorResult>> {
        vec![
            RealmProcessor::get_one_time_handler_map(),
        ]
    }

    fn get_initial_processors(&self) -> Vec<ProcessorFunction> {
        vec![
            Box::new(AuthProcessor::get_initial_handlers)
        ]
    }
}