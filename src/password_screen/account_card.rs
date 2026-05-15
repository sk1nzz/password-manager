use iced::{
    Element, Length,
    alignment::Vertical,
    widget::{Row, button, column, container, row, text, text_input},
};

use crate::models::{Account, Login};

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
                text(&self.account.site_name),
                self.view_login(),
                self.view_password(),
                row![
                    button("Изменить")
                        .style(button::subtle)
                        .on_press(Message::ModifyAccount),
                    button("Удалить")
                        .style(button::danger)
                        .on_press(Message::DeleteAccount)
                ]
                .spacing(10)
            ]
            .spacing(10)
            .width(Length::Fill),
        )
        .padding(10)
        .style(container::bordered_box)
        .into()
    }

    fn view_login(&self) -> Row<'_, Message> {
        match &self.account.login {
            Login::Username(username) => row![
                text("Имя пользователя").width(Length::FillPortion(2)),
                text_input("", username).width(Length::FillPortion(3))
            ],
            Login::Email(email) => row![
                text("Почта").width(Length::FillPortion(2)),
                text_input("", email).width(Length::FillPortion(3))
            ],
        }
        .spacing(10)
        .align_y(Vertical::Center)
    }

    fn view_password(&self) -> Row<'_, Message> {
        match self.password_visible {
            true => row![
                text("Пароль").width(Length::FillPortion(4)),
                text_input("", &self.account.password).width(Length::FillPortion(5)),
                button("Скрыть")
                    .on_press(Message::TogglePasswordVisibility)
                    .style(button::subtle)
                    .width(Length::FillPortion(1))
            ],
            false => row![
                text("Пароль").width(Length::FillPortion(2)),
                button("Показать")
                    .on_press(Message::TogglePasswordVisibility)
                    .style(button::subtle)
                    .width(Length::FillPortion(3))
            ],
        }
        .spacing(10)
        .align_y(Vertical::Center)
    }
}
