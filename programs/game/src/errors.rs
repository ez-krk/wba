use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("Not enough health")]
    NotEnoughHealth,
    #[msg("Target Shielded")]
    TargetShielded,
    #[msg("Player Not Idle")]
    PlayerNotIdle,
    #[msg("Target Not Idle")]
    TargetNotIdle,
    #[msg("Out Of Bounds")]
    OutOfBounds,
    #[msg("Target Dead")]
    TargetDead,
    #[msg("Too Early")]
    TooEarly,
    #[msg("Already Claimed")]
    AlreadyClaimed,
}
