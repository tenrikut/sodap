use crate::error::CustomError;
use crate::store::{has_role, Store};
use crate::types::{AdminRoleType, AnomalyFlag, MintStatus, TokenizedType, TransactionStatus};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{MintTo, Token};

// Product instructions
// (register_product, purchase_product, update_product, deactivate_product, purchase_cart)
// Product events
// (ProductRegistered, ProductPurchased, ProductUpdated, ProductDeactivated, CartPurchased)
// Product accounts
// (Product, Cart, CartProduct, RegisterProduct, PurchaseProduct, UpdateProduct, DeactivateProduct, PurchaseCart)

// (Move the actual code from tools.rs here in the next step)

pub fn register_product(
    ctx: Context<RegisterProduct>,
    _product_uuid: [u8; 16],
    price: u64,
    stock: u64,
    tokenized_type: TokenizedType,
    metadata_uri: String,
) -> Result<()> {
    let product = &mut ctx.accounts.product;
    let store_account = &ctx.accounts.store_account;
    let signer = ctx.accounts.store.key();
    require!(
        has_role(store_account, &signer, AdminRoleType::Owner)
            || has_role(store_account, &signer, AdminRoleType::Manager),
        CustomError::Unauthorized
    );
    require!(price > 0, CustomError::InvalidPrice);
    require!(stock > 0, CustomError::InvalidStock);
    require!(store_account.is_active, CustomError::StoreInactive);
    product.product_uuid = _product_uuid;
    product.store_id = ctx.accounts.store.key();
    product.price = price;
    product.stock = stock;
    product.tokenized_type = tokenized_type.clone();
    product.metadata_uri = metadata_uri.clone();
    product.created_at = Clock::get()?.unix_timestamp;
    product.mint_status = MintStatus::NotMinted;
    if tokenized_type == TokenizedType::SplToken {
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.product_mint_authority.to_account_info(),
        };
        let bump = ctx.bumps.product_mint_authority;
        let authority_seeds = &[b"product_mint", _product_uuid.as_ref(), &[bump]];
        let signer_seeds: &[&[&[u8]]] = &[&authority_seeds[..]];
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        anchor_spl::token::mint_to(cpi_ctx, 1)?;
        product.mint_status = MintStatus::Minted;
    }
    emit!(ProductRegistered {
        product_uuid: product.product_uuid,
        store_id: ctx.accounts.store.key(),
        pda_address: product.key(),
        price,
        stock,
        tokenized_type: tokenized_type.clone(),
        created_at: product.created_at,
        mint_status: product.mint_status.clone(),
    });
    require!(
        metadata_uri.starts_with("https://") || metadata_uri.starts_with("ipfs://"),
        CustomError::InvalidMetadataUri
    );
    Ok(())
}

pub fn purchase_product(
    ctx: Context<PurchaseProduct>,
    _product_uuid: [u8; 16],
    amount_paid: u64,
    gas_fee: u64,
    status: TransactionStatus,
    anomaly_flag: Option<AnomalyFlag>,
) -> Result<()> {
    let product = &mut ctx.accounts.product;
    let buyer = &ctx.accounts.buyer;
    require!(product.stock > 0, CustomError::OutOfStock);
    require!(
        amount_paid >= product.price,
        CustomError::InsufficientPayment
    );
    product.stock = product
        .stock
        .checked_sub(1)
        .ok_or(CustomError::StockUnderflow)?;
    let store = &ctx.accounts.store;
    let transfer_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        anchor_lang::system_program::Transfer {
            from: buyer.to_account_info(),
            to: store.to_account_info(),
        },
    );
    anchor_lang::system_program::transfer(transfer_ctx, product.price)?;
    emit!(ProductPurchased {
        product_uuid: product.product_uuid,
        store_id: product.store_id,
        buyer_id: buyer.key(),
        amount_paid,
        gas_fee,
        status: status.clone(),
        anomaly_flag: anomaly_flag.clone(),
        timestamp: Clock::get()?.unix_timestamp,
    });
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
    let store_account = &ctx.accounts.store_account;
    let signer = ctx.accounts.store.key();
    require!(
        has_role(store_account, &signer, AdminRoleType::Owner)
            || has_role(store_account, &signer, AdminRoleType::Manager),
        CustomError::Unauthorized
    );
    require!(store_account.is_active, CustomError::StoreInactive);
    require!(
        product.store_id == ctx.accounts.store.key(),
        CustomError::Unauthorized
    );
    if let Some(price) = new_price {
        require!(price > 0, CustomError::InvalidPrice);
        product.price = price;
    }
    if let Some(stock) = new_stock {
        product.stock = stock;
    }
    if let Some(uri) = new_metadata_uri {
        product.metadata_uri = uri.clone();
    }
    if let Some(tokenized_type) = new_tokenized_type {
        product.tokenized_type = tokenized_type;
    }
    emit!(ProductUpdated {
        product_uuid: product.product_uuid,
        store_id: product.store_id,
        price: product.price,
        stock: product.stock,
        tokenized_type: product.tokenized_type.clone(),
        metadata_uri: product.metadata_uri.clone(),
        updated_at: Clock::get()?.unix_timestamp,
    });
    Ok(())
}

