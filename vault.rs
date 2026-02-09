use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub total_supply: u64,
}
