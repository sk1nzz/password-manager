mod totp_card;

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use iced::{
    Element, Length, Subscription,
    alignment::{Horizontal, Vertical},
    time::every,
    time::{Duration, Instant},
    widget::{
        Column, Container, Grid, Row, button, column, container, grid, grid::Sizing, row,
        scrollable, space, stack, text, text_input,
    },
};
use rusqlite::Connection;
use uuid::Uuid;

use crate::{
    forms::{
        LoginType,
        new_totp_form::{self, NewTotpForm},
    },
    models::{Login, TotpKey},
    totp_screen::totp_card::TotpCard,
};

#[derive(Default)]
pub struct TotpScreen {
    keys: HashMap<Uuid, TotpCard>,
    timer: u8,
    new_totp_form: NewTotpForm,
    new_totp_form_opened: bool,
}

#[derive(Clone)]
pub enum Message {
    OpenNewTotp,
    LoadKeys,
    Tick(Instant),
    NewTotpFormMessage(new_totp_form::Message),
    DeleteKey(Uuid),
    TotpCardMessage(Uuid, totp_card::Message),
}

impl TotpScreen {
    pub fn new() -> Self {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let offset = ts % 30;
        Self {
            timer: offset as u8,
            ..Default::default()
        }
    }

    pub fn update(&mut self, msg: Message, db: &Connection) {
        match msg {
            Message::OpenNewTotp => self.new_totp_form_opened = true,
            Message::LoadKeys => {
                let keys = TotpKey::get_all(db);
                self.keys = keys.into_iter().map(|i| (i.id, TotpCard::new(i))).collect();
            }
            Message::NewTotpFormMessage(msg) => match msg {
                new_totp_form::Message::Submit => {
                    let form = std::mem::take(&mut self.new_totp_form);
                    let key = TotpKey::new(
                        form.site_name,
                        match form.login_type {
                            LoginType::Email => Login::Email(form.login),
                            LoginType::Username => Login::Username(form.login),
                        },
                        form.secret,
                    );
                    key.save(db);
                    self.keys.insert(key.id, TotpCard::new(key));
                    self.new_totp_form_opened = false;
                }
                new_totp_form::Message::Cancel => self.new_totp_form_opened = false,
                _ => self.new_totp_form.update(msg),
            },
            Message::DeleteKey(id) => {
                TotpKey::delete(db, id);
                self.keys.remove(&id);
            }
            Message::Tick(_) => {
                self.timer += 1;
                if self.timer == 30 {
                    for (_, v) in &mut self.keys {
                        v.update(totp_card::Message::Refresh);
                    }
                    self.timer = 0;
                }
            }
            Message::TotpCardMessage(id, msg) => match msg {
                totp_card::Message::Delete => {
                    TotpKey::delete(db, id);
                    self.keys.remove(&id);
                }
                _ => self.keys.get_mut(&id).unwrap().update(msg),
            },
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        stack![
            scrollable(column![
                text(format!("Обновление через {} сек", 30 - self.timer)),
                self.view_keys()
            ]),
            container(button("Новый").on_press(Message::OpenNewTotp))
                .align_x(Horizontal::Right)
                .align_y(Vertical::Bottom)
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(10),
            container(self.view_new_totp_form())
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .width(Length::Fill)
                .height(Length::Fill),
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        every(Duration::from_secs(1)).map(Message::Tick)
    }

    fn view_new_totp_form(&self) -> Element<'_, Message> {
        if self.new_totp_form_opened {
            self.new_totp_form.view().map(Message::NewTotpFormMessage)
        } else {
            space().into()
        }
    }

    fn view_keys(&self) -> Grid<'_, Message> {
        grid(self.keys.iter().map(|key| {
            key.1
                .view()
                .map(|msg| Message::TotpCardMessage(key.0.clone(), msg))
        }))
        .columns(2)
        .spacing(10)
        .height(Sizing::EvenlyDistribute(Length::Shrink))
    }
}