pub fn deactivate_product(ctx: Context<DeactivateProduct>, _product_uuid: [u8; 16]) -> Result<()> {
    let product = &mut ctx.accounts.product;
    let store_account = &ctx.accounts.store_account;
    let signer = ctx.accounts.store.key();
    require!(
        has_role(store_account, &signer, AdminRoleType::Owner)
            || has_role(store_account, &signer, AdminRoleType::Manager),
        CustomError::Unauthorized
    );
    require!(store_account.is_active, CustomError::StoreInactive);
    require!(
        product.store_id == ctx.accounts.store.key(),
        CustomError::Unauthorized
    );
    product.deactivated = true;
    emit!(ProductDeactivated {
        product_uuid: product.product_uuid,
        store_id: product.store_id,
        deactivated_at: Clock::get()?.unix_timestamp,
    });
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
    require!(!product_uuids.is_empty(), CustomError::CartEmpty);
    require!(
        product_uuids.len() == quantities.len(),
        CustomError::InvalidCart
    );
    require!(product_uuids.len() <= 5, CustomError::CartTooLarge);
    let buyer = &ctx.accounts.buyer;
    let store = &ctx.accounts.store;
    let store_account = &mut ctx.accounts.store_account;
    let cart = &mut ctx.accounts.cart;
    let mut total_price: u64 = 0;
    cart.buyer = buyer.key();
    cart.store = store.key();
    cart.products = Vec::new();
    cart.created_at = Clock::get()?.unix_timestamp;
    for (i, product_uuid) in product_uuids.iter().enumerate() {
        let product = match i {
            0 => ctx
                .accounts
                .product1
                .as_mut()
                .ok_or(CustomError::ProductNotFound)?,
            1 => ctx
                .accounts
                .product2
                .as_mut()
                .ok_or(CustomError::ProductNotFound)?,
            2 => ctx
                .accounts
                .product3
                .as_mut()
                .ok_or(CustomError::ProductNotFound)?,
            3 => ctx
                .accounts
                .product4
                .as_mut()
                .ok_or(CustomError::ProductNotFound)?,
            4 => ctx
                .accounts
                .product5
                .as_mut()
                .ok_or(CustomError::ProductNotFound)?,
            _ => unreachable!(),
        };
        let quantity = quantities[i];
        require!(product.stock >= quantity, CustomError::InsufficientStock);
        total_price = total_price
            .checked_add(
                product
                    .price
                    .checked_mul(quantity)
                    .ok_or(CustomError::PriceOverflow)?,
            )
            .ok_or(CustomError::PriceOverflow)?;
        product.stock = product
            .stock
            .checked_sub(quantity)
            .ok_or(CustomError::StockUnderflow)?;
        cart.products.push(CartProduct {
            product_uuid: *product_uuid,
            quantity,
        });
    }
    cart.total_price = total_price;
    require!(
        total_amount_paid >= total_price,
        CustomError::InsufficientPayment
    );
    let transfer_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        anchor_lang::system_program::Transfer {
            from: buyer.to_account_info(),
            to: store.to_account_info(),
        },
    );
    anchor_lang::system_program::transfer(transfer_ctx, total_price)?;
    store_account.revenue = store_account
        .revenue
        .checked_add(total_price)
        .ok_or(CustomError::PriceOverflow)?;
    emit!(CartPurchased {
        product_uuids,
        quantities,
        store_id: store.key(),
        buyer_id: buyer.key(),
        total_amount_paid,
        gas_fee,
        status: status.clone(),
        anomaly_flag: anomaly_flag.clone(),
        timestamp: Clock::get()?.unix_timestamp,
    });
    Ok(())
}

// Product events
#[event]
pub struct ProductRegistered {
    pub product_uuid: [u8; 16],
    pub store_id: Pubkey,
    pub pda_address: Pubkey,
    pub price: u64,
    pub stock: u64,
    pub tokenized_type: TokenizedType,
    pub created_at: i64,
    pub mint_status: MintStatus,
}

#[event]
pub struct ProductPurchased {
    pub product_uuid: [u8; 16],
    pub store_id: Pubkey,
    pub buyer_id: Pubkey,
    pub amount_paid: u64,
    pub gas_fee: u64,
    pub status: TransactionStatus,
    pub anomaly_flag: Option<AnomalyFlag>,
    pub timestamp: i64,
}

#[event]
pub struct ProductUpdated {
    pub product_uuid: [u8; 16],
    pub store_id: Pubkey,
    pub price: u64,
    pub stock: u64,
    pub tokenized_type: TokenizedType,
    pub metadata_uri: String,
    pub updated_at: i64,
}

