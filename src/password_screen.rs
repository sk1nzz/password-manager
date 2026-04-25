mod account_card;
mod new_account_form;

use std::collections::HashMap;

use iced::widget::{
    Column, Container, Row, button, column, container, float, row, scrollable, space, text,
};
use iced::{Element, Length};
use uuid::Uuid;

use crate::models::{Account, Login};
use crate::password_screen::{account_card::AccountCard, new_account_form::NewAccountForm};

#[derive(Default)]
pub struct PasswordScreen {
    account_cards: HashMap<Uuid, AccountCard>,
    new_account_form: NewAccountForm,
    new_account_form_opened: bool,
}

#[derive(Clone)]
pub enum Message {
    OpenNewAccount,
    NewAccountFormMessage(new_account_form::Message),
    AccountCardMessage(Uuid, account_card::Message),
}

impl PasswordScreen {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::OpenNewAccount => self.new_account_form_opened = true,
            Message::NewAccountFormMessage(msg) => match msg {
                new_account_form::Message::Cancel => self.new_account_form_opened = false,
                new_account_form::Message::Submit => {
                    let login = match self.new_account_form.login_type {
                        new_account_form::LoginType::Username => {
                            Login::Username(self.new_account_form.login.clone())
                        }
                        new_account_form::LoginType::Email => {
                            Login::Email(self.new_account_form.login.clone())
                        }
                    };
                    let acc = Account::new(
                        self.new_account_form.site_name.clone(),
                        login,
                        self.new_account_form.password.clone(),
                    );
                    self.account_cards.insert(acc.id, AccountCard::new(acc));
                }
                _ => self.new_account_form.update(msg),
            },
            Message::AccountCardMessage(uuid, msg) => match msg {
                account_card::Message::DeleteAccount => {
                    self.account_cards.remove(&uuid);
                }
                _ => self.account_cards.get_mut(&uuid).unwrap().update(msg),
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
        column(self.account_cards.iter().map(|acc| {
            acc.1
                .view()
                .map(|msg| Message::AccountCardMessage(acc.0.clone(), msg))
        }))
        .spacing(10)
    }
}
