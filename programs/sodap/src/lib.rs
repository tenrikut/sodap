use crate::instructions::{admin, loyalty, product, store, user};
use anchor_lang::prelude::*;
// Declare the program ID used by Anchor
declare_id!("4eLJ3QGiNrPN6UUr2fNxq6tUZqFdBMVpXkL2MhsKNriv");

mod error;
mod types;
mod utils;

pub mod instructions;

mod state;
use state::*;
use types::*;

#[program]
pub mod sodap {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Program initialized by: {:?}", ctx.accounts.payer.key());
        Ok(())
    }

    // Store-related instructions
    pub fn register_store(
        ctx: Context<RegisterStore>,
        store_id: Pubkey,
        name: String,
        description: String,
        logo_uri: String,
        loyalty_config: LoyaltyConfig,
    ) -> Result<()> {
        store::register_store(ctx, store_id, name, description, logo_uri, loyalty_config)
    }

    pub fn update_store(
        ctx: Context<UpdateStore>,
        store_id: Pubkey,
        name: Option<String>,
        description: Option<String>,
        logo_uri: Option<String>,
        loyalty_config: Option<LoyaltyConfig>,
    ) -> Result<()> {
        store::update_store(ctx, store_id, name, description, logo_uri, loyalty_config)
    }

    pub fn add_admin(
        ctx: Context<AddAdmin>,
        store_id: Pubkey,
        admin_pubkey: Pubkey,
        role_type: AdminRoleType,
    ) -> Result<()> {
        store::add_admin(ctx, store_id, admin_pubkey, role_type)
    }

    pub fn remove_admin(
        ctx: Context<RemoveAdmin>,
        store_id: Pubkey,
        admin_pubkey: Pubkey,
    ) -> Result<()> {
        store::remove_admin(ctx, store_id, admin_pubkey)
    }

    // Product-related instructions
    pub fn register_product(
        ctx: Context<RegisterProduct>,
        product_uuid: [u8; 16],
        price: u64,
        stock: u64,
        tokenized_type: TokenizedType,
        metadata_uri: String,
    ) -> Result<()> {
        product::register_product(
            ctx,
            product_uuid,
            price,
            stock,
            tokenized_type,
            metadata_uri,
        )
    }

    pub fn update_product(
        ctx: Context<UpdateProduct>,
        product_uuid: [u8; 16],
        new_price: Option<u64>,
        new_stock: Option<u64>,
        new_metadata_uri: Option<String>,
        new_tokenized_type: Option<TokenizedType>,
    ) -> Result<()> {
        product::update_product(
            ctx,
            product_uuid,
            new_price,
            new_stock,
            new_metadata_uri,
            new_tokenized_type,
        )
    }

    pub fn deactivate_product(
        ctx: Context<DeactivateProduct>,
        product_uuid: [u8; 16],
    ) -> Result<()> {
        product::deactivate_product(ctx, product_uuid)
    }

    pub fn purchase_cart<'info>(
        ctx: Context<'_, '_, 'info, 'info, PurchaseCart>,
        product_uuids: Vec<[u8; 16]>,
        quantities: Vec<u64>,
        total_price: u64,
        gas_fee: u64,
        status: TransactionStatus,
    ) -> Result<()> {
        product::purchase_cart(ctx, product_uuids, quantities, total_price, gas_fee, status)
    }

    // Loyalty system instructions
    pub fn redeem_loyalty_points(
        ctx: Context<RedeemLoyaltyPoints>,
        points_to_redeem: u64,
    ) -> Result<()> {
        loyalty::redeem_loyalty_points(ctx, points_to_redeem)
    }

    pub fn mint_loyalty_tokens(ctx: Context<MintLoyaltyTokens>, amount: u64) -> Result<()> {
        loyalty::mint_loyalty_tokens(ctx, amount)
    }

    pub fn initialize_loyalty_mint(ctx: Context<InitializeLoyaltyMint>) -> Result<()> {
        loyalty::initialize_loyalty_mint(ctx)
    }

    // User profile instructions
    pub fn create_or_update_user_profile(
        ctx: Context<CreateOrUpdateUserProfile>,
        user_id: Option<String>,
        delivery_address: Option<String>,
        preferred_store: Option<Pubkey>,
    ) -> Result<()> {
        user::create_or_update_user_profile(ctx, user_id, delivery_address, preferred_store)
    }

    pub fn scan_and_purchase(
        ctx: Context<ScanAndPurchase>,
        product_uuids: Vec<[u8; 16]>,
        quantities: Vec<u64>,
        store_id: Pubkey,
    ) -> Result<()> {
        user::scan_and_purchase(ctx, product_uuids, quantities, store_id)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
