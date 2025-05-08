// Helper functions (has_role, is_super_root_admin, check_root_password, is_platform_admin, etc.) will be placed here.
use anchor_lang::prelude::*;

// Utility functions
pub fn check_root_password(
    username: &str,
    password: &str,
    admin_username: &str,
    admin_password: &str,
) -> bool {
    // This is just a placeholder - in production you would use proper auth
    username == admin_username && password == admin_password
}

pub fn is_super_root_admin(key: &Pubkey, super_admin_pubkey: &Pubkey) -> bool {
    // Check if the key matches the super admin key
    key == super_admin_pubkey
}
