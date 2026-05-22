use iced::{
    Element, Font, Length,
    alignment::Vertical,
    widget::{Row, button, column, container, row, text, text_input},
};

use crate::models::{Account, Login};

pub struct TotpScreen {}

#[derive(Clone)]
pub enum Message {}

impl AccountCard {
    pub fn update(&mut self, msg: Message) {
        match msg {}
    }

    pub fn view(&self) -> Element<'_, Message> {}
}
