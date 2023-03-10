use crate::structs::state::State;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

mod macros;
pub mod structs;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
// TODO: change type from &str to &[u8]
const AUTHORITY_SEED: &str = "EVENTSNAP";
const STATE_SEED: &str = "STATE";
const EVNET_BUFFER: &str = "EVENT_BUFFER";

#[program]
pub mod event_span {
    use anchor_lang::solana_program::system_instruction;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, nonce: u8) -> Result<()> {
        // initialize state and initalize event buffer
        let state = &mut ctx.accounts.state.load_init()?;
        let event_buffer = *ctx.accounts.event_buffer.to_account_info().key;

        **state = State {
            nonce,
            bump: *ctx.bumps.get("state").unwrap(),
            program_authority: *ctx.accounts.program_authority.key,
            eventsnap_admin: *ctx.accounts.admin.key,
            event_buffer,
            event_buffer_bump: *ctx.bumps.get("event_buffer").unwrap(),
        };

        Ok(())
    }

    pub fn deposit_event_buffer(ctx: Context<DepositEventBuffer>, amount: u64) -> Result<()> {
        let ix = system_instruction::transfer(
            &ctx.accounts.depositor.key(),
            &ctx.accounts.event_buffer.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.depositor.to_account_info(),
                ctx.accounts.event_buffer.to_account_info(),
            ],
        )?;

        Ok(())
    }

    // pub fn withdraw_event_buffer(ctx: Context<WithdrawEventBuffer>) -> Result<()> {
    //     Ok(())
    // }
}

// TODO: rent in not required
// TODO: passing program_authority is not required
// TODO: I can use Account with empty strcut insted of UncheckedAccount
// TODO: Remove "CHECK:" from system_program

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
    /// CHECK: safe as seed checked
    #[account(
        init,
        payer = admin,
        space = 0,
        seeds = [EVNET_BUFFER.as_bytes()],
        bump,
    )]
    pub event_buffer: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: safe as constant
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction( amount: u64)]
pub struct DepositEventBuffer<'info> {
    #[account(seeds = [STATE_SEED.as_bytes().as_ref()], bump = state.load()?.bump)]
    pub state: AccountLoader<'info, State>,
    #[account(mut)]
    pub depositor: Signer<'info>,
    /// CHECK: safe as seed checked
    #[account(mut, seeds = [EVNET_BUFFER.as_bytes()], bump = state.load()?.event_buffer_bump)]
    pub event_buffer: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: safe as constant
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
}

// #[derive(Accounts)]
// pub struct WithdrawEventBuffer<'info> {
//     // #[account(init, seeds = [STATE_SEED.as_bytes().as_ref()], bump, space = State::LEN, payer = admin)]
//     // pub state: AccountLoader<'info, State>,
//     // #[account(mut)]
//     // pub admin: Signer<'info>,
//     // /// CHECK: safe as seed checked
//     // #[account(seeds = [AUTHORITY_SEED.as_bytes().as_ref()], bump = nonce)]
//     // pub program_authority: AccountInfo<'info>,

//     // // #[account(init,
//     // //     token::authority = authority,
//     // //     payer = payer,
//     // // )]
//     // // pub event_buffer: Account<'info, >,
//     // pub rent: Sysvar<'info, Rent>,
//     // /// CHECK: safe as constant
//     // #[account(address = system_program::ID)]
//     // pub system_program: AccountInfo<'info>,
// }
