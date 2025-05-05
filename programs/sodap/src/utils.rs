// Helper functions (has_role, is_super_root_admin, check_root_password, is_platform_admin, etc.) will be placed here.

use crate::admin::PlatformAdmins;
use anchor_lang::prelude::*;

pub fn is_super_root_admin(signer: &Pubkey, super_admin_pubkey: &Pubkey) -> bool {
    signer == super_admin_pubkey
}

pub fn check_root_password(
    username: &str,
    password: &str,
    expected_username: &str,
    expected_password: &str,
) -> bool {
    username == expected_username && password == expected_password
}

pub fn is_platform_admin(signer: &Pubkey, admins: &PlatformAdmins) -> bool {
    admins.admins.contains(signer)
}
