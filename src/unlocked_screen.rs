use iced::{
    Alignment, Element, Length, Subscription, Theme,
    widget::{button, column, row, space, text},
};
use rusqlite::Connection;

use crate::password_screen::{self, PasswordScreen};
use crate::totp_screen::{self, TotpScreen};

pub struct UnlockedScreen {
    password_screen_state: PasswordScreen,
    totp_screen_state: TotpScreen,
    current_page: CurrentPage,
    db: rusqlite::Connection,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum CurrentPage {
    #[default]
    PasswordScreen,
    CodeScreen,
    SettingsScreen,
}

#[derive(Clone)]
pub enum Message {
    SetPage(CurrentPage),
    PasswordScreenMessage(password_screen::Message),
    TotpScreenMessage(totp_screen::Message),
}

impl UnlockedScreen {
    pub fn new(db: Connection) -> Self {
        let password_screen_state = PasswordScreen::new(&db);
        let totp_screen_state = TotpScreen::new(&db);

        Self {
            db,
            password_screen_state,
            totp_screen_state,
            current_page: CurrentPage::default(),
        }
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::SetPage(page) => self.current_page = page,
            Message::PasswordScreenMessage(msg) => self.password_screen_state.update(msg, &self.db),
            Message::TotpScreenMessage(msg) => self.totp_screen_state.update(msg, &self.db),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
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

    pub fn subscription(&self) -> Subscription<Message> {
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

    fn button_style(&self, page: CurrentPage) -> fn(&Theme, button::Status) -> button::Style {
        if self.current_page == page {
            button::secondary
        } else {
            button::subtle
        }
    }
}
