use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, 
    token_interface::{ Mint, TokenAccount, TransferChecked, TokenInterface, transfer_checked}
};


use crate::Escrow;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    // the source token account owner
    #[account(mut)]
    pub maker: Signer<'info>,

    // mint acc specifying the type of token to be sent
    #[account(
        mint::token_program = token_program,
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,

    // mint acc specifying the type of token to be received
    #[account(
        mint::token_program = token_program,
    )]
    pub mint_b: InterfaceAccount<'info, Mint>,

    // this is a token acc (ex:- an ATA)
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    // stores the metadata of the trade: maker, amount, mints, bumps, etc... 
    #[account(
        init,
        payer = maker,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
        space = 8 + Escrow::INIT_SPACE
    )]
    pub escrow: Account<'info, Escrow>,

    // this vault actually hold the tokens that are being ecsrowed (token A) 
    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    
    
    // Interface is an Anchor type that allows for "flexible" program constraints
    // "I need an account in the context that must be the SPL Token program, and it must implement the TokenInterface
    // (i.e., allow token instructions like transfer, mint_to, etc.)."
    // The token program that will process the transfer
    pub token_program: Interface<'info, TokenInterface>,

    // The Associated Token Program is a small helper program that: Derives ATA addresses and x Creates them for you
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Make<'info> {
    pub fn init_escrow(&mut self, seed: u64, bump: &MakeBumps, receive: u64) -> Result<()> {
        self.escrow.set_inner(Escrow { 
            seed, 
            maker: self.maker.key(), 
            mint_a: self.mint_a.key(), 
            mint_b: self.mint_b.key(), 
            receive, 
            bump: bump.escrow
        });
        
        Ok(())
    }

    pub fn deposit(&mut self, deposit: u64) -> Result<()> {
        // number of decimal for the mint
        let decimals = self.mint_a.decimals;

        // The program being invoked in the CPI
        let cpi_program = self.token_program.to_account_info();

        // create the TransferChecked struct with required accounts
        let cpi_accounts = TransferChecked{
            mint: self.mint_a.to_account_info(),
            from: self.maker_ata_a.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info()
        };

        // create the TransferChecked instuction
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, deposit, decimals)?;

        Ok(())
    }
}

