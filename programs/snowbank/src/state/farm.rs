use anchor_lang::prelude::*;

#[account]
pub struct Pool {
    pub stake_mint: Pubkey,
    pub vault_ta: Pubkey,
    pub alloc_point: u64,
    pub deposit_fee_bps: u16,
    pub acc_reward_per_share: u128,
    pub last_reward_ts: i64,
    pub total_staked: u64,
    pub sweeps_enabled: bool,
    pub emission_per_sec: u64,
}

#[account]
pub struct UserStake {
    pub user: Pubkey,
    pub pool: Pubkey,
    pub amount: u64,
    pub reward_debt: i128,
    pub last_interaction: i64,
    pub next_sweep_at: i64,
}
