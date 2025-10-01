use anchor_lang::prelude::*;

pub mod errors;
pub mod state { pub mod config; pub mod presale; pub mod vesting; pub mod farm; pub mod timelock; }
pub mod ix { pub mod admin; pub mod presale; pub mod vesting; pub mod farm; }
pub mod util { pub mod math; pub mod token; }

use ix::*;

declare_id!("SNOWBnk111111111111111111111111111111111111");

#[program]
pub mod snowbank {
    use super::*;

    pub fn init_global(ctx: Context<admin::InitGlobal>, args: admin::InitArgs) -> Result<()> {
        admin::init_global(ctx, args)
    }
    pub fn pause(ctx: Context<admin::Pause>) -> Result<()> { admin::pause(ctx) }
    pub fn unpause(ctx: Context<admin::Unpause>) -> Result<()> { admin::unpause(ctx) }

    pub fn presale_buy(ctx: Context<presale::Buy>, lamports_in: u64, referrer: Option<Pubkey>) -> Result<()> {
        presale::buy(ctx, lamports_in, referrer)
    }
    pub fn presale_claim_referral(ctx: Context<presale::ClaimReferral>) -> Result<()> {
        presale::claim_referral(ctx)
    }
    pub fn presale_withdraw_free(ctx: Context<presale::WithdrawFree>) -> Result<()> {
        presale::withdraw_free(ctx)
    }

    pub fn vesting_configure(ctx: Context<vesting::Configure>, upon_bps: u16, int_bps: u16, int_secs: u64) -> Result<()> {
        vesting::configure(ctx, upon_bps, int_bps, int_secs)
    }
    pub fn vesting_begin(ctx: Context<vesting::Begin>, initial_forfeit_after_secs: u64) -> Result<()> {
        vesting::begin(ctx, initial_forfeit_after_secs)
    }
    pub fn vesting_claim(ctx: Context<vesting::Claim>) -> Result<()> { vesting::claim(ctx) }
    pub fn vesting_forfeit_unclaimed(ctx: Context<vesting::ForfeitUnclaimed>, user: Pubkey) -> Result<()> {
        vesting::forfeit_unclaimed(ctx, user)
    }

    pub fn add_pool(ctx: Context<farm::AddPool>, alloc: u64, fee_bps: u16, sweeps_enabled: bool) -> Result<()> {
        farm::add_pool(ctx, alloc, fee_bps, sweeps_enabled)
    }
    pub fn set_alloc_point(ctx: Context<farm::SetAllocPoint>, alloc: u64) -> Result<()> {
        farm::set_alloc_point(ctx, alloc)
    }
    pub fn set_deposit_fee_bps(ctx: Context<farm::SetDepositFeeBps>, bps: u16) -> Result<()> {
        farm::set_deposit_fee_bps(ctx, bps)
    }
    pub fn set_emission_per_sec_tokens(ctx: Context<farm::SetEmissionTokens>, tokens_per_sec: u64) -> Result<()> {
        farm::set_emission_per_sec_tokens(ctx, tokens_per_sec)
    }
    pub fn deposit(ctx: Context<farm::Deposit>, amount: u64) -> Result<()> { farm::deposit(ctx, amount) }
    pub fn withdraw(ctx: Context<farm::Withdraw>, amount: u64) -> Result<()> { farm::withdraw(ctx, amount) }
    pub fn harvest(ctx: Context<farm::Harvest>) -> Result<()> { farm::harvest(ctx) }
    pub fn sweep_inactive(ctx: Context<farm::SweepInactive>, users: Vec<Pubkey>) -> Result<()> {
        farm::sweep_inactive(ctx, users)
    }
}
