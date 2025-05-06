// Submodules for instruction handlers
pub mod admin;
pub mod loyalty;
pub mod product;
pub mod store;
pub mod user;

// Re-export for easier use in lib.rs
pub use admin::*;
pub use loyalty::*;
pub use product::*;
pub use store::*;
pub use user::*;
