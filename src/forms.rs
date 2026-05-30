pub mod lock_screen;
pub mod new_account_form;
pub mod new_totp_form;
pub mod welcome_screen;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum LoginType {
    #[default]
    Username,
    Email,
}
