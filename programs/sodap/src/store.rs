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
    store.owner = ctx.accounts.authority.key();
    store.name = name;
    store.description = description;
    store.logo_uri = logo_uri;
    store.loyalty_config = loyalty_config;
    store.is_active = true;
    store.revenue = 0;
    store.admin_roles = vec![AdminRole {
        admin_pubkey: ctx.accounts.authority.key(),
        role_type: AdminRoleType::Owner,
    }];

    emit!(StoreRegistered {
        store_id,
        owner: ctx.accounts.authority.key(),
        name: store.name.clone(),
        created_at: Clock::get().unwrap().unix_timestamp,
    });

    Ok(())
}

pub fn update_store(
    ctx: Context<UpdateStore>,
    _store_id: Pubkey,
    name: Option<String>,
    description: Option<String>,
    logo_uri: Option<String>,
    loyalty_config: Option<LoyaltyConfig>,
) -> Result<()> {
    let store = &mut ctx.accounts.store;

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
        store_id: ctx.accounts.store.key(),
        updated_by: ctx.accounts.owner.key(),
        updated_at: Clock::get().unwrap().unix_timestamp,
    });

    Ok(())
}

pub fn add_admin(
    ctx: Context<AddAdmin>,
    _store_id: Pubkey,
    admin_pubkey: Pubkey,
    role_type: AdminRoleType,
) -> Result<()> {
    let store = &mut ctx.accounts.store;

    if store
        .admin_roles
        .iter()
        .any(|r| r.admin_pubkey == admin_pubkey)
    {
        return Err(CustomError::AdminAlreadyExists.into());
    }

    store.admin_roles.push(AdminRole {
        admin_pubkey,
        role_type: role_type.clone(),
    });

    emit!(AdminAdded {
        store_id: ctx.accounts.store.key(),
        admin_pubkey,
        role_type,
        added_at: Clock::get().unwrap().unix_timestamp,
    });

    Ok(())
}

pub fn remove_admin(
    ctx: Context<RemoveAdmin>,
    _store_id: Pubkey,
    admin_pubkey: Pubkey,
) -> Result<()> {
    let store = &mut ctx.accounts.store;

    if !store
        .admin_roles
        .iter()
        .any(|r| r.admin_pubkey == admin_pubkey)
    {
        return Err(CustomError::AdminNotFound.into());
    }

    store
        .admin_roles
        .retain(|role| role.admin_pubkey != admin_pubkey);

    emit!(AdminRemoved {
        store_id: ctx.accounts.store.key(),
        admin_pubkey,
        removed_at: Clock::get().unwrap().unix_timestamp,
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
    pub loyalty_config: LoyaltyConfig,
    pub is_active: bool,
    pub revenue: u64,
    pub admin_roles: Vec<AdminRole>,
}

impl Store {
    pub const LEN: usize =
        8 + 32 + (4 + 200) + (4 + 500) + (4 + 200) + 16 + 1 + 8 + (4 + (33 * 10));
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
        payer = authority,
        space = Store::LEN,
        seeds = [b"store", store_id.as_ref()],
        bump
    )]
    pub store: Account<'info, Store>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(store_id: Pubkey)]
pub struct UpdateStore<'info> {
    #[account(
        mut,
        seeds = [b"store", store_id.as_ref()],
        bump,
        has_one = owner
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
        has_one = owner
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
        has_one = owner
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
