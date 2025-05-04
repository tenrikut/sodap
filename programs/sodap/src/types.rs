// Shared enums and structs (TokenizedType, MintStatus, TransactionStatus, AnomalyFlag, LoyaltyConfig, AdminRoleType, etc.) will be placed here.

use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct LoyaltyConfig {
    pub points_per_dollar: u64,
    pub minimum_purchase: u64,
    pub reward_percentage: u64,
    pub is_active: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum AdminRoleType {
    SuperRootAdmin,
    PlatformAdmin,
    Owner,
    Manager,
    Cashier,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum TokenizedType {
    None,
    SplToken,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum MintStatus {
    NotMinted,
    Minted,
    Failed,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum TransactionStatus {
    Success,
    Failed,
    Pending,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum AnomalyFlag {
    HighValue,
    MultiplePurchases,
    UnusualTime,
    Other,
}
