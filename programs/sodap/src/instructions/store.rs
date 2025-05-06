use crate::state::error::CustomError;
use crate::state::store::AdminRole;
use crate::types::AdminRoleType;
use crate::types::LoyaltyConfig;
use anchor_lang::prelude::*;

/// Instruction to register a new store
pub fn register_store(
    ctx: Context<RegisterStore>,
    _store_id: Pubkey, // optional, for logging purposes
    name: String,
    description: String,
    logo_uri: String,
    loyalty_config: LoyaltyConfig,
) -> Result<()> {
    let store = &mut ctx.accounts.store;
    let authority = &ctx.accounts.authority;

    // Authority must be signer
    require!(authority.is_signer, CustomError::Unauthorized);

    store.owner = authority.key();
    store.name = name;
    store.description = description;
    store.logo_uri = logo_uri;
    store.loyalty_config = loyalty_config;
    store.is_active = true;
    store.revenue = 0;
    store.admin_roles = vec![];

    Ok(())
}

/// Instruction to update a store's metadata
pub fn update_store(
    ctx: Context<UpdateStore>,
    _store_id: Pubkey, // optional, for logging
    name: Option<String>,
    description: Option<String>,
    logo_uri: Option<String>,
    loyalty_config: Option<LoyaltyConfig>,
) -> Result<()> {
    let store = &mut ctx.accounts.store;
    let authority = &ctx.accounts.owner;

    // Only the owner can update
    require!(authority.key() == store.owner, CustomError::Unauthorized);
    require!(authority.is_signer, CustomError::Unauthorized);

    if let Some(name) = name {
        store.name = name;
    }
    if let Some(description) = description {
        store.description = description;
    }
    if let Some(logo_uri) = logo_uri {
        store.logo_uri = logo_uri;
    }
    if let Some(config) = loyalty_config {
        store.loyalty_config = config;
    }

    Ok(())
}

/// Instruction to add an admin to a store
pub fn add_admin(
    ctx: Context<AddAdmin>,
    _store_id: Pubkey,
    admin_pubkey: Pubkey,
    role_type: AdminRoleType,
) -> Result<()> {
    let store = &mut ctx.accounts.store;
    let authority = &ctx.accounts.owner;

    // Only the owner can add admins
    require!(authority.key() == store.owner, CustomError::Unauthorized);
    require!(authority.is_signer, CustomError::Unauthorized);

    if store
        .admin_roles
        .iter()
        .any(|r| r.admin_pubkey == admin_pubkey)
    {
        return Err(CustomError::AdminAlreadyExists.into());
    }

    store.admin_roles.push(AdminRole {
        admin_pubkey,
        role_type,
    });
    Ok(())
}

/// Instruction to remove an admin from a store
pub fn remove_admin(
    ctx: Context<RemoveAdmin>,
    _store_id: Pubkey,
    admin_pubkey: Pubkey,
) -> Result<()> {
    let store = &mut ctx.accounts.store;
    let authority = &ctx.accounts.owner;

    // Only the owner can remove admins
    require!(authority.key() == store.owner, CustomError::Unauthorized);
    require!(authority.is_signer, CustomError::Unauthorized);

    store.admin_roles.retain(|r| r.admin_pubkey != admin_pubkey);
    Ok(())
}

// Re-export contexts from state
pub use crate::state::store::{AddAdmin, RegisterStore, RemoveAdmin, UpdateStore};
