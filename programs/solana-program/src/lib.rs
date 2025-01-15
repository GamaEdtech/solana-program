use anchor_lang::prelude::*;

declare_id!("8S7KF153nyYtXSsVNokzZQdpDz3StcfiPBtmt73ZtBJy");

#[program]
pub mod solana_program {

}

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