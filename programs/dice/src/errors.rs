use anchor_lang::error_code;

#[error_code]
pub enum DiceError {
    #[msg("Bump Error !")]
    BumpError,
    #[msg("Overflow !")]
    Overflow,
    #[msg("Minimum bet is 0.01 Sol !")]
    MinimumBet,
    #[msg("Maximum exceeded !")]
    MaximumBet,
    #[msg("Minimum roll is 2 !")]
    MinimumRoll,
    #[msg("Ed25119 Header Error !")]
    Ed25119Header,
    #[msg("Ed25119 Pubkey Error !")]
    Ed25119Pubkey,
    #[msg("Ed25119 Message Error !")]
    Ed25119Message,
    #[msg("Ed25119 Signature Error !")]
    Ed25119Signature,
    #[msg("Ed25119 Program Error !")]
    Ed25119Program,
    #[msg("Ed25119 Accounts Error !")]
    Ed25119Accounts,
    #[msg("Ed25119 Data Length Error !")]
    Ed25119DataLength,
}
