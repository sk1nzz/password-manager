use iced::{
    Element, Length,
    alignment::Vertical,
    widget::{button, center, column, container, row, space, text, text_input},
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
}

impl LockScreen {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::SetPassword(pass) => self.password = pass,
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
                    text("Введите пароль"),
                    self.view_error(),
                    row![
                        text_input("", &self.password)
                            .on_input(Message::SetPassword)
                            .secure(true)
                            .width(Length::Fill),
                        button("Разблокировать").on_press(Message::Submit)
                    ]
                    .spacing(10)
                    .align_y(Vertical::Center),
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
