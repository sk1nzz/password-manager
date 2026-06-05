use iced::{
    Element, Font, Length,
    alignment::Vertical,
    widget::{Row, button, column, container, row, text, text_input},
};

use crate::models::account::Account;

pub struct AccountCard {
    pub account: Account,
    password_visible: bool,
}

#[derive(Clone)]
pub enum Message {
    TogglePasswordVisibility,
    DeleteAccount,
    ModifyAccount,
}

impl AccountCard {
    pub fn new(acc: Account) -> Self {
        Self {
            account: acc,
            password_visible: false,
        }
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::TogglePasswordVisibility => self.password_visible = !self.password_visible,
            Message::ModifyAccount => (),
            Message::DeleteAccount => (),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        container(
            column![
                row![
                    text(self.account.site_name())
                        .font(Font {
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        })
                        .width(Length::Fill),
                    button("Изменить")
                        .style(button::subtle)
                        .on_press(Message::ModifyAccount),
                    button("Удалить")
                        .style(button::danger)
                        .on_press(Message::DeleteAccount)
                ]
                .spacing(10),
                self.view_login(),
                self.view_password(),
            ]
            .spacing(10)
            .width(Length::Fill),
        )
        .padding(10)
        .style(container::bordered_box)
        .into()
    }

    fn view_login(&self) -> Row<'_, Message> {
        row![
            text("Логин/почта").width(Length::FillPortion(4)),
            text_input("", self.account.login()).width(Length::FillPortion(6))
        ]
        .align_y(Vertical::Center)
    }

    fn view_password(&self) -> Row<'_, Message> {
        match self.password_visible {
            true => row![
                text("Пароль").width(Length::FillPortion(4)),
                text_input("", &self.account.password()).width(Length::FillPortion(5)),
                button("Скрыть")
                    .on_press(Message::TogglePasswordVisibility)
                    .style(button::subtle)
                    .width(Length::FillPortion(1))
            ],
            false => row![
                text("Пароль").width(Length::FillPortion(4)),
                text_input("", "*************").width(Length::FillPortion(5)),
                button("Показать")
                    .on_press(Message::TogglePasswordVisibility)
                    .style(button::subtle)
                    .width(Length::FillPortion(1))
            ],
        }
        .align_y(Vertical::Center)
    }
}
