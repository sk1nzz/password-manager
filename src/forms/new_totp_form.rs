use iced::{
    Element, Font, Length,
    alignment::Vertical,
    widget::{Row, button, checkbox, column, container, radio, row, space, text, text_input},
};

use crate::forms::LoginType;
use crate::models::{Login, TotpKey};

#[derive(Default)]
pub struct NewTotpForm {
    pub site_name: String,
    pub login: String,
    pub login_type: LoginType,
    pub secret: String,
}

#[derive(Clone)]
pub enum Message {
    SetSiteName(String),
    SetLogin(String),
    SetLoginType(LoginType),
    SetSecret(String),
    Submit,
    Cancel,
}

impl NewTotpForm {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::SetSiteName(site_name) => self.site_name = site_name,
            Message::SetLogin(login) => self.login = login,
            Message::SetLoginType(login_type) => self.login_type = login_type,
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
                    radio(
                        "Логин",
                        LoginType::Username,
                        Some(self.login_type),
                        Message::SetLoginType
                    )
                    .width(Length::FillPortion(1)),
                    radio(
                        "Почта",
                        LoginType::Email,
                        Some(self.login_type),
                        Message::SetLoginType
                    )
                    .width(Length::FillPortion(1)),
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
                    space().width(Length::Fill),
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