#[event]
pub struct ProductDeactivated {
    pub product_uuid: [u8; 16],
    pub store_id: Pubkey,
    pub deactivated_at: i64,
}

#[event]
pub struct CartPurchased {
    pub product_uuids: Vec<[u8; 16]>,
    pub quantities: Vec<u64>,
    pub store_id: Pubkey,
    pub buyer_id: Pubkey,
    pub total_amount_paid: u64,
    pub gas_fee: u64,
    pub status: TransactionStatus,
    pub anomaly_flag: Option<AnomalyFlag>,
    pub timestamp: i64,
}

// Product accounts
#[account]
pub struct Product {
    pub product_uuid: [u8; 16],
    pub store_id: Pubkey,
    pub price: u64,
    pub stock: u64,
    pub tokenized_type: TokenizedType,
    pub metadata_uri: String,
    pub created_at: i64,
    pub mint_status: MintStatus,
    pub deactivated: bool,
}

impl Product {
    pub const LEN: usize = 16 + 32 + 8 + 8 + 1 + 4 + 200 + 8 + 1 + 1;
}

#[account]
pub struct Cart {
    pub buyer: Pubkey,
    pub store: Pubkey,
    pub products: Vec<CartProduct>,
    pub total_price: u64,
    pub created_at: i64,
}

impl Cart {
    pub const LEN: usize = 32 + 32 + 4 + (16 + 8) * 5 + 8 + 8;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CartProduct {
    pub product_uuid: [u8; 16],
    pub quantity: u64,
}

// Product account contexts
#[derive(Accounts)]
#[instruction(product_uuid: [u8; 16])]
pub struct RegisterProduct<'info> {
    #[account(init, payer = store, space = 8 + Product::LEN, seeds = [b"product", product_uuid.as_ref()], bump)]
    pub product: Account<'info, Product>,
    #[account(mut)]
    pub store: Signer<'info>,
    #[account(
        seeds = [b"store", store.key().as_ref()],
        bump,
        constraint = store_account.owner == store.key() || store_account.admin_roles.iter().any(|role| role.admin_pubkey == store.key())
    )]
    pub store_account: Account<'info, Store>,
    /// CHECK: Only used if tokenized_type == SplToken
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    /// CHECK: Only used if tokenized_type == SplToken
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    /// CHECK: PDA for product mint authority
    #[account(
        seeds = [b"product_mint", product_uuid.as_ref()],
        bump
    )]
    pub product_mint_authority: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(product_uuid: [u8; 16])]
pub struct PurchaseProduct<'info> {
    #[account(mut, seeds = [b"product", product_uuid.as_ref()], bump)]
    pub product: Account<'info, Product>,
    /// CHECK: This is safe because we only transfer lamports to this account
    #[account(mut)]
    pub store: AccountInfo<'info>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(product_uuid: [u8; 16])]
pub struct UpdateProduct<'info> {
    #[account(mut, seeds = [b"product", product_uuid.as_ref()], bump)]
    pub product: Account<'info, Product>,
    #[account(mut)]
    pub store: Signer<'info>,
    #[account(
        seeds = [b"store", store.key().as_ref()],
        bump,
        constraint = store_account.owner == store.key() || store_account.admin_roles.iter().any(|role| role.admin_pubkey == store.key())
    )]
    pub store_account: Account<'info, Store>,
}

#[derive(Accounts)]
#[instruction(product_uuid: [u8; 16])]
pub struct DeactivateProduct<'info> {
    #[account(mut, seeds = [b"product", product_uuid.as_ref()], bump)]
    pub product: Account<'info, Product>,
    #[account(mut)]
    pub store: Signer<'info>,
    #[account(
        seeds = [b"store", store.key().as_ref()],
        bump,
        constraint = store_account.owner == store.key() || store_account.admin_roles.iter().any(|role| role.admin_pubkey == store.key())
    )]
    pub store_account: Account<'info, Store>,
}

#[derive(Accounts)]
#[instruction(product_uuids: Vec<[u8; 16]>, quantities: Vec<u64>)]
pub struct PurchaseCart<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    /// CHECK: This is safe because we only transfer lamports to this account
    #[account(mut)]
    pub store: AccountInfo<'info>,
    #[account(
        seeds = [b"store", store.key().as_ref()],
        bump,
        constraint = store_account.is_active
    )]
    pub store_account: Account<'info, Store>,
    #[account(
        init,
        payer = buyer,
        space = 8 + Cart::LEN,
        seeds = [b"cart", buyer.key().as_ref(), store.key().as_ref()],
        bump
    )]
    pub cart: Account<'info, Cart>,
    #[account(mut)]
    pub product1: Option<Account<'info, Product>>,
    #[account(mut)]
    pub product2: Option<Account<'info, Product>>,
    #[account(mut)]
    pub product3: Option<Account<'info, Product>>,
    #[account(mut)]
    pub product4: Option<Account<'info, Product>>,
    #[account(mut)]
    pub product5: Option<Account<'info, Product>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Initialize {}
