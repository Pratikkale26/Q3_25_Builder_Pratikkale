use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    // User who wants to participate in staking
    #[account(mut)]
    pub user: Signer<'info>,

    // User's staking account PDA to track their staking data
    #[account(
        init,
        payer = user,                             
        seeds = [b"user", user.key.as_ref()],     
        bump,                                     
        space = 8 + UserAccount::INIT_SPACE,    
    )]
    pub user_account: Account<'info, UserAccount>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeUser<'info> {
    // Initialize user's staking account with default values
    pub fn initialize_user(&mut self, bumps: InitializeUserBumps) -> Result<()> {
        self.user_account.set_inner(UserAccount {
            points: 0,                             // Start with 0 points
            amount_staked: 0,                      // No NFTs staked initially
            bump: bumps.user_account,              // Store PDA bump
        });
        Ok(())
    }
}
