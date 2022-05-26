mod process;

use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::*;


declare_id!("FhWUGomvmZfN2G2tyhEiaYgjCePg5gF6GC6GA3ccrVRc");

#[program]
pub mod contract {
    use anchor_lang::solana_program::{
        entrypoint::ProgramResult,
        system_instruction,
    };
    use anchor_lang::solana_program::program::invoke;
    use spl_token;

    use super::*;

    pub fn reward(ctx: Context<TransferRewardToken>, lamports: u64) -> ProgramResult {
        msg!("Signer info {:#?}", ctx.accounts.signer.to_account_info());
        invoke(&system_instruction::transfer(ctx.accounts.signer.key,
                                                    ctx.accounts.receiver.key, lamports),
                      &[
                          ctx.accounts.signer.to_account_info(),
                          ctx.accounts.system_program.to_account_info(),
                          ctx.accounts.receiver.to_account_info()
                      ])
    }

    pub fn transfer_spl_token(ctx: Context<TransferSPLToken>, amount: u64) -> ProgramResult {
        let result = spl_token::instruction::transfer(
            ctx.accounts.token_program.key,
            ctx.accounts.source_account.key,
            ctx.accounts.receiver_account.key,
            ctx.accounts.authority.key,
            &[],
            amount
        );

        return match result {
            Ok(instruction) => {
                spl_token::solana_program::program::invoke_signed(&instruction,
                                                                  &[
                                                                      ctx.accounts.source_account.to_account_info(),
                                                                      ctx.accounts.authority.to_account_info(),
                                                                      ctx.accounts.receiver_account.to_account_info(),
                                                                  ],
                                                                  &[])
            }

            Err(error) => {
                Err(error)
            }
        }
    }

    pub fn transfer_spl_token2(ctx: Context<TransferSPLToken>, amount: u64) -> Result<()> {
        process::transfer_spl_token(ctx.accounts.source_account.to_account_info(),
                                    ctx.accounts.receiver_account.to_account_info(),
                                    ctx.accounts.authority.to_account_info(),
                                    ctx.accounts.token_program.to_account_info(),
                                    amount)
    }

    pub fn create_associated_token_account(ctx: Context<CreateAssociatedTokenAccount>) -> Result<()> {
        msg!("Created associated token account: {:#?}", ctx.accounts.token.to_account_info().key);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferRewardToken<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub receiver: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferSPLToken<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub source_account: AccountInfo<'info>, // Associated token account
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub receiver_account: AccountInfo<'info>, // Associated token account
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CreateAssociatedTokenAccount<'info> {
    #[account(
    init_if_needed,
    payer = payer,
    associated_token::mint = mint,
    associated_token::authority = owner,
    )]
    pub token: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: unsafe
    pub owner: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}