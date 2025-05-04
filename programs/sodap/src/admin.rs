// Admin and platform admin-related instructions, events, and accounts will be placed here.

use crate::error::CustomError;
use crate::utils::{check_root_password, is_super_root_admin};
use anchor_lang::prelude::*;

// Platform admin accounts
#[account]
pub struct PlatformAdmins {
    pub admins: Vec<Pubkey>,
}

impl PlatformAdmins {
    pub const LEN: usize = 4 + 32 * 10; // Up to 10 platform admins
}

// Platform admin events
#[event]
pub struct PlatformAdminAdded {
    pub admin_pubkey: Pubkey,
    pub added_at: i64,
}

#[event]
pub struct PlatformAdminRemoved {
    pub admin_pubkey: Pubkey,
    pub removed_at: i64,
}

// Platform admin instructions
pub fn add_platform_admin(
    ctx: Context<AddPlatformAdmin>,
    new_admin: Pubkey,
    username: String,
    password: String,
) -> Result<()> {
    let signer = ctx.accounts.signer.key();
    require!(is_super_root_admin(&signer), CustomError::Unauthorized);
    require!(
        check_root_password(&username, &password),
        CustomError::Unauthorized
    );
    let platform_admins = &mut ctx.accounts.platform_admins;
    if platform_admins.admins.contains(&new_admin) {
        return Err(CustomError::AdminAlreadyExists.into());
    }
    platform_admins.admins.push(new_admin);
    emit!(PlatformAdminAdded {
        admin_pubkey: new_admin,
        added_at: Clock::get()?.unix_timestamp,
    });
    Ok(())
}

pub fn remove_platform_admin(
    ctx: Context<RemovePlatformAdmin>,
    admin_pubkey: Pubkey,
    username: String,
    password: String,
) -> Result<()> {
    let signer = ctx.accounts.signer.key();
    require!(is_super_root_admin(&signer), CustomError::Unauthorized);
    require!(
        check_root_password(&username, &password),
        CustomError::Unauthorized
    );
    let platform_admins = &mut ctx.accounts.platform_admins;
    if !platform_admins.admins.contains(&admin_pubkey) {
        return Err(CustomError::AdminAlreadyExists.into());
    }
    platform_admins.admins.retain(|a| a != &admin_pubkey);
    emit!(PlatformAdminRemoved {
        admin_pubkey,
        removed_at: Clock::get()?.unix_timestamp,
    });
    Ok(())
}

#[derive(Accounts)]
pub struct AddPlatformAdmin<'info> {
    #[account(mut, seeds = [b"platform_admins"], bump)]
    pub platform_admins: Account<'info, PlatformAdmins>,
    /// CHECK: This is safe because we only check that the signer is the super root admin
    #[account(signer)]
    pub signer: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RemovePlatformAdmin<'info> {
    #[account(mut, seeds = [b"platform_admins"], bump)]
    pub platform_admins: Account<'info, PlatformAdmins>,
    /// CHECK: This is safe because we only check that the signer is the super root admin
    #[account(signer)]
    pub signer: AccountInfo<'info>,
}
