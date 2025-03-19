use std::cmp::PartialEq;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_broadcast::{Receiver, Sender};
use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyModifiers};
use futures::{FutureExt, StreamExt};
use ratatui::{DefaultTerminal, Frame};
use ratatui::style::Color;
use tentacli_traits::{Feature, FeatureError};
use tentacli_traits::types::{HandlerOutput, Task};
use tentacli_traits::types::realm::Realm;
use tentacli_traits::types::shared::Object;
use tokio::sync::mpsc;

use crate::features::ui2::builders::modal::{CharacterItem, RealmItem};
use crate::features::ui2::components::{App, ComponentEvent, ModalResponse};
pub(crate) use crate::features::ui2::components::{DEBG_MSG, DONE_MSG, FAIL_MSG, RECV_MSG, SENT_MSG};
use crate::features::ui2::traits::{EventHandler, UIComponent};

mod traits;
mod components;
pub mod layout;
mod builders;

#[derive(Default)]
pub struct UI2 {
    sender: Option<Sender<HandlerOutput>>,
    receiver: Option<Receiver<HandlerOutput>>,
}

impl Feature for UI2 {
    fn set_broadcast_channel(
        &mut self,
        sender: Sender<HandlerOutput>,
        receiver: Receiver<HandlerOutput>,
    ) {
        self.sender = Some(sender);
        self.receiver = Some(receiver);
    }

    fn get_tasks(&mut self) -> anyhow::Result<Vec<Task>> {
        let mut receiver = self.receiver.as_mut().ok_or(FeatureError::ReceiverNotFound)?.clone();
        let sender = self.sender.as_mut().ok_or(FeatureError::SenderNotFound)?.clone();

        let app = Arc::new(Mutex::new(App::new()));
        let state = Arc::new(Mutex::new(UIState::default()));

        let handle_io = || {
            let mut sender = sender.clone();
            let app = Arc::clone(&app);
            let state = Arc::clone(&state);

            tokio::spawn(async move {
                let mut reader = EventStream::new();

                loop {
                    let next_event = reader.next().fuse();

                    tokio::select! {
                        Some(Ok(event)) = next_event => {
                            if let Event::Key(KeyEvent { code, modifiers: mods, .. }) = event {
                                match code {
                                    KeyCode::Char('c') if mods.contains(KeyModifiers::CONTROL) => {
                                        sender.broadcast(HandlerOutput::Drop).await?;
                                        *state.lock().unwrap() = UIState::Exiting;
                                    },
                                    _ => {
                                        let outputs = app.lock().unwrap().handle_event(
                                            ComponentEvent::KeyEvent(code, mods)
                                        )?;

                                        if let Some(outputs) = outputs {
                                            for output in outputs {
                                                sender.broadcast(output).await?;
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        Ok(input) = receiver.recv() => {
                            match input {
                                HandlerOutput::SuccessMessage(msg, _) => {
                                    let event = ComponentEvent::LogItem(DONE_MSG, msg);
                                    app.lock().unwrap().handle_event(event)?;
                                },
                                HandlerOutput::ErrorMessage(msg, _) => {
                                    let event = ComponentEvent::LogItem(FAIL_MSG, msg);
                                    app.lock().unwrap().handle_event(event)?;
                                },
                                HandlerOutput::DebugMessage(msg, _) => {
                                    let event = ComponentEvent::LogItem(DEBG_MSG, msg);
                                    app.lock().unwrap().handle_event(event)?;
                                },
                                HandlerOutput::ResponseMessage(msg, _) => {
                                    let event = ComponentEvent::LogItem(RECV_MSG, msg);
                                    app.lock().unwrap().handle_event(event)?;
                                },
                                HandlerOutput::RequestMessage(msg, _) => {
                                    let event = ComponentEvent::LogItem(SENT_MSG, msg);
                                    app.lock().unwrap().handle_event(event)?;
                                },
                                HandlerOutput::TransferRealmsList(mut realm_list) => {
                                    let items: Vec<RealmItem> = realm_list
                                        .clone()
                                        .iter()
                                        .map(|r| {
                                            RealmItem {
                                                name: r.name.to_string(),
                                                address: r.address.to_string(),
                                                server_id: r.server_id.to_string()
                                            }
                                        }).collect();

                                    let callback = Box::new(move |indexes: Vec<usize>| {
                                        let realm = realm_list.swap_remove(indexes[0]);

                                        Ok(ModalResponse::External(
                                            vec![HandlerOutput::SelectRealm(realm)]
                                        ))
                                    });

                                    app.lock().unwrap().handle_event(
                                        ComponentEvent::RealmList(items, callback)
                                    )?;
                                },
                                HandlerOutput::TransferCharactersList(mut character_list) => {
                                    let items = character_list
                                        .clone()
                                        .iter()
                                        .map(|c| {
                                            CharacterItem {
                                                name: c.name.to_string(),
                                                guid: c.guid.to_string(),
                                            }
                                        }).collect();

                                    let callback = Box::new(move |indexes: Vec<usize>| {
                                        let character = character_list.swap_remove(indexes[0]);

                                        Ok(ModalResponse::External(
                                            vec![HandlerOutput::SelectCharacter(character.guid)]
                                        ))
                                    });

                                    app.lock().unwrap().handle_event(
                                        ComponentEvent::CharactersList(items, callback)
                                    )?;
                                },
                                _ => {},
                            }
                        },
                    }
                }
            })
        };

        let handle_render = || {
            let sender = sender.clone();
            let mut terminal = ratatui::init();
            let app = Arc::clone(&app);
            let state = Arc::clone(&state);

            tokio::spawn(async move {
                loop {
                    if *state.lock().unwrap() == UIState::Exiting {
                        break;
                    }

                    terminal.draw(|frame| {
                        app.lock().unwrap().render(frame, frame.area());
                    })?;

                    tokio::time::sleep(Duration::from_millis(25)).await;
                }

                ratatui::restore();

                Ok(())
            })
        };

        Ok(vec![
            handle_io(),
            handle_render(),
        ])
    }
}

#[derive(Default, PartialEq)]
enum UIState {
    #[default]
    Idle,
    Exiting,
}
