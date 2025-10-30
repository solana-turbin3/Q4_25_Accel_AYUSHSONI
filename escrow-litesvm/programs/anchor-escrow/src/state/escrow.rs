use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct Escrow {
    pub seed: u64,
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub lock_period: i64,
    pub receive: u64,
    pub bump: u8,
}