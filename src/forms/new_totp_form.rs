use iced::{
    Element, Font, Length,
    alignment::Vertical,
    widget::{button, column, container, row, text, text_input},
};

use crate::models::totp::{TotpKey, TotpValidationError};

#[derive(Default)]
pub struct NewTotpForm {
    pub site_name: String,
    pub login: String,
    pub secret: String,
    pub error: Option<&'static str>,
}

#[derive(Clone)]
pub enum Message {
    SetSiteName(String),
    SetLogin(String),
    SetSecret(String),
    Submit,
    Cancel,
}

impl NewTotpForm {
    pub fn create_totp(&mut self) -> Result<TotpKey, TotpValidationError> {
        let form = std::mem::take(self);
        TotpKey::new(form.site_name, form.login, form.secret)
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::SetSiteName(site_name) => self.site_name = site_name,
            Message::SetLogin(login) => self.login = login,
            Message::SetSecret(secret) => self.secret = secret,
            _ => (),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        container(
            column![
                text("Добавить TOTP-ключ").size(20).font(Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                }),
                row![
                    text("Название сайта").width(Length::FillPortion(2)),
                    text_input("", &self.site_name)
                        .on_input(Message::SetSiteName)
                        .width(Length::FillPortion(3)),
                ]
                .align_y(Vertical::Center),
                row![
                    text("Логин/почта").width(Length::FillPortion(2)),
                    text_input("", &self.login)
                        .on_input(Message::SetLogin)
                        .width(Length::FillPortion(3)),
                ]
                .align_y(Vertical::Center),
                row![
                    text("Ключ").width(Length::FillPortion(2)),
                    text_input("", &self.secret)
                        .on_input(Message::SetSecret)
                        .secure(true)
                        .width(Length::FillPortion(3)),
                ]
                .align_y(Vertical::Center),
                row![
                    text(self.error.unwrap_or_default())
                        .width(Length::Fill)
                        .style(text::danger)
                        .width(Length::Fill),
                    button("Отмена")
                        .style(button::subtle)
                        .on_press(Message::Cancel),
                    button("Добавить").on_press(Message::Submit)
                ]
                .width(Length::Fill)
                .spacing(10)
            ]
            .spacing(10),
        )
        .padding(10)
        .style(container::bordered_box)
        .width(750)
        .into()
    }
}
