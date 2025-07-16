use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub seed: u64,                  // Unique identifier of each AMM pool, Helps in deriving unique PDAs for each AMM instance.
    pub authority: Option<Pubkey>,  // can control parameters like Fees or make inactive of AMM. 
    pub mint_x: Pubkey,             //SPL token mint address for token X
    pub mint_y: Pubkey,             //SPL token mint address for token y
    pub fees: u16,                  // as name suggests, cut/fee charged per trade 
    pub locked: bool,               // bool expration- option to lock/unlock AMM 
    pub config_bump: u8,            // Bump used to derive the pda for this config acc.
    pub lp_bump: u8,                // bump used to derive the pda for this LP token mint acc.

}