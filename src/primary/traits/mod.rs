use async_broadcast::{Sender as BroadcastSender, Receiver as BroadcastReceiver};
use tokio::task::JoinHandle;

use crate::primary::types::{HandlerOutput};

pub mod binary_converter;
pub mod packet_handler;
pub mod paginator;
pub mod processor;

pub trait Feature: Send {
    fn new() -> Self where Self: Sized;
    fn set_broadcast_channel(
        &mut self,
        sender: BroadcastSender<HandlerOutput>,
        receiver: BroadcastReceiver<HandlerOutput>,
    );
    fn get_tasks(&mut self) -> Vec<JoinHandle<()>>;
}