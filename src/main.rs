use iced::{
    Alignment, Element, Length, Subscription, Task,
    futures::lock,
    widget::{Column, button, column, row, space, text},
};
use std::fs;
use std::{env, ops::Sub};

mod forms;
mod models;
mod password_screen;
mod totp_screen;

use forms::lock_screen;
use forms::welcome_screen;
use models::{ACCOUNT_SQL, TOTP_SQL};
use password_screen::PasswordScreen;
use totp_screen::TotpScreen;

pub fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("Менеджер паролей")
        .subscription(App::subscription)
        .run()
}

enum App {
    NoDatabase(welcome_screen::WelcomeScreen),
    Locked(lock_screen::LockScreen),
    Unlocked(UnlockedState),
}

struct UnlockedState {
    password_screen_state: PasswordScreen,
    totp_screen_state: TotpScreen,
    current_page: CurrentPage,
    connection: rusqlite::Connection,
}

impl UnlockedState {
    fn view_unlocked(&self) -> Element<'_, Message> {
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
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        self.totp_screen_state
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
    LockScreenMessage(lock_screen::Message),
    WelcomeScreenMessage(welcome_screen::Message),
}

impl App {
    fn new() -> Self {
        // let db = rusqlite::Connection::open("./data.db").unwrap();

        // db.execute(ACCOUNT_SQL, ()).unwrap();
        // db.execute(TOTP_SQL, ()).unwrap();
        let mut path = env::home_dir().unwrap();
        path.push(".local/share/password-manager/data.db");

        if fs::exists(path).unwrap() {
            Self::Locked(lock_screen::LockScreen::default())
        } else {
            Self::NoDatabase(welcome_screen::WelcomeScreen::default())
        }

        // Self {
        //     connection: None,
        //     password_screen_state: PasswordScreen::default(),
        //     totp_screen_state: TotpScreen::new(),
        //     current_page: CurrentPage::default(),
        //     lock_state,
        // }
    }

    fn update(&mut self, msg: Message) -> Task<Message> {
        match self {
            Self::NoDatabase(welcome_screen) => match msg {
                Message::WelcomeScreenMessage(msg) => match msg {
                    welcome_screen::Message::Submit => {
                        let state = std::mem::take(welcome_screen);
                        let mut path = env::home_dir().unwrap();
                        path.push(".local/share/password-manager");
                        fs::create_dir_all(&path).unwrap();
                        path.push("data.db");

                        let db = rusqlite::Connection::open(path).unwrap();

                        db.pragma_update(None, "key", state.password).unwrap();

                        db.execute(ACCOUNT_SQL, ()).unwrap();
                        db.execute(TOTP_SQL, ()).unwrap();

                        *self = Self::Unlocked(UnlockedState {
                            connection: db,
                            password_screen_state: PasswordScreen::default(),
                            totp_screen_state: TotpScreen::new(),
                            current_page: CurrentPage::default(),
                        });

                        Task::batch(vec![
                            Task::done(Message::PasswordScreenMessage(
                                password_screen::Message::LoadAccounts,
                            )),
                            Task::done(Message::TotpScreenMessage(totp_screen::Message::LoadKeys)),
                        ])
                    }
                    _ => {
                        welcome_screen.update(msg);
                        Task::none()
                    }
                },
                _ => Task::none(),
            },
            Self::Locked(lock_screen) => match msg {
                Message::LockScreenMessage(msg) => match msg {
                    lock_screen::Message::Submit => {
                        let state = std::mem::take(lock_screen);
                        let mut path = env::home_dir().unwrap();
                        path.push(".local/share/password-manager/data.db");

                        let db = rusqlite::Connection::open(path).unwrap();

                        db.pragma_update(None, "key", state.password).unwrap();

                        match db.query_row("SELECT count(*) FROM sqlite_master", (), |row| {
                            row.get::<_, i32>(0)
                        }) {
                            Ok(_) => {
                                *self = Self::Unlocked(UnlockedState {
                                    connection: db,
                                    password_screen_state: PasswordScreen::default(),
                                    totp_screen_state: TotpScreen::new(),
                                    current_page: CurrentPage::default(),
                                });
                                Task::batch(vec![
                                    Task::done(Message::PasswordScreenMessage(
                                        password_screen::Message::LoadAccounts,
                                    )),
                                    Task::done(Message::TotpScreenMessage(
                                        totp_screen::Message::LoadKeys,
                                    )),
                                ])
                            }
                            Err(_) => {
                                lock_screen.error = Some("Ошибка входа".to_string());
                                Task::none()
                            }
                        }
                    }
                    _ => {
                        lock_screen.update(msg);
                        Task::none()
                    }
                },
                _ => Task::none(),
            },
            Self::Unlocked(unlocked_state) => {
                match msg {
                    Message::SetPage(page) => unlocked_state.current_page = page,
                    Message::PasswordScreenMessage(msg) => unlocked_state
                        .password_screen_state
                        .update(msg, &unlocked_state.connection),
                    Message::TotpScreenMessage(msg) => unlocked_state
                        .totp_screen_state
                        .update(msg, &unlocked_state.connection),
                    _ => (),
                };
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        match self {
            Self::NoDatabase(state) => state.view().map(Message::WelcomeScreenMessage),
            Self::Locked(state) => state.view().map(Message::LockScreenMessage),
            Self::Unlocked(state) => state.view_unlocked(),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        match self {
            Self::Unlocked(state) => state.subscription(),
            _ => Subscription::none(),
        }
    }
}
