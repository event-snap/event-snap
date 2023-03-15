use crate::structs::state::State;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

pub mod interfaces;
mod macros;
pub mod structs;
pub mod utiles;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
// TODO: change type from &str to &[u8]
const AUTHORITY_SEED: &str = "EVENTSNAP";
const STATE_SEED: &str = "STATE";
const EVNET_BUFFER: &str = "EVENT_BUFFER";
const MOCKED_EVENT_SEED: &str = "MOCKED_EVENT";

#[program]
pub mod event_span {
    use std::cell::RefMut;

    use anchor_lang::solana_program::system_instruction;

    use crate::structs::EventStruct;

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

    pub fn withdraw_event_buffer(ctx: Context<WithdrawEventBuffer>, amount: u64) -> Result<()> {
        let state = ctx.accounts.state.load()?;
        let signer: &[&[&[u8]]] = get_signer!(EVNET_BUFFER, state.event_buffer_bump);

        let ix = system_instruction::transfer(
            &ctx.accounts.event_buffer.key(),
            &ctx.accounts.admin.key(),
            amount,
        );
        // TODO: check accounts infos are required
        anchor_lang::solana_program::program::invoke_signed(
            &ix,
            &[
                ctx.accounts.event_buffer.to_account_info(),
                ctx.accounts.admin.to_account_info(),
            ],
            signer,
        )?;

        Ok(())
    }

    pub fn trigger_events_creation(ctx: Context<TriggerEventsCreation>) -> Result<()> {
        let state = ctx.accounts.state.load()?;

        let (_, bump) =
            Pubkey::find_program_address(&[MOCKED_EVENT_SEED.as_bytes()], ctx.program_id);
        let signers: &[&[&[u8]]] = &[
            &[EVNET_BUFFER.as_bytes(), &[state.event_buffer_bump]],
            &[MOCKED_EVENT_SEED.as_bytes(), &[bump]],
        ];
        let space: usize = EventStruct::LEN;
        let lamports = Rent::get()?.minimum_balance(space);

        let cpi_accounts = anchor_lang::system_program::CreateAccount {
            from: ctx.accounts.event_buffer.to_account_info(),
            to: ctx.accounts.event_address.to_account_info(),
        };
        let cpi_context = anchor_lang::context::CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            cpi_accounts,
        );

        anchor_lang::system_program::create_account(
            cpi_context.with_signer(signers),
            lamports.clone(),
            space.try_into().unwrap(),
            ctx.program_id,
        )?;
        let event: &mut RefMut<EventStruct> =
            &mut utiles::deserialize_account(&ctx.accounts.event_address)?;
        **event = EventStruct {
            invoker: ctx.accounts.signer.key(),
            payer: ctx.accounts.event_buffer.key(),
            timestamp: Clock::get().unwrap().unix_timestamp,
            bump,
        };

        Ok(())
    }
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
        mut,
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

#[derive(Accounts)]
#[instruction( amount: u64)]
pub struct WithdrawEventBuffer<'info> {
    #[account(seeds = [STATE_SEED.as_bytes().as_ref()], bump = state.load()?.bump)]
    pub state: AccountLoader<'info, State>,
    #[account(mut, constraint = admin.key() == state.load()?.eventsnap_admin)] // TODO: admin error
    pub admin: Signer<'info>,
    /// CHECK: safe as seed checked
    #[account(mut, seeds = [EVNET_BUFFER.as_bytes()], bump = state.load()?.event_buffer_bump)]
    pub event_buffer: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: safe as constant
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,

    /// CHECK: safe as seed checked
    #[account(mut, seeds = [AUTHORITY_SEED.as_bytes().as_ref()], bump = state.load()?.nonce)]
    pub program_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct TriggerEventsCreation<'info> {
    /// CHECK: safe as constant
    #[account(mut)]
    pub event_address: AccountInfo<'info>,

    #[account(seeds = [STATE_SEED.as_bytes().as_ref()], bump = state.load()?.bump)]
    pub state: AccountLoader<'info, State>,
    /// CHECK: safe as seed checked
    #[account(mut, seeds = [EVNET_BUFFER.as_bytes().as_ref()], bump = state.load()?.event_buffer_bump)]
    pub event_buffer: AccountInfo<'info>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: safe as constant
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
}
