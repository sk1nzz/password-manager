use iced::{
    Element, Length,
    alignment::Vertical,
    widget::{button, column, row, space, text, text_input},
};

#[derive(Default)]
pub struct LockScreen {
    pub password: String,
    pub error: Option<String>,
}

#[derive(Clone)]
pub enum Message {
    SetPassword(String),
    Submit,
    SetError(String),
}

impl LockScreen {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::SetPassword(pass) => self.password = pass,
            Message::SetError(err) => self.error = Some(err),
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
        column![
            text("Введите пароль"),
            self.view_error(),
            row![
                text("Пароль").width(Length::FillPortion(2)),
                text_input("", &self.password)
                    .on_input(Message::SetPassword)
                    .secure(true)
                    .width(Length::FillPortion(3)),
            ]
            .align_y(Vertical::Center),
            button("Разблокировать").on_press(Message::Submit)
        ]
        .into()
    }
}
