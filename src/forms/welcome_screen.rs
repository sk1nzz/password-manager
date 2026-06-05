use iced::{
    Element, Font, Length,
    alignment::Vertical,
    font::Weight,
    widget::{button, center, column, container, row, text, text_input},
};

#[derive(Default)]
pub struct WelcomeScreen {
    pub password: String,
    pub password_repeat: String,
    pub error: Option<&'static str>,
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

    pub fn view(&self) -> Element<'_, Message> {
        center(
            container(
                column![
                    text("Добро пожаловать в Менеджер паролей!").font(Font {
                        weight: Weight::Bold,
                        ..Default::default()
                    }),
                    text("Придумайте пароль, которым будет зашифрована база данных."),
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
                    row![
                        text(self.error.unwrap_or_default())
                            .style(text::danger)
                            .width(Length::Fill),
                        button("Продолжить").on_press(Message::Submit)
                    ]
                    .align_y(Vertical::Center)
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
