use crate::error::CustomError;
use crate::store::Store;
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

// Event: user earned loyalty points
#[event]
pub struct LoyaltyPointsEarned {
    pub user: Pubkey,
    pub store_id: Pubkey,
    pub points_earned: u64,
    pub total_points: u64,
    pub timestamp: i64,
}

// Event: user redeemed loyalty points
#[event]
pub struct LoyaltyPointsRedeemed {
    pub user: Pubkey,
    pub store_id: Pubkey,
    pub points_redeemed: u64,
    pub discount_value: u64,
    pub remaining_points: u64,
    pub timestamp: i64,
}

// Event: loyalty tokens were minted
#[event]
pub struct LoyaltyTokensMinted {
    pub user: Pubkey,
    pub amount: u64,
    pub remaining_points: u64,
    pub timestamp: i64,
}

// Account that stores information about the loyalty token mint
#[account]
pub struct LoyaltyMint {
    pub store: Pubkey,     // The store this mint is associated with
    pub authority: Pubkey, // Authority who can mint/redeem tokens
    pub total_supply: u64, // Total loyalty tokens in circulation
}

// Context: Redeem loyalty tokens from the mint
#[derive(Accounts)]
pub struct RedeemLoyaltyPoints<'info> {
    #[account(mut)]
    pub store: Account<'info, Store>,
    #[account(
        mut,
        seeds = [b"loyalty_mint", store.key().as_ref()],
        bump,
        has_one = store,
        has_one = authority
    )]
    pub loyalty_mint: Account<'info, LoyaltyMint>,
    pub authority: Signer<'info>,
}

// Context: Mint new loyalty tokens
#[derive(Accounts)]
pub struct MintLoyaltyTokens<'info> {
    #[account(mut)]
    pub store: Account<'info, Store>,
    #[account(
        mut,
        seeds = [b"loyalty_mint", store.key().as_ref()],
        bump,
        has_one = store,
        has_one = authority
    )]
    pub loyalty_mint: Account<'info, LoyaltyMint>,
    pub authority: Signer<'info>,
}

// Context: Initialize the loyalty mint account for a store
#[derive(Accounts)]
pub struct InitializeLoyaltyMint<'info> {
    #[account(mut)]
    pub store: Account<'info, Store>,
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 8,
        seeds = [b"loyalty_mint", store.key().as_ref()],
        bump
    )]
    pub loyalty_mint: Account<'info, LoyaltyMint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
