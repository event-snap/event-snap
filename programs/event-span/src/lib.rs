use crate::structs::state::State;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

mod macros;
pub mod structs;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
const AUTHORITY_SEED: &str = "EVENTSNAP";
const STATE_SEED: &str = "STATE";

#[program]
pub mod event_span {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, nonce: u8) -> Result<()> {
        let state = &mut ctx.accounts.state.load_init()?;
        **state = State {
            nonce,
            bump: *ctx.bumps.get("state").unwrap(),
            program_authority: *ctx.accounts.program_authority.key,
            eventsnap_admin: *ctx.accounts.admin.key,
            // event_buffer
        };

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction( nonce: u8)]
pub struct Initialize<'info> {
    #[account(init, seeds = [STATE_SEED.as_bytes().as_ref()], bump, space = State::LEN, payer = admin)]
    pub state: AccountLoader<'info, State>,
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: safe as seed checked
    #[account(seeds = [AUTHORITY_SEED.as_bytes().as_ref()], bump = nonce)]
    pub program_authority: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: safe as constant
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
}
