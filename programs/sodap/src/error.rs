// Please add your error types here, or move the content from state/error.rs

use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Invalid price")]
    InvalidPrice,
    #[msg("Invalid stock")]
    InvalidStock,
    #[msg("Product is out of stock")]
    OutOfStock,
    #[msg("Insufficient payment")]
    InsufficientPayment,
    #[msg("Stock underflow")]
    StockUnderflow,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Cart is empty")]
    CartEmpty,
    #[msg("Invalid cart (mismatched product and quantity arrays)")]
    InvalidCart,
    #[msg("Product not found")]
    ProductNotFound,
    #[msg("Insufficient stock")]
    InsufficientStock,
    #[msg("Price overflow when summing cart")]
    PriceOverflow,
    #[msg("Cart too large")]
    CartTooLarge,
    #[msg("Admin already exists")]
    AdminAlreadyExists,
    #[msg("Cannot remove owner")]
    CannotRemoveOwner,
    #[msg("Store not found")]
    StoreNotFound,
    #[msg("Unauthorized store access")]
    UnauthorizedStoreAccess,
    #[msg("Invalid loyalty configuration")]
    InvalidLoyaltyConfig,
    #[msg("Store is inactive")]
    StoreInactive,
    #[msg("Insufficient loyalty points")]
    InsufficientLoyaltyPoints,
    #[msg("Loyalty program is inactive")]
    LoyaltyProgramInactive,
    #[msg("Arithmetic error")]
    ArithmeticError,
    #[msg("Invalid metadata URI")]
    InvalidMetadataUri,
    #[msg("Admin not found")]
    AdminNotFound,
}
