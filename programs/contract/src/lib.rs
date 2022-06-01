mod process;

use anchor_lang::{
    prelude::*,
    solana_program::program::invoke,
    solana_program::{entrypoint::ProgramResult, system_instruction, system_program},
};
use anchor_spl::{associated_token, associated_token::AssociatedToken, token::*};
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v2};
use spl_token;

declare_id!("FUXDcaRXknWTgt5cPcKwidTss5ZNW1kVCGGRfb6EQVXN");

#[program]
pub mod contract {

    use super::*;

    pub fn reward(ctx: Context<TransferRewardToken>, lamports: u64) -> ProgramResult {
        msg!("Signer info {:#?}", ctx.accounts.signer.to_account_info());
        invoke(
            &system_instruction::transfer(
                ctx.accounts.signer.key,
                ctx.accounts.receiver.key,
                lamports,
            ),
            &[
                ctx.accounts.signer.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.receiver.to_account_info(),
            ],
        )
    }

    pub fn transfer_spl_token(ctx: Context<TransferSPLToken>, amount: u64) -> ProgramResult {
        let result = spl_token::instruction::transfer(
            ctx.accounts.token_program.key,
            ctx.accounts.source_account.key,
            ctx.accounts.receiver_account.key,
            ctx.accounts.authority.key,
            &[],
            amount,
        );

        return match result {
            Ok(instruction) => invoke(
                &instruction,
                &[
                    ctx.accounts.source_account.to_account_info(),
                    ctx.accounts.authority.to_account_info(),
                    ctx.accounts.receiver_account.to_account_info(),
                ],
            ),

            Err(error) => Err(error),
        };
    }

    pub fn transfer_spl_token2(ctx: Context<TransferSPLToken>, amount: u64) -> Result<()> {
        process::transfer_spl_token(
            ctx.accounts.source_account.to_account_info(),
            ctx.accounts.receiver_account.to_account_info(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            amount,
        )
    }

    pub fn create_associated_token_account(
        ctx: Context<CreateAssociatedTokenAccount>,
    ) -> Result<()> {
        msg!(
            "Created associated token account: {:#?}",
            ctx.accounts.token.to_account_info().key
        );
        Ok(())
    }

    pub fn create_associated_token_account2(
        ctx: Context<CreateAssociatedTokenAccount2>,
    ) -> Result<()> {
        if ctx.accounts.token.to_account_info().owner == &system_program::ID {
            msg!("Creating associated token account");
            let create_associated_token_ctx = associated_token::Create {
                payer: ctx.accounts.payer.to_account_info(),
                associated_token: ctx.accounts.associated_token_program.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            };

            let cpi_program = ctx.accounts.associated_token_program.to_account_info();
            let cpi_result =
                associated_token::create(CpiContext::new(cpi_program, create_associated_token_ctx));
            if let Ok(_) = cpi_result {
                msg!(
                    "Created associated token account: {}",
                    ctx.accounts.token.to_account_info().key
                );
            }

            return cpi_result;
        }

        Ok(())
    }

    pub fn create_nft(
        ctx: Context<CreateNFTContext>,
        title: String,
        uri: String,
        symbol: String,
    ) -> Result<()> {
        msg!("Initialized mint account {}", ctx.accounts.mint.key());
        msg!(
            "Initialized token account {}",
            ctx.accounts.token_account.key()
        );

        // Mint 1 token
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        mint_to(cpi_ctx, 1)?;

        // Create metadata address
        let creator = vec![mpl_token_metadata::state::Creator {
            address: ctx.accounts.mint_authority.key(),
            verified: false,
            share: 100,
        }];

        invoke(
            &create_metadata_accounts_v2(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.payer.key(),
                ctx.accounts.payer.key(),
                title,
                symbol,
                uri,
                Some(creator),
                1,
                true,
                false,
                None,
                None,
            ),
            &[
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.mint_authority.to_account_info(),
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.token_metadata_program.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ],
        )?;

        msg!("Created metadata");

        invoke(
            &create_master_edition_v3(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.master_edition.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.payer.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.payer.key(),
                Some(0),
            ),
            &[
                ctx.accounts.master_edition.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.mint_authority.to_account_info(),
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.token_metadata_program.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ],
        )?;
        msg!("Created master edition");

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

#[derive(Accounts)]
pub struct CreateAssociatedTokenAccount2<'info> {
    /// CHECK: unsafe
    pub token: AccountInfo<'info>,
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

#[derive(Accounts)]
pub struct CreateNFTContext<'info> {
    /// CHECK: Unsafe
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: Unsafe
    #[account(
        init_if_needed,
        payer = payer,
        mint::decimals = 0,
        mint::authority = mint_authority,
        mint::freeze_authority = mint_authority
    )]
    pub mint: Account<'info, Mint>,
    /// CHECK: Unsafe
    pub rent: AccountInfo<'info>,
    /// CHECK: Unsafe
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = mint_authority,
    )]
    pub token_account: Account<'info, TokenAccount>,
    /// CHECK: Unsafe
    #[account(mut)]
    pub metadata: AccountInfo<'info>,
    /// CHECK: Unsafe
    #[account(mut)]
    pub master_edition: AccountInfo<'info>,
    /// CHECK: Unsafe
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK: Unsafe
    pub token_metadata_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
