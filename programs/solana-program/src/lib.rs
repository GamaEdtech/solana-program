use anchor_lang::prelude::*;

// Define constants
const PROPOSAL_ACCOUNT_SIZE: usize = 8 // Discriminator
    + 8 // ID
    + 32 // Creator's public key
    + 4 + 256 // Title (max 256 chars)
    + 4 + 1024 // Description (max 1024 chars)
    + 8 // Start date
    + 8 // End date
    + 8 // For votes
    + 8 // Against votes
    + 8; // Abstain votes

declare_id!("8S7KF153nyYtXSsVNokzZQdpDz3StcfiPBtmt73ZtBJy");

#[program]
pub mod solana_program {
    use super::*;

    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        title: String,
        description: String,
        start_date: i64,
        end_date: i64,
    ) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        let counter = &mut ctx.accounts.proposal_counter;

        // Validate title and description length
        if title.len() > 256 {
            return Err(error!(CreateProposalError::TitleTooLong));
        }

        if description.len() > 1024 {
            return Err(error!(CreateProposalError::DescriptionTooLong));
        }

        // Validate start and end dates
        if start_date >= end_date {
            return Err(error!(CreateProposalError::InvalidDates));
        }

        // Initialize the proposal
        proposal.id = counter.count;
        proposal.creator = *ctx.accounts.creator.key;
        proposal.title = title;
        proposal.description = description;
        proposal.start_date = start_date;
        proposal.end_date = end_date;
        proposal.for_votes = 0;
        proposal.against_votes = 0;
        proposal.abstain_votes = 0;

        // Increment the proposal counter
        counter.count += 1;

        Ok(())
    }
}


#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(
        init_if_needed, // Initialize the account if it doesn't exist
        payer = creator,
        space = 8 + 8, // Discriminator + count
        seeds = [b"proposal_counter", creator.key().as_ref()],
        bump
    )]
    pub proposal_counter: Account<'info, ProposalCounter>,

    #[account(
        init,
        payer = creator,
        space = PROPOSAL_ACCOUNT_SIZE,
        seeds = [b"proposal", creator.key().as_ref(), &proposal_counter.count.to_le_bytes()],
        bump
    )]
    pub proposal: Account<'info, Proposal>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Proposal {
    pub id: u64, // Unique identifier for the proposal
    pub creator: Pubkey, // Creator's public key or address
    pub title: String, // Title of the proposal
    pub description: String, // Description of the proposal
    pub start_date: i64, // Start date (as a Unix timestamp)
    pub end_date: i64, // End date (as a Unix timestamp)
    pub for_votes: u64, // Votes in favor
    pub against_votes: u64, // Votes against
    pub abstain_votes: u64, // Abstain votes
}

#[account]
pub struct ProposalCounter {
    pub count: u64, // Tracks the number of proposals created by a specific user
}

#[error_code]
pub enum CreateProposalError {
    #[msg("The title exceeds the maximum allowed length.")]
    TitleTooLong,
    #[msg("The description exceeds the maximum allowed length.")]
    DescriptionTooLong,
    #[msg("The start date must be earlier than the end date.")]
    InvalidDates,
}