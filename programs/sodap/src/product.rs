use crate::store::Store;
use crate::types::{AnomalyFlag, TokenizedType, TransactionStatus};
use anchor_lang::prelude::*;

// Product instructions
pub fn register_product(
    ctx: Context<RegisterProduct>,
    product_uuid: [u8; 16],
    price: u64,
    stock: u64,
    tokenized_type: TokenizedType,
    metadata_uri: String,
) -> Result<()> {
    let product = &mut ctx.accounts.product;
    product.uuid = product_uuid;
    product.price = price;
    product.stock = stock;
    product.tokenized_type = tokenized_type;
    product.metadata_uri = metadata_uri;
    product.is_active = true;
    product.store = ctx.accounts.store.key();
    product.authority = ctx.accounts.authority.key();

    Ok(())
}

pub fn update_product(
    ctx: Context<UpdateProduct>,
    _product_uuid: [u8; 16],
    new_price: Option<u64>,
    new_stock: Option<u64>,
    new_metadata_uri: Option<String>,
    new_tokenized_type: Option<TokenizedType>,
) -> Result<()> {
    let product = &mut ctx.accounts.product;

    if let Some(price) = new_price {
        product.price = price;
    }

    if let Some(stock) = new_stock {
        product.stock = stock;
    }

    if let Some(metadata_uri) = new_metadata_uri {
        product.metadata_uri = metadata_uri;
    }

    if let Some(tokenized_type) = new_tokenized_type {
        product.tokenized_type = tokenized_type;
    }

    Ok(())
}

pub fn deactivate_product(ctx: Context<DeactivateProduct>, _product_uuid: [u8; 16]) -> Result<()> {
    let product = &mut ctx.accounts.product;
    product.is_active = false;
    Ok(())
}

pub fn purchase_cart(
    ctx: Context<PurchaseCart>,
    product_uuids: Vec<[u8; 16]>,
    quantities: Vec<u64>,
    total_amount_paid: u64,
    gas_fee: u64,
    status: TransactionStatus,
    anomaly_flag: Option<AnomalyFlag>,
) -> Result<()> {
    let purchase = &mut ctx.accounts.purchase;
    purchase.product_uuids = product_uuids;
    purchase.quantities = quantities;
    purchase.total_amount_paid = total_amount_paid;
    purchase.gas_fee = gas_fee;
    purchase.status = status;
    purchase.anomaly_flag = anomaly_flag;
    purchase.store = ctx.accounts.store.key();
    purchase.buyer = ctx.accounts.buyer.key();
    Ok(())
}

#[account]
pub struct Product {
    pub uuid: [u8; 16],
    pub price: u64,
    pub stock: u64,
    pub tokenized_type: TokenizedType,
    pub is_active: bool,
    pub metadata_uri: String,
    pub store: Pubkey,
    pub authority: Pubkey,
}

impl Product {
    pub const LEN: usize = 8 + 16 + 8 + 8 + 1 + (4 + 200) + 32 + 32;
}

#[account]
pub struct Purchase {
    pub product_uuids: Vec<[u8; 16]>,
    pub quantities: Vec<u64>,
    pub total_amount_paid: u64,
    pub gas_fee: u64,
    pub status: TransactionStatus,
    pub anomaly_flag: Option<AnomalyFlag>,
    pub store: Pubkey,
    pub buyer: Pubkey,
}

impl Purchase {
    pub const LEN: usize = 8 + (4 + 10 * 16) + (4 + 10 * 8) + 8 + 8 + 1 + 1 + 32 + 32;
}

#[derive(Accounts)]
#[instruction(product_uuid: [u8; 16])]
pub struct RegisterProduct<'info> {
    #[account(mut)]
    pub store: Account<'info, Store>,
    #[account(
        init,
        payer = authority,
        space = Product::LEN,
        seeds = [b"product", store.key().as_ref(), product_uuid.as_ref()],
        bump
    )]
    pub product: Account<'info, Product>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(product_uuid: [u8; 16])]
pub struct UpdateProduct<'info> {
    #[account(mut)]
    pub store: Account<'info, Store>,
    #[account(
        mut,
        seeds = [b"product", store.key().as_ref(), product_uuid.as_ref()],
        bump,
        has_one = store,
        has_one = authority
    )]
    pub product: Account<'info, Product>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(product_uuid: [u8; 16])]
pub struct DeactivateProduct<'info> {
    #[account(mut)]
    pub store: Account<'info, Store>,
    #[account(
        mut,
        seeds = [b"product", store.key().as_ref(), product_uuid.as_ref()],
        bump,
        has_one = store,
        has_one = authority
    )]
    pub product: Account<'info, Product>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct PurchaseCart<'info> {
    #[account(mut)]
    pub store: Account<'info, Store>,
    #[account(
        init,
        payer = buyer,
        space = Purchase::LEN,
        seeds = [b"purchase", store.key().as_ref(), buyer.key().as_ref()],
        bump
    )]
    pub purchase: Account<'info, Purchase>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
