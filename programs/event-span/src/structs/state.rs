use anchor_lang::prelude::*;

#[account(zero_copy)]
#[repr(packed)]
#[derive(PartialEq, Default, Debug)]
pub struct State {
    pub eventsnap_admin: Pubkey,
    pub nonce: u8,
    pub program_authority: Pubkey,
    pub bump: u8,
}
