use crate::types::{AnomalyFlag, TokenizedType, TransactionStatus};
use anchor_lang::prelude::*;

use crate::state::product::{DeactivateProduct, PurchaseCart, RegisterProduct, UpdateProduct};
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
