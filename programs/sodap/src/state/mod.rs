// Submodules for on-chain accounts and context structs
pub mod admin;
pub mod error;
pub mod loyalty;
pub mod product;
pub mod store;
pub mod user;

// Re-export all relevant structs and context types
pub use admin::*;
pub use error::*;
pub use loyalty::*;
pub use product::*;
pub use store::Store;
pub use store::*;
pub use user::*;
