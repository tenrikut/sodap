use crate::error::CustomError;
use crate::state::product::{
    CartPurchased, DeactivateProduct, Product, PurchaseCart, RegisterProduct, UpdateProduct,
};
use crate::types::{TokenizedType, TransactionStatus};
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

/// Validate product cart items against remaining accounts and calculate total
fn validate_cart_and_payment<'a, 'b>(
    product_uuids: &'a [[u8; 16]],
    quantities: &'a [u64],
    remaining_accounts: &'b [AccountInfo<'b>],
    total_amount_paid: u64,
) -> Result<u64> {
    require!(
        product_uuids.len() == quantities.len() && !product_uuids.is_empty(),
        CustomError::InvalidCart
    );

    let mut total_price = 0u64;
    let mut i = 0;
    while i < product_uuids.len() {
        let acc_info = &remaining_accounts[i];
        let product = Account::<Product>::try_from(acc_info)?;
        require!(
            product.uuid == product_uuids[i],
            CustomError::ProductNotFound
        );
        require!(product.is_active, CustomError::ProductNotFound);
        require!(
            product.stock >= quantities[i],
            CustomError::InsufficientStock
        );

        // Calculate price for this item
        let item_total = product
            .price
            .checked_mul(quantities[i])
            .ok_or(CustomError::ArithmeticError)?;
        total_price = total_price
            .checked_add(item_total)
            .ok_or(CustomError::ArithmeticError)?;

        i += 1;
    }

    // Verify payment amount matches cart total
    require!(
        total_amount_paid >= total_price,
        CustomError::InsufficientPayment
    );

    Ok(total_price)
}

pub fn purchase_cart<'info>(
    ctx: Context<'_, '_, 'info, 'info, PurchaseCart>,
    product_uuids: Vec<[u8; 16]>,
    quantities: Vec<u64>,
    total_amount_paid: u64,
    gas_fee: u64,
    status: TransactionStatus,
) -> Result<()> {
    let remaining_accounts: &'info [AccountInfo<'info>] = ctx.remaining_accounts;

    // Validate cart and get total price
    let total_price = validate_cart_and_payment(
        &product_uuids,
        &quantities,
        remaining_accounts,
        total_amount_paid,
    )?;

    // Transfer payment from buyer to store owner
    let transfer_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        anchor_lang::system_program::Transfer {
            from: ctx.accounts.buyer.to_account_info(),
            to: ctx.accounts.store_owner.to_account_info(),
        },
    );
    anchor_lang::system_program::transfer(transfer_ctx, total_price)?;

    // Update product stocks
    let mut i = 0;
    while i < product_uuids.len() {
        let product_account = &mut Account::<Product>::try_from(&remaining_accounts[i])?;
        product_account.stock = product_account
            .stock
            .checked_sub(quantities[i])
            .ok_or(CustomError::StockUnderflow)?;
        i += 1;
    }

    // Create receipt
    let receipt = &mut ctx.accounts.receipt;
    receipt.product_uuids = product_uuids.clone();
    receipt.quantities = quantities.clone();
    receipt.total_paid = total_price;
    receipt.gas_fee = gas_fee;
    receipt.status = status;
    receipt.store = ctx.accounts.store.key();
    receipt.buyer = ctx.accounts.buyer.key();
    receipt.ts = Clock::get()?.unix_timestamp;

    // Emit purchase event
    emit!(CartPurchased {
        store_id: ctx.accounts.store.key(),
        buyer_id: ctx.accounts.buyer.key(),
        product_uuids: product_uuids,
        quantities: quantities,
        total_paid: total_price,
        gas_fee,
        timestamp: receipt.ts,
    });

    Ok(())
}
