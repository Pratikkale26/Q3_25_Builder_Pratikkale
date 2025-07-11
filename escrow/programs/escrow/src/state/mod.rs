use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub seed: u64,
    pub maker: Pubkey,
    pub mint_a: Pubkey, // token the make is offering
    pub mint_b: Pubkey, // token the maker wants
    pub receive: u64,   // amount of token B expected
    pub bump: u8,
}

