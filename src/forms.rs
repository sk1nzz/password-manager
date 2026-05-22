pub mod new_account_form;
pub mod new_totp_form;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum LoginType {
    #[default]
    Username,
    Email,
}
