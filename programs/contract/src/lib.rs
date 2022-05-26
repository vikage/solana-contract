mod process;

use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::*;


declare_id!("FhWUGomvmZfN2G2tyhEiaYgjCePg5gF6GC6GA3ccrVRc");

#[program]
pub mod contract {
    use anchor_lang::solana_program::{
        entrypoint::ProgramResult,
        program::invoke_signed,
        system_instruction,
    };
    use anchor_lang::solana_program::program::invoke;
    use anchor_spl::token;
    use spl_token;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
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

        return token::transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(), transfer_ctx), amount)
    }
}

#[derive(Accounts)]
pub struct Initialize {}

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