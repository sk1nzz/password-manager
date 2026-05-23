use iced::{
    Element, Length,
    widget::{button, column, container, text},
};

use crate::models::{Login, TotpKey};

pub struct TotpCard {
    pub key: TotpKey,
    current_code: String,
}

#[derive(Clone)]
pub enum Message {
    Refresh,
    Delete,
}

impl TotpCard {
    pub fn new(key: TotpKey) -> Self {
        let init_code = key.data.generate_current().unwrap();
        Self {
            key,
            current_code: init_code,
        }
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::Refresh => {
                self.current_code = self.key.data.generate_current().unwrap();
            }
            _ => (),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        container(
            column![
                text(&self.key.site_name),
                match &self.key.login {
                    Login::Email(email) => text(format!("Почта: {}", email)),
                    Login::Username(username) => text(format!("Логин: {}", username)),
                },
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
