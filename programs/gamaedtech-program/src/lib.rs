use anchor_lang::prelude::*;

declare_id!("EUodMYMz5EEq5DAgSTWEqKzYQgiZRFHkZEre6SFByf9N");

use instructions::*;
mod error;
mod instructions;
mod state;
#[program]
pub mod gamaedtech_program {

    use super::*;

    pub fn create_proposal(
        ctx: Context<SubmitProposal>,
        title: String,
        brief: String,
        cate: String,
        reference: String,
        amount: u64,
    ) -> Result<()> {
        proccess_create_proposal(ctx, title, brief, cate, reference, amount)
    }

    pub fn delete_proposal(ctx: Context<DeleteProposal>) -> Result<()> {
        proccess_delete_proposal(ctx)
    }

    pub fn vote(ctx: Context<Vote>, agree: bool, vote_power: u64) -> Result<()> {
        proccess_vote(ctx, agree, vote_power)
    }
}
