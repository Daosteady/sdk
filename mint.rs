use anchor_lang::prelude::*;
use crate::state::Vault;

pub fn mint(ctx: Context<Mint>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    vault.total_supply += amount;
    Ok(())
}

#[derive(Accounts)]
pub struct Mint<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
}
