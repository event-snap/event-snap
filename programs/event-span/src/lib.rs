use anchor_lang::prelude::*;
use structs::EventBuffer;

pub mod interfaces;
mod macros;
pub mod structs;
pub mod utiles;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

const EVENT_BUFFER_SEED: &[u8] = b"EVENT_BUFFER";
const EVENT_AUTHORITY_SEED: &[u8] = b"EVENTSNAP";
const MOCKED_EVENT_SEED: &[u8] = b"MOCKED_EVENT";

#[program]
pub mod event_span {
    use std::cell::RefMut;

    use anchor_lang::solana_program::system_instruction;

    use crate::structs::EventStruct;

    use super::*;

    pub fn init_event_buffer(ctx: Context<InitEventBuffer>) -> Result<()> {
        let event_buffer = &mut ctx.accounts.event_buffer.load_init()?;
        let event_authority = *ctx.accounts.event_authority.to_account_info().key;

        **event_buffer = EventBuffer {
            event_authority,
            admin: ctx.accounts.admin.key(),
            nonce: *ctx.bumps.get("event_authority").unwrap(),
            bump: *ctx.bumps.get("event_buffer").unwrap(),
        };

        Ok(())
    }

    pub fn deposit_event_buffer(ctx: Context<DepositEventBuffer>, amount: u64) -> Result<()> {
        let ix = system_instruction::transfer(
            &ctx.accounts.depositor.key(),
            &ctx.accounts.event_authority.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.depositor.to_account_info(),
                ctx.accounts.event_authority.to_account_info(),
            ],
        )?;

        Ok(())
    }

    pub fn withdraw_event_buffer(ctx: Context<WithdrawEventBuffer>, amount: u64) -> Result<()> {
        let event_buffer = ctx.accounts.event_buffer.load()?;
        let signer: &[&[&[u8]]] = get_signer!(EVENT_AUTHORITY_SEED, event_buffer.nonce);

        let ix = system_instruction::transfer(
            &ctx.accounts.event_authority.key(),
            &ctx.accounts.admin.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke_signed(
            &ix,
            &[
                ctx.accounts.event_authority.to_account_info(),
                ctx.accounts.admin.to_account_info(),
            ],
            signer,
        )?;

        Ok(())
    }

    pub fn trigger_events_creation(ctx: Context<TriggerEventsCreation>, emit: bool) -> Result<()> {
        if emit {
            let event_buffer = ctx.accounts.event_buffer.load()?;

            let (_, bump) = Pubkey::find_program_address(&[MOCKED_EVENT_SEED], ctx.program_id);
            let signers: &[&[&[u8]]] = &[
                &[EVENT_AUTHORITY_SEED, &[event_buffer.nonce]],
                &[MOCKED_EVENT_SEED, &[bump]],
            ];
            let space: usize = EventStruct::LEN;
            let lamports = Rent::get()?.minimum_balance(space);

            let cpi_accounts = anchor_lang::system_program::CreateAccount {
                from: ctx.accounts.event_authority.to_account_info(),
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
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitEventBuffer<'info> {
    #[account(init, seeds = [EVENT_BUFFER_SEED.as_ref()], bump, space = EventBuffer::LEN, payer = admin)]
    pub event_buffer: AccountLoader<'info, EventBuffer>,
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: determinstic authority
    #[account(
        mut,
        seeds = [EVENT_AUTHORITY_SEED],
        bump,
    )]
    pub event_authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction( amount: u64)]
pub struct DepositEventBuffer<'info> {
    #[account(seeds = [EVENT_BUFFER_SEED.as_ref()], bump = event_buffer.load()?.bump)]
    pub event_buffer: AccountLoader<'info, EventBuffer>,
    #[account(mut)]
    pub depositor: Signer<'info>,
    /// CHECK: determinstic authority
    #[account(mut, seeds = [EVENT_AUTHORITY_SEED], bump = event_buffer.load()?.nonce)]
    pub event_authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction( amount: u64)]
pub struct WithdrawEventBuffer<'info> {
    #[account(seeds = [EVENT_BUFFER_SEED.as_ref()], bump = event_buffer.load()?.bump)]
    pub event_buffer: AccountLoader<'info, EventBuffer>,
    #[account(mut, constraint = admin.key() == event_buffer.load()?.admin)]
    pub admin: Signer<'info>,
    /// CHECK: determinstic authority
    #[account(mut, seeds = [EVENT_AUTHORITY_SEED], bump = event_buffer.load()?.nonce)]
    pub event_authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction( emit: bool)]
pub struct TriggerEventsCreation<'info> {
    /// CHECK: event account
    #[account(mut)]
    pub event_address: AccountInfo<'info>,
    #[account(seeds = [EVENT_BUFFER_SEED.as_ref()], bump = event_buffer.load()?.bump)]
    pub event_buffer: AccountLoader<'info, EventBuffer>,
    /// CHECK: determinstic authority
    #[account(mut, seeds = [EVENT_AUTHORITY_SEED], bump = event_buffer.load()?.nonce)]
    pub event_authority: AccountInfo<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
