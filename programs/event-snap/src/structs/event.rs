use anchor_lang::prelude::*;
use event_snap_lib::size;

#[account(zero_copy)]
#[derive(PartialEq, Default, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct EventStruct {
    pub invoker: Pubkey,
    pub payer: Pubkey,
    pub timestamp: i64,
    pub bump: u8,
}
size!(EventStruct);
