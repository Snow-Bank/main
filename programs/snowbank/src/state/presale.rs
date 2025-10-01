use anchor_lang::prelude::*;

#[account]
pub struct Presale {
    pub active: bool,
    pub base_tokens_per_sol: u64,
    pub step_lamports: u64,
    pub step_percent_bps: u16,
    pub referral_bps: u16,
    pub total_raised: u64,
    pub liabilities: u64,
}
