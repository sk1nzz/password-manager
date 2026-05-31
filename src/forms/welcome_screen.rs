use iced::{
    Element, Length,
    alignment::Vertical,
    widget::{button, center, column, container, row, space, text, text_input},
};

#[derive(Default)]
pub struct WelcomeScreen {
    pub password: String,
    pub password_repeat: String,
    error: Option<String>,
}

#[derive(Clone)]
pub enum Message {
    SetPassword(String),
    SetPasswordRepeat(String),
    Submit,
}

impl WelcomeScreen {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::SetPassword(pass) => self.password = pass,
            Message::SetPasswordRepeat(pass) => self.password_repeat = pass,
            Message::Submit => (),
        }
    }

    fn view_error(&self) -> Element<'_, Message> {
        if let Some(err) = &self.error {
            text(err).style(text::danger).into()
        } else {
            space().into()
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        center(
            container(
                column![
                    text("Добро пожаловать в Менеджер паролей!"),
                    text("Придумайте пароль, которым будет зашифрована база данных."),
                    self.view_error(),
                    row![
                        text("Пароль").width(Length::FillPortion(2)),
                        text_input("", &self.password)
                            .on_input(Message::SetPassword)
                            .secure(true)
                            .width(Length::FillPortion(3)),
                    ]
                    .align_y(Vertical::Center),
                    row![
                        text("Повторите пароль").width(Length::FillPortion(2)),
                        text_input("", &self.password_repeat)
                            .on_input(Message::SetPasswordRepeat)
                            .secure(true)
                            .width(Length::FillPortion(3)),
                    ]
                    .align_y(Vertical::Center),
                    button("Продолжить").on_press(Message::Submit)
                ]
                .spacing(10),
            )
            .style(container::bordered_box)
            .padding(10)
            .width(750),
        )
        .into()
    }
}
