use anchor_lang::prelude::*;

#[error_code]
pub enum CustomErrors {
    #[msg("voting period is not started yet")]
    VotingNotStarted,

    #[msg("voting period is already end")]
    VotingEnded,
}
