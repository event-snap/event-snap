use anchor_lang::prelude::*;

use crate::size;

#[account(zero_copy)]
#[repr(packed)]
#[derive(PartialEq, Default, Debug)]
pub struct EventBuffer {
    pub event_authority: Pubkey,
    pub admin: Pubkey,
    pub nonce: u8,
    pub bump: u8,
}
size!(EventBuffer);
