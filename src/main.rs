use iced::{
    Alignment, Element, Font, Length,
    widget::{Column, button, column, row, space, text},
};

mod models;
mod password_screen;

use password_screen::PasswordScreen;

pub fn main() -> iced::Result {
    iced::run(App::update, App::view)
}

#[derive(Default)]
struct App {
    password_screen_state: PasswordScreen,
    current_page: CurrentPage,
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
}

impl App {
    fn update(&mut self, msg: Message) {
        match msg {
            Message::SetPage(page) => self.current_page = page,
            Message::PasswordScreenMessage(msg) => self.password_screen_state.update(msg),
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

    fn view_page(&self) -> Element<'_, Message> {
        match self.current_page {
            CurrentPage::PasswordScreen => self
                .password_screen_state
                .view()
                .map(Message::PasswordScreenMessage),
            _ => space().into(),
        }
    }

    fn button_style(
        &self,
        page: CurrentPage,
    ) -> fn(&iced::Theme, iced::widget::button::Status) -> iced::widget::button::Style {
        if self.current_page == page {
            button::primary
        } else {
            button::subtle
        }
    }
}
