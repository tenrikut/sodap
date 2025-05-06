use crate::state::loyalty::{InitializeLoyaltyMint, MintLoyaltyTokens, RedeemLoyaltyPoints};
use anchor_lang::prelude::*;

// Redeem a specified amount of loyalty points from a store's loyalty mint
pub fn redeem_loyalty_points(
    ctx: Context<RedeemLoyaltyPoints>,
    points_to_redeem: u64,
) -> Result<()> {
    let loyalty_mint = &mut ctx.accounts.loyalty_mint;
    loyalty_mint.total_supply -= points_to_redeem;
    Ok(())
}

// Mint new loyalty tokens to the store's loyalty mint
pub fn mint_loyalty_tokens(ctx: Context<MintLoyaltyTokens>, amount: u64) -> Result<()> {
    let loyalty_mint = &mut ctx.accounts.loyalty_mint;
    loyalty_mint.total_supply += amount;
    Ok(())
}

// Initialize a new loyalty mint for the given store
pub fn initialize_loyalty_mint(ctx: Context<InitializeLoyaltyMint>) -> Result<()> {
    let loyalty_mint = &mut ctx.accounts.loyalty_mint;
    loyalty_mint.store = ctx.accounts.store.key();
    loyalty_mint.authority = ctx.accounts.authority.key();
    Ok(())
}
