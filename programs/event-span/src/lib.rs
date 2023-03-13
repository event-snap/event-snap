use crate::structs::state::State;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use structs::EventStruct;

pub mod interfaces;
mod macros;
pub mod structs;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
// TODO: change type from &str to &[u8]
const AUTHORITY_SEED: &str = "EVENTSNAP";
const STATE_SEED: &str = "STATE";
const EVNET_BUFFER: &str = "EVENT_BUFFER";
const MOCKED_EVENT_SEED: &str = "MOCKED_EVENT";

#[program]
pub mod event_span {
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

    pub fn trigger_events_creation_two(ctx: Context<TriggerEventsCreationTwo>) -> Result<()> {
        let event_address = &mut ctx.accounts.event_address.load_init()?;

        **event_address = EventStruct {
            bump: *ctx.bumps.get("event_address").unwrap(),
            invoker: ctx.accounts.signer.key(),
            payer: ctx.accounts.signer.key(),
            timestamp: Clock::get().unwrap().unix_timestamp,
        };

        // let signer: &[&[&[u8]]] = get_signer!(EVNET_BUFFER, state.event_buffer_bump);
        // let lamports = Rent::get()?.minimum_balance(EventStruct::LEN);
        // let space: u64 = EventStruct::LEN.try_into().unwrap();
        // let event_payer = ctx.accounts.event_buffer.key();

        // let cpi_accounts = CreateAccount {
        //     from: ctx.accounts.event_buffer.to_account_info(),
        //     to: ctx.accounts.event_address.to_account_info(),
        // };

        // let _cpi_context = anchor_lang::context::CpiContext::new(
        //     ctx.accounts.system_program.to_account_info(),
        //     cpi_accounts,
        // );

        // anchor_lang::system_program::create_account(
        //     cpi_context.with_signer(signer),
        //     lamports.clone(),
        //     space,
        //     &event_payer,
        // )?;

        Ok(())
    }

    // pub fn trigger_events_creation(
    //     ctx: Context<TriggerEventsCreation>,
    //     event_seed: u64,
    // ) -> Result<()> {
    //     let seed: [u8; 8] = event_seed.to_le_bytes();
    //     // TODO: in future SEED of account is [tx + EventTypeName]
    //     let state = ctx.accounts.state.load()?;
    //     let (event_address, bump) = Pubkey::find_program_address(&[&seed], ctx.program_id);
    //     let signer: &[&[&[u8]]] = get_signer!(EVNET_BUFFER, state.event_buffer_bump);
    //     let timestamp = Clock::get().unwrap().unix_timestamp;
    //     let event_payer = ctx.accounts.event_buffer.key();

    //     let event_strcut = EventStruct {
    //         bump,
    //         timestamp,
    //         invoker: ctx.accounts.singer.key(),
    //         payer: event_payer,
    //     };

    //     let mut lamports = Rent::get()?.minimum_balance(EventStruct::LEN);
    //     let space: u64 = EventStruct::LEN.try_into().unwrap();

    //     // it is also possible to call [solana_sdk::system_instruction::create_account]
    //     // maybe switch ix to direct call

    //     ///////////////////NEW_SOLUTION/////////////////////
    //     // let mut dummpy_data = b"data".to_vec();
    //     // let event_account_info = AccountInfo::new(
    //     //     &event_address,
    //     //     true,
    //     //     true,
    //     //     &mut lamports,
    //     //     &mut dummpy_data,
    //     //     &event_payer,
    //     //     false,
    //     //     anchor_lang::solana_program::stake_history::Epoch::default(),
    //     // );

    //     // let cpi_accounts = CreateAccount {
    //     //     from: ctx.accounts.event_buffer.to_account_info(),
    //     //     // to: event_account_info.clone(),
    //     //     to: event_account_info,
    //     // };

    //     // // system_program account info instead of passing via context

    //     // let cpi_context = anchor_lang::context::CpiContext::new(
    //     //     ctx.accounts.system_program.to_account_info(),
    //     //     cpi_accounts,
    //     // );
    //     // anchor_lang::system_program::create_account(
    //     //     cpi_context.with_signer(signer),
    //     //     lamports.clone(),
    //     //     space,
    //     //     &event_payer,
    //     // )?;
    //     ///////////////////////////////////////////////

    //     ///////////////////////////////////////////////

    //     // ctx: CpiContext<'a, 'b, 'c, 'info, CreateAccount<'info>>,

    //     // let create_account_ctx =
    //     //     anchor_lang::system_program::create_account(ctx, lamports, space, &event_payer)?;

    //     //
    //     // let ix = solana_sdk::system_instruction::create_account(
    //     //     &event_payer,
    //     //     &event_address,
    //     //     lamports,
    //     //     space,
    //     //     &event_payer,
    //     // );
    //     // anchor_lang::solana_program::program::invoke_signed(
    //     //     &ix,
    //     //     &[
    //     //         ctx.accounts.event_buffer.to_account_info(),
    //     //         ctx.accounts.admin.to_account_info(),
    //     //     ],
    //     //     signer,
    //     // )?;

