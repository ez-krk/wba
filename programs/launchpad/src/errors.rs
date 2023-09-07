use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("Instruction Owner Only")]
    InstructionOwnerOnly,
    #[msg("Instruction Not For Owner")]
    InstructionNotForOwner,
}