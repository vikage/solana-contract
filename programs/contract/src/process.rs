use anchor_spl::token;
use anchor_lang::prelude::*;

pub fn transfer_spl_token<'info>(source: AccountInfo<'info>, destination: AccountInfo<'info>, authority: AccountInfo<'info>, token_program: AccountInfo<'info>, amount: u64) -> Result<()> {
    let transfer_ctx = token::Transfer {
        from: source,
        to: destination,
        authority,
    };

    return token::transfer(CpiContext::new(token_program, transfer_ctx), amount)
}