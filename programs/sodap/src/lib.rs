use anchor_lang::prelude::*;

pub mod admin;
pub mod error;
pub mod loyalty;
pub mod product;
pub mod store;
pub mod types;
pub mod user;
pub mod utils;

declare_id!("4eLJ3QGiNrPN6UUr2fNxq6tUZqFdBMVpXkL2MhsKNriv");

#[program]
pub mod sodap {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    // Delegate to modules
    pub use admin::*;
    pub use loyalty::*;
    pub use product::*;
    pub use store::*;
    pub use user::*;
}
