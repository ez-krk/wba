use anchor_lang::prelude::*;

mod constants;
mod contexts;
mod errors;
mod state;

use contexts::*;

declare_id!("SMBKnshAzPi9WZQEUyRxsm6PYZbyZEmxbxbaoMWNHkA");

#[program]
pub mod whitehat {
    use super::*;

    // register a protocol, set a name and % paid to hackers
    pub fn register_protocol(
        ctx: Context<RegisterProtocol>,
        name: String,
        percent: u64,
    ) -> Result<()> {
        ctx.accounts.register_protocol(&ctx.bumps, name, percent)
    }

    // TODO: do we delete this, if we don't want to doxx hackers
    pub fn register_hacker(ctx: Context<RegisterHacker>, name: String) -> Result<()> {
        ctx.accounts.register_hacker(&ctx.bumps, name)
    }

    // TODO: do we delete this, if we don't want to doxx hackers
    pub fn report_doxx_vulnerability(
        ctx: Context<ReportDoxxVulnerability>,
        message: Vec<u8>,
        seed: u64,
    ) -> Result<()> {
        ctx.accounts
            .report_doxx_vulnerability(&ctx.bumps, message, seed)
    }

    // vulnerability report, text ecies encrypted off-chain for protocol pubkey
    pub fn report_anon_vulnerability(
        ctx: Context<ReportAnonVulnerability>,
        message: Vec<u8>,
        seed: u64,
    ) -> Result<()> {
        ctx.accounts
            .report_anon_vulnerability(&ctx.bumps, message, seed)
    }

    // deposit from signer to protocol vault, passes a `hacker` pda, hacker input payout adress through instruction accounts
    pub fn new_doxx_sol_hack(ctx: Context<NewDoxxSolHack>, amount: u64, seed: u64) -> Result<()> {
        ctx.accounts.new_doxx_sol_hack(&ctx.bumps, amount, seed)
    }

    // deposit from signer to protocol vault anonymously, hacker input payout adress through instruction accounts
    pub fn new_anon_sol_hack(ctx: Context<NewAnonSolHack>, amount: u64, seed: u64) -> Result<()> {
        ctx.accounts.new_anon_sol_hack(&ctx.bumps, amount, seed)
    }

    // (ONLY PROTOCOL OWNER) pay the hacker to inputed payout address for % set by protocol 
    pub fn approve_doxx_sol_hack(ctx: Context<ApproveDoxxSolHack>) -> Result<()> {
        ctx.accounts.approve_doxx_sol_hack()
    }

    // (ONLY PROTOCOL OWNER) pay the hacker to inputed payout address for % set by protocol 
    pub fn approve_anon_sol_hack(ctx: Context<ApproveAnonSolHack>) -> Result<()> {
        ctx.accounts.approve_anon_sol_hack()
    }
}
