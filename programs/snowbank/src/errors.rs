use anchor_lang::prelude::*;

#[error_code]
pub enum SnowErr {
    #[msg("Operation is paused")]
    Paused,
    #[msg("Invalid bps value")]
    InvalidBps,
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Too soon to claim again")]
    TooSoon,
    #[msg("After initial claim deadline")]
    AfterDeadline,
    #[msg("Nothing to claim")]
    NothingToClaim,
    #[msg("Insufficient free balance")]
    InsufficientFreeBalance,
}
