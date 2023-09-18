use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("Protocol Name Empty.")]
    ProtocolNameEmpty,
    #[msg("Protocol Name Too Long, 50 Characters Maximum.")]
    ProtocolNameTooLong,
    #[msg("Hacker Name Empty.")]
    HackerNameEmpty,
    #[msg("Hacker Name Too Long, 50 Characters Maximum.")]
    HackerNameTooLong,
    #[msg("GPG Key Empty.")]
    GPGKeyEmpty,
    #[msg("GPG Key Too Small, 2048 Characters Min.")]
    GPGKeyTooSmall,
    #[msg("GPG Key Too Big, 4096 characters maximum.")]
    GPGKeyTooBig,
    #[msg("Message empty.")]
    MessageEmpty,
}
