use anchor_lang::prelude::*;

use crate::size;

#[account(zero_copy)]
#[repr(packed)]
#[derive(PartialEq, Default, Debug)]
pub struct EventStruct {
    pub invoker: Pubkey,
    pub payer: Pubkey,
    pub timestamp: i64,
    pub bump: u8,
}
size!(EventStruct);
