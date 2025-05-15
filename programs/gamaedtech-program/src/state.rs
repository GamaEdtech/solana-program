use anchor_lang::prelude::*;
#[account]
pub struct Proposal {
    pub owner: Pubkey,
    pub title: String,
    pub brief: String,
    pub cate: String,
    pub reference: String,
    pub amount: u64,
    pub agree_votes: u64, // Number of "Agree" votes
    pub disagree_votes: u64,
    pub created_at: i64, // Creation timestamp in seconds
    pub expires_at: i64, // Expiration timestamp in seconds
}

#[account]
pub struct VoteRecord {
    pub proposal_id: Pubkey, // Linked proposal ID
    pub voter: Pubkey,       // Voter's public key
    pub has_voted: bool,     // Prevent double voting
    pub vote: String,
    pub vote_power: u64,
}
