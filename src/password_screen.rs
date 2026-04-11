mod new_account_form;

use iced::widget::{
    Column, Container, Row, button, column, container, float, row, scrollable, space, text,
};
use iced::{Element, Length};

use crate::models::{Account, Login};
use crate::password_screen::new_account_form::NewAccountForm;

#[derive(Default)]
pub struct PasswordScreen {
    accounts: Vec<Account>,
    new_account_form: NewAccountForm,
    new_account_form_opened: bool,
}

#[derive(Clone)]
pub enum Message {
    OpenNewAccount,
    NewAccountFormMessage(new_account_form::Message),
}

impl PasswordScreen {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::OpenNewAccount => self.new_account_form_opened = true,
            Message::NewAccountFormMessage(msg) => match msg {
                new_account_form::Message::Cancel => self.new_account_form_opened = false,
                new_account_form::Message::Submit => self.accounts.push(Account::new(
                    self.new_account_form.site_name.clone(),
                    match self.new_account_form.login_type {
                        new_account_form::LoginType::Username => {
                            Login::Username(self.new_account_form.login.clone())
                        }
                        new_account_form::LoginType::Email => {
                            Login::Email(self.new_account_form.login.clone())
                        }
                    },
                    self.new_account_form.password.clone(),
                )),
                _ => self.new_account_form.update(msg),
            },
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        column![
            float(button("Новый").on_press(Message::OpenNewAccount)),
            self.view_new_account_form(),
            scrollable(self.view_accounts()),
        ]
        .spacing(10)
        .into()
    }

    fn view_new_account_form(&self) -> Element<'_, Message> {
        if self.new_account_form_opened {
            self.new_account_form
                .view()
                .map(Message::NewAccountFormMessage)
        } else {
            space().into()
        }
    }

    fn view_accounts(&self) -> Column<'_, Message> {
        column(
            self.accounts
                .iter()
                .map(|acc| self.view_account(acc).into()),
        )
        .spacing(10)
    }

    fn view_account<'a>(&self, acc: &'a Account) -> Container<'a, Message> {
        container(
            column![text(&acc.site_name), Self::view_login(&acc.login)]
                .spacing(10)
                .width(Length::Fill),
        )
        .padding(10)
        .style(container::bordered_box)
    }

    fn view_login(login: &Login) -> Row<'_, Message> {
        match login {
            Login::Username(username) => row![text("Имя пользователя"), text(username)],
            Login::Email(email) => row![text("Почта"), text(email)],
        }
        .spacing(10)
    }
}
