use anchor_lang::prelude::*;

#[account]
pub struct Vesting {
    pub vesting_active: bool,
    pub upon_launch_bps: u16,
    pub interval_bps: u16,
    pub interval_secs: u64,
    pub start_ts: i64,
    pub initial_claim_deadline: i64,
    pub snow_mint: Pubkey,
}

#[account]
pub struct Grant {
    pub user: Pubkey,
    pub total: u64,
    pub claimed: u64,
    pub last_claim_at: i64,
    pub forfeited: bool,
}
