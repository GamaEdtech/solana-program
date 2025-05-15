use crate::error::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>, // The proposal being voted on
    #[account(mut)]
    pub voter: Signer<'info>, // Voter's wallet
    #[account(
        init_if_needed,
        payer = voter,
        space = 8 + std::mem::size_of::<VoteRecord>(),
        seeds = [
            b"vote-record", // Static seed
            proposal.key().as_ref(), // Proposal key (32 bytes)
            voter.key().as_ref() // Voter's public key (32 bytes)
        ],  bump,
    )]
    pub vote_record: Account<'info, VoteRecord>, // Prevent double voting
    pub system_program: Program<'info, System>, // Solana system program
}

pub fn proccess_vote(ctx: Context<Vote>, agree: bool, vote_power: u64) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let vote_record = &mut ctx.accounts.vote_record;

    let current_time = Clock::get()?.unix_timestamp;
    require!(
        proposal.expires_at >= current_time,
        ErrorCode::ExpiredProposal
    );

    // Prevent double voting
    require!(!vote_record.has_voted, ErrorCode::AlreadyVoted);
    vote_record.has_voted = true;

    // Count the vote
    vote_record.vote_power = vote_power;
    if agree {
        proposal.agree_votes += vote_power;
        vote_record.vote = "agree".to_string();
    } else {
        proposal.disagree_votes += vote_power;
        vote_record.vote = "disagree".to_string();
    }

    Ok(())
}
