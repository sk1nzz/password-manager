mod account_card;
mod new_account_form;

use std::collections::HashMap;

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Column, button, column, container, scrollable, space, stack};
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
                new_account_form::Message::Cancel => {
                    self.new_account_form_opened = false;
                    self.new_account_form = NewAccountForm::default();
                }
                new_account_form::Message::Submit => {
                    let login = match self.new_account_form.login_type {
                        new_account_form::LoginType::Username => {
                            Login::Username(self.new_account_form.login.clone())
                        }
                        new_account_form::LoginType::Email => {
                            Login::Email(self.new_account_form.login.clone())
                        }
                    };

                    if let Some(modify_id) = self.new_account_form.modify_id {
                        let mod_acc = self.account_cards.get_mut(&modify_id).unwrap();
                        mod_acc.account.site_name = self.new_account_form.site_name.clone();
                        mod_acc.account.login = login;
                        mod_acc.account.password = self.new_account_form.password.clone();
                    } else {
                        let acc = Account::new(
                            self.new_account_form.site_name.clone(),
                            login,
                            self.new_account_form.password.clone(),
                        );
                        self.account_cards.insert(acc.id, AccountCard::new(acc));
                    }

                    self.new_account_form_opened = false;
                    self.new_account_form = NewAccountForm::default();
                }
                _ => self.new_account_form.update(msg),
            },
            Message::AccountCardMessage(uuid, msg) => match msg {
                account_card::Message::DeleteAccount => {
                    self.account_cards.remove(&uuid);
                }
                account_card::Message::ModifyAccount => {
                    self.new_account_form.modify_id = Some(uuid);
                    let acc = &self.account_cards.get(&uuid).unwrap().account;
                    self.new_account_form.site_name = acc.site_name.clone();
                    match &acc.login {
                        Login::Email(email) => {
                            self.new_account_form.login_type = new_account_form::LoginType::Email;
                            self.new_account_form.login = email.clone();
                        }
                        Login::Username(username) => {
                            self.new_account_form.login_type =
                                new_account_form::LoginType::Username;
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
