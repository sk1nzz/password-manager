use iced::alignment::Vertical;
use iced::widget::{button, column, container, radio, row, space, text, text_input};
use iced::{Element, Length};
use uuid::Uuid;

#[derive(Default)]
pub struct NewAccountForm {
    pub modify_id: Option<Uuid>,
    pub site_name: String,
    pub login: String,
    pub password: String,
    pub login_type: LoginType,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum LoginType {
    #[default]
    Username,
    Email,
}

#[derive(Clone)]
pub enum Message {
    SetSiteName(String),
    SetLogin(String),
    SetPassword(String),
    SetLoginType(LoginType),
    Submit,
    Cancel,
}

impl NewAccountForm {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::SetSiteName(site_name) => self.site_name = site_name,
            Message::SetLogin(login) => self.login = login,
            Message::SetPassword(password) => self.password = password,
            Message::SetLoginType(login_type) => self.login_type = login_type,
            _ => (),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        container(
            column![
                text(if let Some(_) = self.modify_id {
                    "Изменить аккаунт"
                } else {
                    "Добавить аккаунт"
                })
                .size(20),
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
                    text("Пароль").width(Length::FillPortion(2)),
                    text_input("", &self.password)
                        .on_input(Message::SetPassword)
                        .secure(true)
                        .width(Length::FillPortion(3)),
                ]
                .align_y(Vertical::Center),
                row![
                    space().width(Length::Fill),
                    button("Отмена")
                        .style(button::subtle)
                        .on_press(Message::Cancel),
                    button(if let Some(_) = self.modify_id {
                        "Изменить"
                    } else {
                        "Добавить"
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
