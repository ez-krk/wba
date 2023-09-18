use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("Protocol Name Empty.")]
    ProtocolNameEmpty,
    #[msg("Protocol Name Too Long, 50 Characters Maximum.")]
    ProtocolNameTooLong,
    #[msg("Answer too long, 280 characters maximum.")]
    AnswerTooLong,
    #[msg("Answer empty.")]
    AnswerEmpty,
    #[msg("Form Incompletey.")]
    FormIncomplete,
}
