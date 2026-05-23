use iced::{
    Alignment, Element, Length, Subscription, Task,
    widget::{Column, button, column, row, space, text},
};

mod forms;
mod models;
mod password_screen;
mod totp_screen;

use models::{ACCOUNT_SQL, TOTP_SQL};
use password_screen::PasswordScreen;

use totp_screen::TotpScreen;

pub fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("Менеджер паролей")
        .subscription(App::subscription)
        .run()
}

struct App {
    password_screen_state: PasswordScreen,
    totp_screen_state: TotpScreen,
    current_page: CurrentPage,
    connection: rusqlite::Connection,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum CurrentPage {
    #[default]
    PasswordScreen,
    CodeScreen,
    SettingsScreen,
}

#[derive(Clone)]
enum Message {
    SetPage(CurrentPage),
    PasswordScreenMessage(password_screen::Message),
    TotpScreenMessage(totp_screen::Message),
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let db = rusqlite::Connection::open("./data.db").unwrap();

        db.execute(ACCOUNT_SQL, ()).unwrap();
        db.execute(TOTP_SQL, ()).unwrap();

        (
            Self {
                connection: db,
                password_screen_state: PasswordScreen::default(),
                totp_screen_state: TotpScreen::new(),
                current_page: CurrentPage::default(),
            },
            Task::batch(vec![
                Task::done(Message::PasswordScreenMessage(
                    password_screen::Message::LoadAccounts,
                )),
                Task::done(Message::TotpScreenMessage(totp_screen::Message::LoadKeys)),
            ]),
        )
    }

    fn update(&mut self, msg: Message) {
        match msg {
            Message::SetPage(page) => self.current_page = page,
            Message::PasswordScreenMessage(msg) => {
                self.password_screen_state.update(msg, &self.connection)
            }
            Message::TotpScreenMessage(msg) => self.totp_screen_state.update(msg, &self.connection),
        }
    }

    fn view(&self) -> Column<'_, Message> {
        column![
            row![
                text("Менеджер паролей").size(30).width(Length::Fill),
                button("Аккаунты")
                    .on_press(Message::SetPage(CurrentPage::PasswordScreen))
                    .style(self.button_style(CurrentPage::PasswordScreen)),
                button("Коды")
                    .on_press(Message::SetPage(CurrentPage::CodeScreen))
                    .style(self.button_style(CurrentPage::CodeScreen)),
                button("Настройки")
                    .on_press(Message::SetPage(CurrentPage::SettingsScreen))
                    .style(self.button_style(CurrentPage::SettingsScreen))
            ]
            .align_y(Alignment::Center)
            .spacing(10),
            self.view_page()
        ]
        .padding(10)
    }

    fn subscription(app: &Self) -> Subscription<Message> {
        app.totp_screen_state
            .subscription()
            .map(Message::TotpScreenMessage)
    }

    fn view_page(&self) -> Element<'_, Message> {
        match self.current_page {
            CurrentPage::PasswordScreen => self
                .password_screen_state
                .view()
                .map(Message::PasswordScreenMessage),
            CurrentPage::CodeScreen => self
                .totp_screen_state
                .view()
                .map(Message::TotpScreenMessage),
            _ => space().into(),
        }
    }

    fn button_style(
        &self,
        page: CurrentPage,
    ) -> fn(&iced::Theme, iced::widget::button::Status) -> iced::widget::button::Style {
        if self.current_page == page {
            button::secondary
        } else {
            button::subtle
        }
    }
}
