use iced::{
    Element, Length,
    alignment::{Horizontal, Vertical},
    widget::{
        Column, Container, Row, button, column, container, row, scrollable, space, stack, text,
        text_input,
    },
};

use crate::{
    forms::{
        LoginType,
        new_totp_form::{self, NewTotpForm},
    },
    models::{Login, TotpKey},
};

#[derive(Default)]
pub struct TotpScreen {
    keys: Vec<TotpKey>,
    new_totp_form: NewTotpForm,
    new_totp_form_opened: bool,
}

#[derive(Clone)]
pub enum Message {
    OpenNewTotp,
    NewTotpFormMessage(new_totp_form::Message),
}

impl TotpScreen {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::OpenNewTotp => self.new_totp_form_opened = true,
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
                    self.keys.push(key);
                    self.new_totp_form_opened = false;
                }
                new_totp_form::Message::Cancel => self.new_totp_form_opened = false,
                _ => self.new_totp_form.update(msg),
            },
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        stack![
            scrollable(self.view_keys()),
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

    fn view_new_totp_form(&self) -> Element<'_, Message> {
        if self.new_totp_form_opened {
            self.new_totp_form.view().map(Message::NewTotpFormMessage)
        } else {
            space().into()
        }
    }

    fn view_keys(&self) -> Column<'_, Message> {
        column(self.keys.iter().map(|key| Self::view_key(key).into())).spacing(10)
    }

    fn view_key(key: &TotpKey) -> Container<'_, Message> {
        container(
            column![
                text(&key.site_name),
                match &key.login {
                    Login::Email(email) => text(format!("Почта: {}", email)),
                    Login::Username(username) => text(format!("Логин: {}", username)),
                }
            ]
            .spacing(10)
            .width(Length::Fill),
        )
        .padding(10)
        .style(container::bordered_box)
    }
}
