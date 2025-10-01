use anchor_lang::prelude::*;
use crate::state::config::GlobalConfig;
use crate::state::presale::Presale;
use crate::errors::SnowErr;

#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(mut, seeds = [GlobalConfig::SEED], bump)]
    pub global: Account<'info, GlobalConfig>,
    #[account(mut)]
    pub presale: Account<'info, Presale>,
    /// CHECK: lamports are accumulated in this PDA
    #[account(mut)]
    pub presale_treasury: AccountInfo<'info>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn buy(_ctx: Context<Buy>, _lamports_in: u64, _referrer: Option<Pubkey>) -> Result<()> {
    // NOTE: Implement stepped pricing and vesting grant crediting here.
    Ok(())
}

#[derive(Accounts)]
pub struct ClaimReferral<'info> {
    #[account(seeds = [GlobalConfig::SEED], bump)]
    pub global: Account<'info, GlobalConfig>,
    pub claimer: Signer<'info>,
}

pub fn claim_referral(_ctx: Context<ClaimReferral>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawFree<'info> {
    #[account(mut, seeds = [GlobalConfig::SEED], bump)]
    pub global: Account<'info, GlobalConfig>,
    /// CHECK:
    #[account(mut)]
    pub presale_treasury: AccountInfo<'info>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn withdraw_free(_ctx: Context<WithdrawFree>) -> Result<()> {
    Ok(())
}
