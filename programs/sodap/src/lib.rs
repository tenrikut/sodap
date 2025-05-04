mod admin;
mod cart;
mod errors;
mod loyalty;
mod product;
mod store;
mod types;
mod user;

use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, initialize_mint, InitializeMint, Mint, MintTo, Token, TokenAccount};

// Declare the program ID
// This ID is used to identify the deployed program on the Solana blockchain
declare_id!("4eLJ3QGiNrPN6UUr2fNxq6tUZqFdBMVpXkL2MhsKNriv");

// Main program module
#[program]
pub mod sodap {
    use super::*;
    // All logic is now in the respective modules
}
