use iced::{
    Element, Length,
    widget::{button, column, container, text},
};

use crate::models::totp::TotpKey;

pub struct TotpCard {
    pub key: TotpKey,
    pub current_code: String,
}

#[derive(Clone)]
pub enum Message {
    Delete,
}

impl TotpCard {
    pub fn new(key: TotpKey) -> Self {
        let init_code = key.gen_key();
        Self {
            key,
            current_code: init_code,
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        container(
            column![
                text(self.key.site_name()),
                text(self.key.login()),
                text(&self.current_code).size(30).style(text::primary),
                button("Удалить")
                    .style(button::danger)
                    .on_press(Message::Delete)
            ]
            .spacing(10)
            .width(Length::Fill),
        )
        .padding(10)
        .style(container::bordered_box)
        .into()
    }
}
