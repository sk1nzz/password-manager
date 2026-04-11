use iced::widget::{button, column, container, radio, row, space, text, text_input};
use iced::{Element, Length};

#[derive(Default)]
pub struct NewAccountForm {
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
                text("Добавить аккаунт"),
                text_input("Название сайта", &self.site_name).on_input(Message::SetSiteName),
                row![
                    radio(
                        "Имя пользователя",
                        LoginType::Username,
                        Some(self.login_type),
                        Message::SetLoginType
                    ),
                    radio(
                        "Почта",
                        LoginType::Email,
                        Some(self.login_type),
                        Message::SetLoginType
                    ),
                ],
                text_input("", &self.login).on_input(Message::SetLogin),
                text_input("Пароль", &self.password).on_input(Message::SetPassword),
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
        .into()
    }
}
