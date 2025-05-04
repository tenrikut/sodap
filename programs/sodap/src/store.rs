// Store-related instructions, events, and accounts will be placed here.

use crate::error::CustomError;
use crate::types::{AdminRoleType, LoyaltyConfig};
use anchor_lang::prelude::*;

// Store instructions
pub fn register_store(
    ctx: Context<RegisterStore>,
    store_id: Pubkey,
    name: String,
    description: String,
    logo_uri: String,
    loyalty_config: LoyaltyConfig,
) -> Result<()> {
    let store = &mut ctx.accounts.store;
    store.owner = ctx.accounts.owner.key();
    store.name = name;
    store.description = description;
    store.logo_uri = logo_uri;
    store.created_at = Clock::get()?.unix_timestamp;
    store.revenue = 0;
    store.loyalty_config = loyalty_config;
    store.admin_roles = vec![AdminRole {
        admin_pubkey: ctx.accounts.owner.key(),
        role_type: AdminRoleType::Owner,
    }];
    store.is_active = true;
    emit!(StoreRegistered {
        store_id,
        owner: ctx.accounts.owner.key(),
        name: store.name.clone(),
        created_at: store.created_at,
    });
    Ok(())
}

pub fn update_store(
    ctx: Context<UpdateStore>,
    store_id: Pubkey,
    name: Option<String>,
    description: Option<String>,
    logo_uri: Option<String>,
    loyalty_config: Option<LoyaltyConfig>,
) -> Result<()> {
    let store = &mut ctx.accounts.store;
    let signer = ctx.accounts.owner.key();
    require!(
        has_role(store, &signer, AdminRoleType::Owner)
            || has_role(store, &signer, AdminRoleType::Manager),
        CustomError::Unauthorized
    );
    if let Some(name) = name {
        store.name = name;
    }
    if let Some(description) = description {
        store.description = description;
    }
    if let Some(logo_uri) = logo_uri {
        store.logo_uri = logo_uri;
    }
    if let Some(loyalty_config) = loyalty_config {
        store.loyalty_config = loyalty_config;
    }
    emit!(StoreUpdated {
        store_id,
        updated_by: ctx.accounts.owner.key(),
        updated_at: Clock::get()?.unix_timestamp,
    });
    Ok(())
}

pub fn add_admin(
    ctx: Context<AddAdmin>,
    store_id: Pubkey,
    admin_pubkey: Pubkey,
    role_type: AdminRoleType,
) -> Result<()> {
    let store = &mut ctx.accounts.store;
    let signer = ctx.accounts.owner.key();
    require!(
        has_role(store, &signer, AdminRoleType::Owner),
        CustomError::Unauthorized
    );
    if store
        .admin_roles
        .iter()
        .any(|role| role.admin_pubkey == admin_pubkey)
    {
        return Err(CustomError::AdminAlreadyExists.into());
    }
    store.admin_roles.push(AdminRole {
        admin_pubkey,
        role_type: role_type.clone(),
    });
    emit!(AdminAdded {
        store_id,
        admin_pubkey,
        role_type,
        added_at: Clock::get()?.unix_timestamp,
    });
    Ok(())
}

pub fn remove_admin(
    ctx: Context<RemoveAdmin>,
    store_id: Pubkey,
    admin_pubkey: Pubkey,
) -> Result<()> {
    let store = &mut ctx.accounts.store;
    let signer = ctx.accounts.owner.key();
    require!(
        has_role(store, &signer, AdminRoleType::Owner),
        CustomError::Unauthorized
    );
    if admin_pubkey == store.owner {
        return Err(CustomError::CannotRemoveOwner.into());
    }
    store
        .admin_roles
        .retain(|role| role.admin_pubkey != admin_pubkey);
    emit!(AdminRemoved {
        store_id,
        admin_pubkey,
        removed_at: Clock::get()?.unix_timestamp,
    });
    Ok(())
}

// Store events
#[event]
pub struct StoreRegistered {
    pub store_id: Pubkey,
    pub owner: Pubkey,
    pub name: String,
    pub created_at: i64,
}

#[event]
pub struct StoreUpdated {
    pub store_id: Pubkey,
    pub updated_by: Pubkey,
    pub updated_at: i64,
}

#[event]
pub struct AdminAdded {
    pub store_id: Pubkey,
    pub admin_pubkey: Pubkey,
    pub role_type: AdminRoleType,
    pub added_at: i64,
}

#[event]
pub struct AdminRemoved {
    pub store_id: Pubkey,
    pub admin_pubkey: Pubkey,
    pub removed_at: i64,
}

// Store/admin accounts
#[account]
pub struct Store {
    pub owner: Pubkey,
    pub name: String,
    pub description: String,
    pub logo_uri: String,
    pub created_at: i64,
    pub revenue: u64,
    pub loyalty_config: LoyaltyConfig,
    pub admin_roles: Vec<AdminRole>,
    pub is_active: bool,
}

impl Store {
    pub const LEN: usize = 32 + 4 + 200 + 4 + 200 + 8 + 8 + 1 + 4 + (32 + 1) * 5 + 1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct AdminRole {
    pub admin_pubkey: Pubkey,
    pub role_type: AdminRoleType,
}

// Context structs for store/admin instructions
#[derive(Accounts)]
#[instruction(store_id: Pubkey)]
pub struct RegisterStore<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + Store::LEN,
        seeds = [b"store", store_id.as_ref()],
        bump
    )]
    pub store: Account<'info, Store>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(store_id: Pubkey)]
pub struct UpdateStore<'info> {
    #[account(
        mut,
        seeds = [b"store", store_id.as_ref()],
        bump,
        constraint = store.owner == owner.key() || store.admin_roles.iter().any(|role| role.admin_pubkey == owner.key())
    )]
    pub store: Account<'info, Store>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(store_id: Pubkey)]
pub struct AddAdmin<'info> {
    #[account(
        mut,
        seeds = [b"store", store_id.as_ref()],
        bump,
        constraint = store.owner == owner.key()
    )]
    pub store: Account<'info, Store>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(store_id: Pubkey)]
pub struct RemoveAdmin<'info> {
    #[account(
        mut,
        seeds = [b"store", store_id.as_ref()],
        bump,
        constraint = store.owner == owner.key()
    )]
    pub store: Account<'info, Store>,
    pub owner: Signer<'info>,
}

pub fn has_role(store: &Store, user: &Pubkey, role: AdminRoleType) -> bool {
    store
        .admin_roles
        .iter()
        .any(|r| r.admin_pubkey == *user && r.role_type == role)
}
