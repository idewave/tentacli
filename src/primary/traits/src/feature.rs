use anyhow::{Result as AnyResult};
use tokio::task::JoinHandle;
use async_broadcast::{Receiver as BroadcastReceiver, Sender as BroadcastSender};

use crate::types::HandlerOutput;

pub trait Feature: Send {
    fn new() -> Self where Self: Sized;
    fn set_broadcast_channel(
        &mut self,
        sender: BroadcastSender<HandlerOutput>,
        receiver: BroadcastReceiver<HandlerOutput>,
    );
    fn get_tasks(&mut self) -> AnyResult<Vec<JoinHandle<()>>>;
}