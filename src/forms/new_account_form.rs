use iced::alignment::Vertical;
use iced::widget::{button, column, container, row, text, text_input};
use iced::{Element, Font, Length};
use uuid::Uuid;

use crate::models::account::{Account, AccountValidationError};

#[derive(Default)]
pub struct NewAccountForm {
    pub mode: FormMode,
    pub site_name: String,
    pub login: String,
    pub password: String,
    pub error: Option<&'static str>,
}

#[derive(Default)]
pub enum FormMode {
    #[default]
    Create,
    Modify(Uuid),
}

#[derive(Clone)]
pub enum Message {
    SetSiteName(String),
    SetLogin(String),
    SetPassword(String),
    Submit,
    Cancel,
}

impl NewAccountForm {
    pub fn create_account(&mut self) -> Result<Account, AccountValidationError> {
        let form = std::mem::take(self);
        Account::new(form.site_name, form.login, form.password)
    }

    pub fn init_modify(&mut self, acc: &Account) {
        self.mode = FormMode::Modify(acc.id());
        self.site_name = acc.site_name().to_string();
        self.login = acc.login().to_string();
        self.password = acc.password().to_string();
        self.error = None;
    }

    pub fn modify_account(&mut self, acc: &mut Account) -> Result<(), AccountValidationError> {
        let form = std::mem::take(self);
        acc.set_site_name(form.site_name)?;
        acc.set_login(form.login)?;
        acc.set_password(form.password)?;
        Ok(())
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::SetSiteName(site_name) => self.site_name = site_name,
            Message::SetLogin(login) => self.login = login,
            Message::SetPassword(password) => self.password = password,
            _ => (),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        container(
            column![
                text(match self.mode {
                    FormMode::Create => "Добавить аккаунт",
                    FormMode::Modify(_) => "Изменить аккаунт",
                })
                .size(20)
                .font(Font {
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
                    text("Пароль").width(Length::FillPortion(2)),
                    text_input("", &self.password)
                        .on_input(Message::SetPassword)
                        .secure(true)
                        .width(Length::FillPortion(3)),
                ]
                .align_y(Vertical::Center),
                row![
                    text(self.error.unwrap_or_default())
                        .width(Length::Fill)
                        .style(text::danger),
                    button("Отмена")
                        .style(button::subtle)
                        .on_press(Message::Cancel),
                    button(match self.mode {
                        FormMode::Create => "Добавить",
                        FormMode::Modify(_) => "Сохранить",
                    })
                    .on_press(Message::Submit)
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
