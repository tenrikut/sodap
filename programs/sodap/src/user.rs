// User profile-related instructions, events, and accounts will be placed here.

use crate::error::CustomError;
use anchor_lang::prelude::*;

pub fn create_or_update_user_profile(
    ctx: Context<CreateOrUpdateUserProfile>,
    user_id: Option<String>,
    delivery_address: Option<String>,
    preferred_store: Option<Pubkey>,
) -> Result<()> {
    let user_profile = &mut ctx.accounts.user_profile;
    user_profile.authority = ctx.accounts.authority.key();

    if let Some(user_id) = user_id {
        user_profile.user_id = user_id.clone();
        emit!(UserProfileUpdated {
            wallet_address: ctx.accounts.authority.key(),
            user_id,
            updated_at: Clock::get()?.unix_timestamp,
        });
    }

    if let Some(delivery_address) = delivery_address {
        user_profile.delivery_address = delivery_address;
        emit!(UserProfileUpdated {
            wallet_address: ctx.accounts.authority.key(),
            user_id: user_profile.user_id.clone(),
            updated_at: Clock::get()?.unix_timestamp,
        });
    }

    if let Some(preferred_store) = preferred_store {
        user_profile.preferred_store = preferred_store;
        emit!(UserProfileUpdated {
            wallet_address: ctx.accounts.authority.key(),
            user_id: user_profile.user_id.clone(),
            updated_at: Clock::get()?.unix_timestamp,
        });
    }

    Ok(())
}

pub fn scan_and_purchase(
    ctx: Context<ScanAndPurchase>,
    product_uuids: Vec<[u8; 16]>,
    quantities: Vec<u64>,
    _store_id: Pubkey,
) -> Result<()> {
    require!(
        product_uuids.len() == quantities.len(),
        CustomError::InvalidCart
    );
    let user_profile = &mut ctx.accounts.user_profile;
    user_profile.total_purchases += 1;
    emit!(UserProfileUpdated {
        wallet_address: ctx.accounts.authority.key(),
        user_id: user_profile.user_id.clone(),
        updated_at: Clock::get()?.unix_timestamp,
    });
    Ok(())
}

#[event]
pub struct UserProfileUpdated {
    pub wallet_address: Pubkey,
    pub user_id: String,
    pub updated_at: i64,
}

#[account]
pub struct UserProfile {
    pub authority: Pubkey,
    pub user_id: String,
    pub delivery_address: String,
    pub preferred_store: Pubkey,
    pub total_purchases: u64,
}

impl UserProfile {
    pub const MAX_USER_ID_LEN: usize = 50;
    pub const MAX_ADDRESS_LEN: usize = 200;
    pub const MAX_PURCHASE_HISTORY: usize = 10;
    pub const LEN: usize = 32
        + 4
        + Self::MAX_USER_ID_LEN
        + 33
        + 4
        + Self::MAX_ADDRESS_LEN
        + 8
        + 4
        + (32 + 32 + 8 + 8 + 8) * Self::MAX_PURCHASE_HISTORY
        + 8;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PurchaseRecord {
    pub store_id: Pubkey,
    pub transaction_id: Pubkey,
    pub amount: u64,
    pub loyalty_earned: u64,
    pub timestamp: i64,
}

#[derive(Accounts)]
pub struct CreateOrUpdateUserProfile<'info> {
    #[account(
        init_if_needed,
        payer = authority,
        space = UserProfile::LEN,
        seeds = [b"user_profile", authority.key().as_ref()],
        bump
    )]
    pub user_profile: Account<'info, UserProfile>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ScanAndPurchase<'info> {
    #[account(
        mut,
        seeds = [b"user_profile", authority.key().as_ref()],
        bump,
        has_one = authority
    )]
    pub user_profile: Account<'info, UserProfile>,
    pub authority: Signer<'info>,
}
