use iced::{Element, Subscription};

mod db;
mod forms;
mod models;
mod password_screen;
mod totp_screen;
mod unlocked_screen;

use forms::lock_screen::{self, LockScreen};
use forms::welcome_screen::{self, WelcomeScreen};
use unlocked_screen::UnlockedScreen;

pub fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("Менеджер паролей")
        .subscription(App::subscription)
        .run()
}

enum App {
    NoDatabase(WelcomeScreen),
    Locked(LockScreen),
    Unlocked(UnlockedScreen),
}

#[derive(Clone)]
enum Message {
    LockScreenMessage(lock_screen::Message),
    WelcomeScreenMessage(welcome_screen::Message),
    UnlockedScreenMessage(unlocked_screen::Message),
}

impl App {
    fn new() -> Self {
        if db::check_db_exists() {
            Self::Locked(LockScreen::default())
        } else {
            Self::NoDatabase(WelcomeScreen::default())
        }
    }

    fn update(&mut self, msg: Message) {
        match self {
            Self::NoDatabase(welcome_screen) => match msg {
                Message::WelcomeScreenMessage(msg) => match msg {
                    welcome_screen::Message::Submit => {
                        let state = std::mem::take(welcome_screen);
                        if state.password == state.password_repeat {
                            match db::validate_password(&state.password) {
                                Ok(_) => {
                                    let db = db::init_db_with_password(&state.password);
                                    *self = Self::Unlocked(UnlockedScreen::new(db));
                                }
                                Err(_) => {
                                    welcome_screen.error = Some("Пароль короче 8 символов");
                                }
                            }
                        } else {
                            welcome_screen.error = Some("Пароли не совпадают");
                        }
                    }
                    _ => welcome_screen.update(msg),
                },
                _ => (),
            },
            Self::Locked(lock_screen) => match msg {
                Message::LockScreenMessage(msg) => match msg {
                    lock_screen::Message::Submit => {
                        let state = std::mem::take(lock_screen);
                        match db::unlock_db(&state.password) {
                            Ok(db) => *self = Self::Unlocked(UnlockedScreen::new(db)),
                            Err(()) => lock_screen.error = Some("Login error".to_string()),
                        }
                    }
                    _ => lock_screen.update(msg),
                },
                _ => (),
            },
            Self::Unlocked(unlocked_state) => match msg {
                Message::UnlockedScreenMessage(msg) => unlocked_state.update(msg),
                _ => (),
            },
        }
    }

    fn view(&self) -> Element<'_, Message> {
        match self {
            Self::NoDatabase(welcome_scr) => welcome_scr.view().map(Message::WelcomeScreenMessage),
            Self::Locked(lock_scr) => lock_scr.view().map(Message::LockScreenMessage),
            Self::Unlocked(unlocked_scr) => unlocked_scr.view().map(Message::UnlockedScreenMessage),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        match self {
            Self::Unlocked(state) => state.subscription().map(Message::UnlockedScreenMessage),
            _ => Subscription::none(),
        }
    }
}
