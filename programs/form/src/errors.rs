use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("Question empty.")]
    QuestionEmpty,
    #[msg("Question too long, 50 characters maximum.")]
    QuestionTooLong,
    #[msg("Answer too long, 280 characters maximum.")]
    AnswerTooLong,
    #[msg("Answer empty.")]
    AnswerEmpty,
}
