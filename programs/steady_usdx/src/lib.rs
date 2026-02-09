use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

declare_id!("Steady1111111111111111111111111111111111");

#[program]
pub mod steady_usdx {
    use super::*;

    pub fn initialize(ctx: Context<instructions::Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    pub fn mint(ctx: Context<instructions::Mint>, amount: u64) -> Result<()> {
        instructions::mint(ctx, amount)
    }
}
