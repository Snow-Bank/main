use anchor_lang::prelude::*;

#[account]
pub struct TimelockItem {
    pub op_hash: [u8; 32],
    pub execute_at: i64,
    pub executed: bool,
}
