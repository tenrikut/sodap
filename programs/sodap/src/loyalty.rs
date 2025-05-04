// Loyalty-related instructions, events, and accounts will be placed here.

use crate::error::CustomError;
use crate::store::has_role;
use crate::types::AdminRoleType;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

// Loyalty instructions
// (redeem_loyalty_points, mint_loyalty_tokens, initialize_loyalty_mint)
// Loyalty events
// (LoyaltyPointsEarned, LoyaltyPointsRedeemed, LoyaltyTokensMinted)
// Loyalty accounts and context structs
// (RedeemLoyaltyPoints, MintLoyaltyTokens, InitializeLoyaltyMint)

pub fn redeem_loyalty_points(
    ctx: Context<RedeemLoyaltyPoints>,
    points_to_redeem: u64,
) -> Result<()> {
    let user_profile = &mut ctx.accounts.user_profile;
    let store_account = &ctx.accounts.store_account;
    require!(
        user_profile.loyalty_balance >= points_to_redeem,
        CustomError::InsufficientLoyaltyPoints
    );
    require!(
        store_account.loyalty_config.is_active,
        CustomError::LoyaltyProgramInactive
    );
    let discount_value = points_to_redeem * 10_000_000; // 0.01 SOL in lamports
    user_profile.loyalty_balance = user_profile
        .loyalty_balance
        .checked_sub(points_to_redeem)
        .ok_or(CustomError::ArithmeticError)?;
    emit!(LoyaltyPointsRedeemed {
        user: ctx.accounts.user.key(),
        store_id: ctx.accounts.store.key(),
        points_redeemed: points_to_redeem,
        discount_value,
        remaining_points: user_profile.loyalty_balance,
        timestamp: Clock::get()?.unix_timestamp,
    });
    Ok(())
}

pub fn mint_loyalty_tokens(ctx: Context<MintLoyaltyTokens>, amount: u64) -> Result<()> {
    let user_profile = &mut ctx.accounts.user_profile;
    require!(
        user_profile.loyalty_balance >= amount,
        CustomError::InsufficientLoyaltyPoints
    );
    // Mint SPL tokens logic would go here (omitted for brevity)
    user_profile.loyalty_balance = user_profile
        .loyalty_balance
        .checked_sub(amount)
        .ok_or(CustomError::ArithmeticError)?;
    emit!(LoyaltyTokensMinted {
        user: ctx.accounts.user.key(),
        amount,
        remaining_points: user_profile.loyalty_balance,
        timestamp: Clock::get()?.unix_timestamp,
    });
    Ok(())
}

pub fn initialize_loyalty_mint(ctx: Context<InitializeLoyaltyMint>) -> Result<()> {
    let store = &ctx.accounts.store;
    let signer = ctx.accounts.authority.key();
    require!(
        has_role(store, &signer, AdminRoleType::Owner)
            || has_role(store, &signer, AdminRoleType::PlatformAdmin),
        CustomError::Unauthorized
    );
    // CPI to initialize the mint would go here (omitted for brevity)
    Ok(())
}

#[event]
pub struct LoyaltyPointsEarned {
    pub user: Pubkey,
    pub store_id: Pubkey,
    pub points_earned: u64,
    pub total_points: u64,
    pub timestamp: i64,
}

#[event]
pub struct LoyaltyPointsRedeemed {
    pub user: Pubkey,
    pub store_id: Pubkey,
    pub points_redeemed: u64,
    pub discount_value: u64,
    pub remaining_points: u64,
    pub timestamp: i64,
}

#[event]
pub struct LoyaltyTokensMinted {
    pub user: Pubkey,
    pub amount: u64,
    pub remaining_points: u64,
    pub timestamp: i64,
}

#[derive(Accounts)]
pub struct RedeemLoyaltyPoints<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"user_profile", user.key().as_ref()],
        bump
    )]
    pub user_profile: Account<'info, crate::user::UserProfile>,
    /// CHECK: This is safe because we're only checking its key
    pub store: AccountInfo<'info>,
    #[account(
        seeds = [b"store", store.key().as_ref()],
        bump,
        constraint = store_account.is_active && store_account.loyalty_config.is_active
    )]
    pub store_account: Account<'info, crate::store::Store>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintLoyaltyTokens<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"user_profile", user.key().as_ref()],
        bump
    )]
    pub user_profile: Account<'info, crate::user::UserProfile>,
    #[account(
        mut,
        constraint = mint.decimals == 0 // Loyalty tokens have 0 decimals
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user
    )]
    pub token_account: Account<'info, TokenAccount>,
    /// CHECK: We're using PDA for mint authority
    #[account(
        seeds = [b"loyalty_mint"],
        bump
    )]
    pub mint_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct InitializeLoyaltyMint<'info> {
    #[account(mut)]
    pub authority: Signer<'info>, // store owner or platform admin
    #[account(
        mut,
        seeds = [b"store", store.key().as_ref()],
        bump
    )]
    pub store: Account<'info, crate::store::Store>,
    #[account(
        init,
        payer = authority,
        mint::decimals = 0,
        mint::authority = loyalty_mint_authority,
        seeds = [b"loyalty_mint", store.key().as_ref()],
        bump
    )]
    pub loyalty_mint: Account<'info, Mint>,
    /// CHECK: PDA for mint authority
    #[account(
        seeds = [b"loyalty_mint", store.key().as_ref()],
        bump
    )]
    pub loyalty_mint_authority: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}
