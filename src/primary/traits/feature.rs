use tokio::task::JoinHandle;
use crate::async_broadcast::{BroadcastReceiver, BroadcastSender};
use crate::types::HandlerOutput;

pub trait Feature: Send {
    fn new() -> Self where Self: Sized;
    fn set_broadcast_channel(
        &mut self,
        sender: BroadcastSender<HandlerOutput>,
        receiver: BroadcastReceiver<HandlerOutput>,
    );
    fn get_tasks(&mut self) -> Vec<JoinHandle<()>>;
}