use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized to Action")]
    Unauthorized,

    #[msg("Already voted!")]
    AlreadyVoted,

    #[msg("The Proposal Has Already Expired")]
    ExpiredProposal,
}
