use std::collections::BTreeMap;

use async_broadcast::{Receiver as BroadcastReceiver, Sender as BroadcastSender};
use tentacli_traits::{Feature, FeatureError, Processor};
use tentacli_traits::types::{HandlerOutput, ProcessorFunction, ProcessorResult, Task};
use tentacli_traits::types::opcodes::Opcode;

pub use auth::{LoginChallengeResponse, LoginProofResponse, RealmlistResponse};
use auth::AuthProcessor;

use crate::features::wotlk_login::realm::packet::LogoutOutcoming;
use crate::features::wotlk_login::realm::RealmProcessor;

mod auth;
mod realm;

#[derive(Default)]
pub struct WotlkLogin {
    _receiver: Option<BroadcastReceiver<HandlerOutput>>,
    _sender: Option<BroadcastSender<HandlerOutput>>,
}

impl Feature for WotlkLogin {
    fn set_broadcast_channel(
        &mut self,
        sender: BroadcastSender<HandlerOutput>,
        receiver: BroadcastReceiver<HandlerOutput>,
    ) {
        self._sender = Some(sender);
        self._receiver = Some(receiver);
    }

    fn get_tasks(&mut self) -> anyhow::Result<Vec<Task>> {
        let sender = self._sender.as_ref().ok_or(FeatureError::SenderNotFound)?.clone();
        let mut receiver = self._receiver.as_mut().ok_or(FeatureError::ReceiverNotFound)?.clone();

        let handle_exit = || {
            tokio::spawn(async move {
                loop {
                    if let Ok(HandlerOutput::ExitRequest) = receiver.recv().await {
                        sender.broadcast(
                            HandlerOutput::Data(LogoutOutcoming::default()
                                .unpack_with_client_opcode(Opcode::CMSG_LOGOUT_REQUEST)
                                .unwrap()
                            )
                        ).await.unwrap();
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