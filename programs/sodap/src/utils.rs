// Helper functions (has_role, is_super_root_admin, check_root_password, is_platform_admin, etc.) will be placed here.

use crate::admin::PlatformAdmins;
use anchor_lang::prelude::*;

pub fn is_super_root_admin(signer: &Pubkey) -> bool {
    *signer == Pubkey::new_from_array([0u8; 32])
}

pub fn check_root_password(username: &str, password: &str) -> bool {
    username == "admin" && password == "password"
}

pub fn is_platform_admin(signer: &Pubkey, admins: &PlatformAdmins) -> bool {
    admins.admins.contains(signer)
}
