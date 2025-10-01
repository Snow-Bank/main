use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};
use crate::state::config::GlobalConfig;
use crate::errors::SnowErr;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct InitArgs {
    pub owner: Pubkey,
    pub treasury: Pubkey,
    pub timelock_seconds: u64,
}

#[derive(Accounts)]
#[instruction(args: InitArgs)]
pub struct InitGlobal<'info> {
    #[account(
        init,
        payer = payer,
        seeds = [GlobalConfig::SEED],
        bump,
        space = 8 + 32 + 32 + 32 + 8 + 1 + 1 + 64
    )]
    pub global: Account<'info, GlobalConfig>,

    #[account(mut)]
    pub snow_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn init_global(ctx: Context<InitGlobal>, args: InitArgs) -> Result<()> {
    let global = &mut ctx.accounts.global;
    global.owner = args.owner;
    global.treasury = args.treasury;
    global.snow_mint = ctx.accounts.snow_mint.key();
    global.timelock_seconds = args.timelock_seconds;
    global.paused = false;
    global.sweeps_enabled = true;
    Ok(())
}

#[derive(Accounts)]
pub struct Pause<'info> {
    #[account(mut, seeds = [GlobalConfig::SEED], bump, has_one = owner @ SnowErr::Unauthorized)]
    pub global: Account<'info, GlobalConfig>,
    pub owner: Signer<'info>,
}

pub fn pause(ctx: Context<Pause>) -> Result<()> {
    ctx.accounts.global.paused = true;
    Ok(())
}

#[derive(Accounts)]
pub struct Unpause<'info> {
    #[account(mut, seeds = [GlobalConfig::SEED], bump, has_one = owner @ SnowErr::Unauthorized)]
    pub global: Account<'info, GlobalConfig>,
    pub owner: Signer<'info>,
}

pub fn unpause(ctx: Context<Unpause>) -> Result<()> {
    ctx.accounts.global.paused = false;
    Ok(())
}
