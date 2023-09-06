use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("The provided name should be 50 characters long maximum")]
    NameTooLong,
    #[msg("The provided content should be 280 characters long maximum")]
    ContentTooLong,
}