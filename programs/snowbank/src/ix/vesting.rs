use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::state::config::GlobalConfig;
use crate::state::vesting::{Vesting, Grant};
use crate::util::token as token_util;
use crate::errors::SnowErr;

#[derive(Accounts)]
pub struct Configure<'info> {
    #[account(mut, seeds = [GlobalConfig::SEED], bump, has_one = owner @ SnowErr::Unauthorized)]
    pub global: Account<'info, GlobalConfig>,
    pub owner: Signer<'info>,
    #[account(init_if_needed, payer = owner, space = 8 + 2 + 2 + 8 + 8 + 8 + 32)]
    pub vesting: Account<'info, Vesting>,
    pub system_program: Program<'info, System>,
}

pub fn configure(ctx: Context<Configure>, upon_bps: u16, int_bps: u16, int_secs: u64) -> Result<()> {
    require!(upon_bps <= 10_000 && int_bps <= 10_000, SnowErr::InvalidBps);
    let v = &mut ctx.accounts.vesting;
    v.vesting_active = false;
    v.upon_launch_bps = upon_bps;
    v.interval_bps = int_bps;
    v.interval_secs = int_secs;
    v.start_ts = 0;
    v.initial_claim_deadline = 0;
    v.snow_mint = ctx.accounts.global.snow_mint;
    Ok(())
}

#[derive(Accounts)]
pub struct Begin<'info> {
    #[account(mut, seeds = [GlobalConfig::SEED], bump, has_one = owner @ SnowErr::Unauthorized)]
    pub global: Account<'info, GlobalConfig>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub vesting: Account<'info, Vesting>,
}

pub fn begin(ctx: Context<Begin>, initial_forfeit_after_secs: u64) -> Result<()> {
    let clock = Clock::get()?;
    let v = &mut ctx.accounts.vesting;
    v.vesting_active = true;
    v.start_ts = clock.unix_timestamp;
    v.initial_claim_deadline = clock.unix_timestamp + initial_forfeit_after_secs as i64;
    Ok(())
}

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(seeds = [GlobalConfig::SEED], bump)]
    pub global: Account<'info, GlobalConfig>,
    #[account(mut)]
    pub vesting: Account<'info, Vesting>,
    /// CHECK: signer
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_snow_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub snow_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

pub fn claim(_ctx: Context<Claim>) -> Result<()> {
    // Implement first-claim window + interval-claim logic; mint SNOW to user ATA
    Ok(())
}

#[derive(Accounts)]
pub struct ForfeitUnclaimed<'info> {
    #[account(seeds = [GlobalConfig::SEED], bump, has_one = owner @ SnowErr::Unauthorized)]
    pub global: Account<'info, GlobalConfig>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub vesting: Account<'info, Vesting>,
}

pub fn forfeit_unclaimed(_ctx: Context<ForfeitUnclaimed>, _user: Pubkey) -> Result<()> {
    Ok(())
}
