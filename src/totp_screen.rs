mod totp_card;

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use iced::{
    Element, Length, Subscription,
    alignment::{Horizontal, Vertical},
    time::Duration,
    time::every,
    widget::{
        Grid, button, center, column, container, grid, grid::Sizing, scrollable, space, stack, text,
    },
};
use rusqlite::Connection;
use uuid::Uuid;

use crate::models::totp::TotpValidationError;
use crate::{
    forms::new_totp_form::{self, NewTotpForm},
    models::totp::TotpKey,
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
    Tick,
    NewTotpFormMessage(new_totp_form::Message),
    TotpCardMessage(Uuid, totp_card::Message),
}

impl TotpScreen {
    pub fn new(db: &Connection) -> Self {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let offset = ts % 30;

        let keys = TotpKey::get_all(db)
            .into_iter()
            .map(|i| (i.id(), TotpCard::new(i)))
            .collect();

        Self {
            keys,
            timer: offset as u8,
            ..Default::default()
        }
    }

    pub fn update(&mut self, msg: Message, db: &Connection) {
        match msg {
            Message::OpenNewTotp => self.new_totp_form_opened = true,
            Message::NewTotpFormMessage(msg) => match msg {
                new_totp_form::Message::Submit => self.handle_form_submit(db),
                new_totp_form::Message::Cancel => self.new_totp_form_opened = false,
                _ => self.new_totp_form.update(msg),
            },
            Message::Tick => self.handle_tick(),
            Message::TotpCardMessage(id, msg) => match msg {
                totp_card::Message::Delete => {
                    self.keys.remove(&id).unwrap().key.delete(db).unwrap();
                }
            },
        }
    }

    fn handle_form_submit(&mut self, db: &Connection) {
        let res = self.new_totp_form.create_totp();
        match res {
            Ok(key) => match key.save(db) {
                Ok(_) => {
                    self.keys.insert(key.id(), TotpCard::new(key));
                    self.new_totp_form_opened = false;
                }
                Err(_) => {
                    self.new_totp_form.error =
                        Some("Уже есть ключ с таким названием сайта и логином")
                }
            },
            Err(err) => self.handle_validation_error(err),
        }
    }

    fn handle_validation_error(&mut self, err: TotpValidationError) {
        match err {
            TotpValidationError::EmptySiteName => {
                self.new_totp_form.error = Some("Пустое название сайта")
            }
            TotpValidationError::EmptyLogin => self.new_totp_form.error = Some("Пустой логин"),
            TotpValidationError::BadSecret => self.new_totp_form.error = Some("Невалидный секрет"),
        }
    }

    fn handle_tick(&mut self) {
        self.timer += 1;
        if self.timer == 30 {
            for (_, v) in &mut self.keys {
                v.current_code = v.key.gen_key();
            }
            self.timer = 0;
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
            center(self.view_new_totp_form()),
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        every(Duration::from_secs(1)).map(|_| Message::Tick)
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
