use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer};
use crate::state::config::GlobalConfig;
use crate::state::farm::{Pool, UserStake};
use crate::errors::SnowErr;

#[derive(Accounts)]
pub struct AddPool<'info> {
    #[account(mut, seeds = [GlobalConfig::SEED], bump, has_one = owner @ SnowErr::Unauthorized)]
    pub global: Account<'info, GlobalConfig>,
    pub owner: Signer<'info>,

    #[account(init, payer = owner, space = 8 + 32 + 32 + 8 + 2 + 16 + 8 + 8 + 1 + 8)]
    pub pool: Account<'info, Pool>,

    pub system_program: Program<'info, System>,
}

pub fn add_pool(ctx: Context<AddPool>, alloc: u64, fee_bps: u16, sweeps_enabled: bool) -> Result<()> {
    require!(fee_bps <= 1000, SnowErr::InvalidBps);
    let p = &mut ctx.accounts.pool;
    p.alloc_point = alloc;
    p.deposit_fee_bps = fee_bps;
    p.sweeps_enabled = sweeps_enabled;
    p.acc_reward_per_share = 0;
    p.last_reward_ts = Clock::get()?.unix_timestamp;
    p.total_staked = 0;
    p.emission_per_sec = 0;
    Ok(())
}

#[derive(Accounts)]
pub struct SetAllocPoint<'info> {
    #[account(mut, has_one = owner @ SnowErr::Unauthorized)]
    pub global: Account<'info, GlobalConfig>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub pool: Account<'info, Pool>,
}
pub fn set_alloc_point(ctx: Context<SetAllocPoint>, alloc: u64) -> Result<()> {
    ctx.accounts.pool.alloc_point = alloc;
    Ok(())
}

#[derive(Accounts)]
pub struct SetDepositFeeBps<'info> {
    #[account(mut, has_one = owner @ SnowErr::Unauthorized)]
    pub global: Account<'info, GlobalConfig>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub pool: Account<'info, Pool>,
}
pub fn set_deposit_fee_bps(ctx: Context<SetDepositFeeBps>, bps: u16) -> Result<()> {
    require!(bps <= 1000, SnowErr::InvalidBps);
    ctx.accounts.pool.deposit_fee_bps = bps;
    Ok(())
}

#[derive(Accounts)]
pub struct SetEmissionTokens<'info> {
    #[account(mut, has_one = owner @ SnowErr::Unauthorized)]
    pub global: Account<'info, GlobalConfig>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub pool: Account<'info, Pool>,
}
pub fn set_emission_per_sec_tokens(ctx: Context<SetEmissionTokens>, tokens_per_sec: u64) -> Result<()> {
    // NOTE: UI should multiply by 10^decimals. This helper keeps it in base units.
    ctx.accounts.pool.emission_per_sec = tokens_per_sec;
    Ok(())
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(seeds = [GlobalConfig::SEED], bump)]
    pub global: Account<'info, GlobalConfig>,
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: vault authority & TAs would be added here
    pub token_program: Program<'info, Token>,
}
pub fn deposit(_ctx: Context<Deposit>, _amount: u64) -> Result<()> { Ok(()) }

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(seeds = [GlobalConfig::SEED], bump)]
    pub global: Account<'info, GlobalConfig>,
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}
pub fn withdraw(_ctx: Context<Withdraw>, _amount: u64) -> Result<()> { Ok(()) }

#[derive(Accounts)]
pub struct Harvest<'info> {
    #[account(seeds = [GlobalConfig::SEED], bump)]
    pub global: Account<'info, GlobalConfig>,
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}
pub fn harvest(_ctx: Context<Harvest>) -> Result<()> { Ok(()) }

#[derive(Accounts)]
pub struct SweepInactive<'info> {
    #[account(seeds = [GlobalConfig::SEED], bump)]
    pub global: Account<'info, GlobalConfig>,
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    pub actor: Signer<'info>,
}
pub fn sweep_inactive(_ctx: Context<SweepInactive>, _users: Vec<Pubkey>) -> Result<()> { Ok(()) }
