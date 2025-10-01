use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount, Transfer, Approve, Revoke};

pub fn mint_to<'info>(
    token_program: Program<'info, Token>,
    mint: Account<'info, Mint>,
    to: Account<'info, TokenAccount>,
    signer_seeds: &[&[u8]],
    amount: u64,
) -> Result<()> {
    let cpi_ctx = CpiContext::new_with_signer(
        token_program.to_account_info(),
        MintTo {
            mint: mint.to_account_info(),
            to: to.to_account_info(),
            authority: mint.to_account_info(),
        },
        &[signer_seeds],
    );
    token::mint_to(cpi_ctx, amount)
}
