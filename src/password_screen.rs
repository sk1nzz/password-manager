mod account_card;

use std::collections::HashMap;

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Column, button, column, container, scrollable, space, stack};
use iced::{Element, Length};
use rusqlite::Connection;
use uuid::Uuid;

use crate::forms::LoginType;
use crate::forms::new_account_form::{self, NewAccountForm};
use crate::models::{Account, Login};
use crate::password_screen::account_card::AccountCard;

#[derive(Default)]
pub struct PasswordScreen {
    account_cards: HashMap<Uuid, AccountCard>,
    new_account_form: NewAccountForm,
    new_account_form_opened: bool,
}

#[derive(Clone)]
pub enum Message {
    OpenNewAccount,
    LoadAccounts,
    NewAccountFormMessage(new_account_form::Message),
    AccountCardMessage(Uuid, account_card::Message),
}

impl PasswordScreen {
    pub fn update(&mut self, msg: Message, db: &Connection) {
        match msg {
            Message::OpenNewAccount => self.new_account_form_opened = true,
            Message::LoadAccounts => {
                let passwords = Account::get_all(db);
                self.account_cards = passwords
                    .into_iter()
                    .map(|i| (i.id, AccountCard::new(i)))
                    .collect();
            }
            Message::NewAccountFormMessage(msg) => match msg {
                new_account_form::Message::Cancel => {
                    self.new_account_form_opened = false;
                    self.new_account_form = NewAccountForm::default();
                }
                new_account_form::Message::Submit => {
                    let form = std::mem::take(&mut self.new_account_form);

                    let login = match form.login_type {
                        LoginType::Username => Login::Username(form.login),
                        LoginType::Email => Login::Email(form.login),
                    };

                    if let Some(modify_id) = form.modify_id {
                        let mod_acc = self.account_cards.get_mut(&modify_id).unwrap();
                        mod_acc.account.site_name = form.site_name;
                        mod_acc.account.login = login;
                        mod_acc.account.password = form.password;
                        mod_acc.account.save(db);
                    } else {
                        let acc = Account::new(form.site_name, login, form.password);
                        acc.save(db);
                        self.account_cards.insert(acc.id, AccountCard::new(acc));
                    }

                    self.new_account_form_opened = false;
                }
                _ => self.new_account_form.update(msg),
            },
            Message::AccountCardMessage(uuid, msg) => match msg {
                account_card::Message::DeleteAccount => {
                    Account::delete(db, uuid);
                    self.account_cards.remove(&uuid);
                }
                account_card::Message::ModifyAccount => {
                    self.new_account_form.modify_id = Some(uuid);
                    let acc = &self.account_cards.get(&uuid).unwrap().account;
                    self.new_account_form.site_name = acc.site_name.clone();
                    match &acc.login {
                        Login::Email(email) => {
                            self.new_account_form.login_type = LoginType::Email;
                            self.new_account_form.login = email.clone();
                        }
                        Login::Username(username) => {
                            self.new_account_form.login_type = LoginType::Username;
                            self.new_account_form.login = username.clone();
                        }
                    };
                    self.new_account_form.password = acc.password.clone();
                    self.new_account_form_opened = true;
                }
                _ => self.account_cards.get_mut(&uuid).unwrap().update(msg),
            },
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        stack![
            scrollable(self.view_accounts()),
            container(button("Новый").on_press(Message::OpenNewAccount))
                .align_x(Horizontal::Right)
                .align_y(Vertical::Bottom)
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(10),
            container(self.view_new_account_form())
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .width(Length::Fill)
                .height(Length::Fill),
        ]
        .width(Length::Fill)
        .height(Length::Fill)
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
