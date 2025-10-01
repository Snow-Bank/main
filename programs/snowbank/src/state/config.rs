use anchor_lang::prelude::*;

#[account]
pub struct GlobalConfig {
    pub owner: Pubkey,
    pub treasury: Pubkey,
    pub snow_mint: Pubkey,
    pub timelock_seconds: u64,
    pub paused: bool,
    pub sweeps_enabled: bool,
}

impl GlobalConfig {
    pub const SEED: &'static [u8] = b"GLOBAL_CONFIG";
}
