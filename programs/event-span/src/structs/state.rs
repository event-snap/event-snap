use anchor_lang::prelude::*;

use crate::size;

// TODO: seperate buffer as seperate account
#[account(zero_copy)]
#[repr(packed)]
#[derive(PartialEq, Default, Debug)]
pub struct State {
    pub nonce: u8,
    pub bump: u8,
    pub program_authority: Pubkey,
    pub event_buffer: Pubkey,
    pub event_buffer_bump: u8,
    pub eventsnap_admin: Pubkey,
}
size!(State);
