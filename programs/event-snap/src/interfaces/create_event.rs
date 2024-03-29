use anchor_lang::{prelude::*, system_program::CreateAccount};

pub trait CreateEvent<'info> {
    fn create_event(
        &self,
        event_address: &Pubkey,
    ) -> CpiContext<'_, '_, '_, 'info, CreateAccount<'info>>;
}
