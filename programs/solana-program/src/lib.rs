use anchor_lang::prelude::*;

declare_id!("8S7KF153nyYtXSsVNokzZQdpDz3StcfiPBtmt73ZtBJy");

#[program]
pub mod solana_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
