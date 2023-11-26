use anchor_lang::error_code;

#[error_code]
pub enum MailError {
    #[msg("Bump Error")]
    BumpError,
    #[msg("The provided content should be 280 characters long maximum")]
    ContentTooLong,
    #[msg("The provided name should be 50 characters long maximum")]
    NameTooLong,
    #[msg("Message Too Long")]
    MessageTooLong,
    #[msg("GPG Key Too Big")]
    GPGKeyTooBig,
    #[msg("GPG Key Too Small")]
    GPGKeyTooSmall
}