    //     // solana_sdk::system_instruction::create_account_with_seed(&event_payer, &event_address, base, seed, lamports, space, owner)

    //     // EventStruct::LEN

    //     // solana_sdk::system_instruction::create_account_with_seed(from_pubkey, to_pubkey, base, seed, lamports, space, owner)

    //     // solana_sdk::system_instruction::create_account()

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
    // #[account(init, seeds = [STATE_SEED.as_bytes().as_ref()], bump, space = State::LEN, payer = admin)]
    // pub state: AccountLoader<'info, State>,
    // #[account(mut)]
    // pub admin: Signer<'info>,
    // /// CHECK: safe as seed checked
    // #[account(seeds = [AUTHORITY_SEED.as_bytes().as_ref()], bump = nonce)]
    // pub program_authority: AccountInfo<'info>,

    // // #[account(init,
    // //     token::authority = authority,
    // //     payer = payer,
    // // )]
    // // pub event_buffer: Account<'info, >,
    // pub rent: Sysvar<'info, Rent>,
    // /// CHECK: safe as constant
    // #[account(address = system_program::ID)]
    // pub system_program: AccountInfo<'info>,
}

// #[derive(Accounts)]
// #[instruction( event_seed: u64)]
// pub struct TriggerEventsCreation<'info> {
//     #[account(seeds = [STATE_SEED.as_bytes().as_ref()], bump = state.load()?.bump)]
//     pub state: AccountLoader<'info, State>,
//     /// CHECK: safe as seed checked
//     #[account(mut, seeds = [EVNET_BUFFER.as_bytes()], bump = state.load()?.event_buffer_bump)]
//     pub event_buffer: UncheckedAccount<'info>,
//     #[account(mut)]
//     pub singer: Signer<'info>,
//     /// CHECK: safe as constant
//     #[account(address = system_program::ID)]
//     pub system_program: AccountInfo<'info>,
// }

#[derive(Accounts)]
pub struct TriggerEventsCreationTwo<'info> {
    #[account(init, seeds = [MOCKED_EVENT_SEED.as_bytes().as_ref()], bump, space = EventStruct::LEN, payer = signer)]
    pub event_address: AccountLoader<'info, EventStruct>,
    // #[account(seeds = [STATE_SEED.as_bytes().as_ref()], bump = state.load()?.bump)]
    // pub state: AccountLoader<'info, State>,
    // /// CHECK: safe as seed checked
    // #[account(mut, seeds = [AUTHORITY_SEED.as_bytes().as_ref()], bump = state.load()?.nonce)]
    // pub program_authority: AccountInfo<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: safe as constant
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
}

// impl<'info> CreateEvent<'info> for TriggerEventsCreation<'info> {
//     fn create_event<'a>(
//         &self,
//         event_address: &'a Pubkey,
//         // data: Vec<u8>,
//     ) -> CpiContext<'_, '_, '_, 'info, CreateAccount<'info>> {
//         let mut lamports = Rent::get().unwrap().minimum_balance(EventStruct::LEN);
//         let event_payer = self.event_buffer.key();
//         let mut dummpy_data = b"data".to_vec();
//         let data = &mut b"data";
//         let test_address = Pubkey::new_unique();

//         let event_account_info = AccountInfo::new(
//             &'a event_address,
//             true,
//             true,
//             &mut lamports,
//             &mut dummpy_data,
//             &event_payer,
//             false,
//             anchor_lang::solana_program::stake_history::Epoch::default(),
//         );

//         let accounts = CreateAccount {
//             from: self.event_buffer.to_account_info(),
//             to: event_account_info,
//         };

//         panic!("txt");
//         // CpiContext::new(self.system_program.to_account_info(), accounts)
//     }
// }

// impl<'info> CreateEvent<'info> for TriggerEventsCreation<'info> {
//     fn create_event(
//         &self,
//         event_address: Pubkey,
//     ) -> CpiContext<'_, '_, '_, 'info, CreateAccount<'info>> {
//         let mut lamports = Rent::get().unwrap().minimum_balance(EventStruct::LEN);
//         let space: u64 = EventStruct::LEN.try_into().unwrap();
//         let mut dummpy_data = b"data".to_vec();
//         let event_payer = self.event_buffer.key();

//         let event_account_info = AccountInfo::new(
//             &event_address,
//             true,
//             true,
//             &mut lamports,
//             &mut dummpy_data,
//             &event_payer,
//             false,
//             anchor_lang::solana_program::stake_history::Epoch::default(),
//         );

//         let accounts = CreateAccount {
//             from: self.event_buffer.to_account_info(),
//             to: event_account_info,
//         };

//         CpiContext::new(self.system_program.to_account_info(), accounts)
//     }
// }
