use anchor_lang::prelude::*;

use solana_program::{ed25519_program, instruction::Instruction};

use crate::errors::DiceError;

pub struct InstructionSysvar {}

impl anchor_lang::Id for InstructionSysvar {
    fn id() -> solana_program::pubkey::Pubkey {
        solana_program::sysvar::instructions::ID
    }
}

pub struct Ed25119InstructionHeader {
    pub num_signatures: u8,
    pub padding: u8,
    pub signature_offset: u16,
    pub pubkey_offset: u16,
    pub message_offset: u16,
    pub message_size: u16,
}

impl Ed25119InstructionHeader {
    pub fn new(message: &[u8]) -> Self {
        Ed25119InstructionHeader {
            num_signatures: 1,
            padding: 0,
            signature_offset: 48,
            pubkey_offset: 16,
            message_offset: 112,
            message_size: message.len() as u16,
        }
    }

    pub fn to_slice(&self) -> &[u8] {
        let mut s = vec![self.num_signatures, self.padding];
        s.extend_from_slice(&self.signature_offset.to_le_bytes());
        s.extend_from_slice(&[0xff, 0xff]);
        s.extend_from_slice(&self.pubkey_offset.to_le_bytes());
        s.extend_from_slice(&[0xff, 0xff]);
        s.extend_from_slice(&self.message_offset.to_le_bytes());
        s.extend_from_slice(&[0xff, 0xff]);
        s.extend_from_slice(&self.message_size.to_le_bytes());
        s.extend_from_slice(&[0xff, 0xff]);
        s.as_slice()
    }
}

// 16 bytes header
// 32 bytes pubkey
// 64 bytes signature
// 112
// however many bytes

pub fn ed25519_verify_ix(
    ix: &Instruction,
    pubkey: &[u8],
    message: &[u8],
    sig: &[u8],
) -> Result<()> {
    require_keys_eq!(
        ix.program_id,
        ed25519_program::ID,
        DiceError::Ed25119Program
    );
    require_eq!(ix.accounts.len(), 0, DiceError::Ed25119Accounts);
    require_eq!(
        ix.data.len(),
        16 + 64 + 32 + message.len(),
        DiceError::Ed25119DataLength
    );
    check_ed25519_data(&ix.data, pubkey, message, sig)
}

pub fn check_ed25519_data(data: &[u8], pubkey: &[u8], message: &[u8], sig: &[u8]) -> Result<()> {
    let header = Ed25119InstructionHeader::new(message).to_slice();
    require!(&data[0..16].eq(header), DiceError::Ed25119Header);

    let data_pubkey = &data[16..16 + 32];
    let data_sig = &data[48..48 + 64];
    let data_msg = &data[112..];

    require!(data_pubkey.eq(pubkey), DiceError::Ed25119Pubkey);
    require!(data_msg.eq(message), DiceError::Ed25119Message);
    require!(data_sig.eq(sig), DiceError::Ed25119Message);

    Ok(())
}
