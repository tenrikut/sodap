// User profile-related instructions, events, and accounts will be placed here.

use crate::error::CustomError;
use anchor_lang::prelude::*;

// User profile instructions
// (create_or_update_user_profile, scan_and_purchase)
// User profile events
// (UserProfileUpdated)
// User profile accounts and context structs
// (UserProfile, PurchaseRecord, CreateOrUpdateUserProfile, ScanAndPurchase)

pub fn create_or_update_user_profile(
    ctx: Context<CreateOrUpdateUserProfile>,
    user_id: Option<String>,
    delivery_address: Option<String>,
    preferred_store: Option<Pubkey>,
) -> Result<()> {
    let user_profile = &mut ctx.accounts.user_profile;
    let user = &ctx.accounts.user;
    if user_profile.wallet_address != user.key() {
        user_profile.wallet_address = user.key();
        user_profile.user_id = user_id.unwrap_or_else(|| "".to_string());
        user_profile.registered_at = Clock::get()?.unix_timestamp;
        user_profile.loyalty_balance = 0;
        user_profile.purchase_history = Vec::new();
    } else {
        if let Some(id) = user_id {
            user_profile.user_id = id;
        }
    }
    if let Some(address) = delivery_address {
        user_profile.delivery_address = address;
    }
    if let Some(store) = preferred_store {
        user_profile.preferred_store = Some(store);
    }
    emit!(UserProfileUpdated {
        wallet_address: user_profile.wallet_address,
        user_id: user_profile.user_id.clone(),
        updated_at: Clock::get()?.unix_timestamp,
    });
    Ok(())
}

pub fn scan_and_purchase(
    _ctx: Context<ScanAndPurchase>,
    product_uuids: Vec<[u8; 16]>,
    quantities: Vec<u64>,
    _store_id: Pubkey,
) -> Result<()> {
    require!(!product_uuids.is_empty(), CustomError::CartEmpty);
    require!(
        product_uuids.len() == quantities.len(),
        CustomError::InvalidCart
    );
    require!(product_uuids.len() <= 5, CustomError::CartTooLarge);
    // ... (rest of scan_and_purchase logic omitted for brevity)
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
    pub wallet_address: Pubkey,
    pub user_id: String,
    pub preferred_store: Option<Pubkey>,
    pub delivery_address: String,
    pub loyalty_balance: u64,
    pub purchase_history: Vec<PurchaseRecord>,
    pub registered_at: i64,
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
        payer = user,
        space = 8 + UserProfile::LEN,
        seeds = [b"user_profile", user.key().as_ref()],
        bump
    )]
    pub user_profile: Account<'info, UserProfile>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ScanAndPurchase<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    /// CHECK: This is safe because we only transfer lamports to this account
    #[account(mut)]
    pub store: AccountInfo<'info>,
    #[account(
        seeds = [b"store", store.key().as_ref()],
        bump,
        constraint = store_account.is_active
    )]
    pub store_account: Account<'info, crate::store::Store>,
    #[account(
        init,
        payer = buyer,
        space = 8 + crate::product::Cart::LEN,
        seeds = [b"cart", buyer.key().as_ref(), store.key().as_ref()],
        bump
    )]
    pub cart: Account<'info, crate::product::Cart>,
    #[account(
        mut,
        seeds = [b"user_profile", buyer.key().as_ref()],
        bump
    )]
    pub user_profile: Account<'info, UserProfile>,
    #[account(mut)]
    pub product1: Option<Account<'info, crate::product::Product>>,
    #[account(mut)]
    pub product2: Option<Account<'info, crate::product::Product>>,
    #[account(mut)]
    pub product3: Option<Account<'info, crate::product::Product>>,
    #[account(mut)]
    pub product4: Option<Account<'info, crate::product::Product>>,
    #[account(mut)]
    pub product5: Option<Account<'info, crate::product::Product>>,
    pub system_program: Program<'info, System>,
}
