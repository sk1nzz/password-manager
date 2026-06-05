mod account_card;

use std::collections::HashMap;

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Column, button, center, column, scrollable, space, stack};
use iced::{Element, Length};
use rusqlite::Connection;
use uuid::Uuid;

use crate::forms::new_account_form::{self, FormMode, NewAccountForm};
use crate::models::account::{Account, AccountValidationError};
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
    NewAccountFormMessage(new_account_form::Message),
    AccountCardMessage(Uuid, account_card::Message),
}

impl PasswordScreen {
    pub fn new(db: &Connection) -> Self {
        let passwords = Account::get_all(db);
        Self {
            account_cards: passwords
                .into_iter()
                .map(|i| (i.id(), AccountCard::new(i)))
                .collect(),
            ..Default::default()
        }
    }

    pub fn update(&mut self, msg: Message, db: &Connection) {
        match msg {
            Message::OpenNewAccount => self.open_form(),
            Message::NewAccountFormMessage(msg) => match msg {
                new_account_form::Message::Cancel => self.close_form(),
                new_account_form::Message::Submit => self.handle_form_submit(db),
                _ => self.new_account_form.update(msg),
            },
            Message::AccountCardMessage(id, msg) => match msg {
                account_card::Message::DeleteAccount => self.handle_delete_acc(id, db),
                account_card::Message::ModifyAccount => self.handle_open_modify(id),
                _ => self.account_cards.get_mut(&id).unwrap().update(msg),
            },
        }
    }

    fn open_form(&mut self) {
        self.new_account_form_opened = true;
        self.new_account_form = NewAccountForm::default();
    }

    fn close_form(&mut self) {
        self.new_account_form_opened = false;
        self.new_account_form = NewAccountForm::default();
    }

    fn handle_form_submit(&mut self, db: &Connection) {
        match self.new_account_form.mode {
            FormMode::Create => {
                let res = self.new_account_form.create_account();
                match res {
                    Ok(acc) => match acc.save(db) {
                        Ok(_) => {
                            self.account_cards.insert(acc.id(), AccountCard::new(acc));
                            self.new_account_form_opened = false;
                        }
                        Err(_) => {
                            self.handle_save_error();
                        }
                    },
                    Err(e) => self.handle_validation_error(e),
                }
            }
            FormMode::Modify(id) => {
                let mod_acc = self.account_cards.get_mut(&id).unwrap();
                let res = self.new_account_form.modify_account(&mut mod_acc.account);
                match res {
                    Ok(_) => match mod_acc.account.save(db) {
                        Ok(_) => self.new_account_form_opened = false,
                        Err(_) => {
                            self.new_account_form.error =
                                Some("Уже есть аккаунт с таким названием сайта и логином")
                        }
                    },
                    Err(e) => self.handle_validation_error(e),
                }
            }
        }
    }

    fn handle_open_modify(&mut self, id: Uuid) {
        let acc = &self.account_cards.get(&id).unwrap().account;
        self.new_account_form.init_modify(acc);
        self.new_account_form_opened = true;
    }

    fn handle_delete_acc(&mut self, id: Uuid, db: &Connection) {
        self.account_cards
            .remove(&id)
            .unwrap()
            .account
            .delete(db)
            .unwrap();
    }

    fn handle_validation_error(&mut self, err: AccountValidationError) {
        match err {
            AccountValidationError::EmptySiteName => {
                self.new_account_form.error = Some("Пустое название сайта")
            }
            AccountValidationError::EmptyLogin => {
                self.new_account_form.error = Some("Пустой логин")
            }
            AccountValidationError::EmptyPassword => {
                self.new_account_form.error = Some("Пустой пароль")
            }
        }
    }

    fn handle_save_error(&mut self) {}

    pub fn view(&self) -> Element<'_, Message> {
        stack![
            scrollable(self.view_accounts()),
            center(button("Новый").on_press(Message::OpenNewAccount))
                .align_x(Horizontal::Right)
                .align_y(Vertical::Bottom)
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(10),
            center(self.view_new_account_form()),
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
