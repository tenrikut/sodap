use super::store::Store;
use crate::error::CustomError;
use crate::types::{AnomalyFlag, TokenizedType, TransactionStatus};
use anchor_lang::prelude::*;
use anchor_lang::{prelude::*, system_program};

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

pub fn purchase_cart(
    ctx: Context<PurchaseCart>,
    product_uuids: Vec<[u8; 16]>,
    quantities: Vec<u64>,
    gas_fee: u64,
) -> Result<()> {
    // -------- 1. sanity checks ------------------------------------------------
    require!(
        product_uuids.len() == quantities.len() && !product_uuids.is_empty(),
        CustomError::InvalidCart
    );

    // remaining_accounts should contain each Product PDA in the same order
    let mut acc_iter = ctx.remaining_accounts.iter();
    let mut calculated_total: u64 = 0;

    for (uuid, qty) in product_uuids.iter().zip(quantities.iter()) {
        let acc_info = acc_iter.next().ok_or(CustomError::InvalidCart)?; // fewer accounts than items
        let product: Account<Product> = Account::try_from(acc_info)?;
        // seed check (optional but safer)
        let expected_seed = Pubkey::create_program_address(
            &[
                b"product",
                ctx.accounts.store.key().as_ref(),
                uuid,
                &[product.bump],
            ],
            ctx.program_id,
        )?;
        require!(expected_seed == product.key(), CustomError::ProductNotFound);

        // stock + activity validation
        require!(product.is_active, CustomError::ProductNotFound);
        require!(product.stock >= *qty, CustomError::InsufficientStock);

        calculated_total = calculated_total
            .checked_add(
                product
                    .price
                    .checked_mul(*qty)
                    .ok_or(CustomError::PriceOverflow)?,
            )
            .ok_or(CustomError::PriceOverflow)?;
    }

    // -------- 2. collect lamports --------------------------------------------
    let buyer = &ctx.accounts.buyer;
    let store_owner = &ctx.accounts.store_owner;
    let transfer_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: buyer.to_account_info(),
            to: store_owner.to_account_info(),
        },
    );
    system_program::transfer(transfer_ctx, calculated_total)?;

    // -------- 3. write receipt -----------------------------------------------
    let purchase = &mut ctx.accountsreceipt;
    purchase.store = ctx.accounts.store.key();
    purchase.buyer = buyer.key();
    purchase.product_uuids = product_uuids.clone();
    purchase.quantities = quantities.clone();
    purchase.total_paid = calculated_total;
    purchase.gas_fee = gas_fee;
    purchase.status = TransactionStatus::Completed;
    purchase.anomaly = AnomalyFlag::None;
    purchase.ts = Clock::get()?.unix_timestamp;

    // -------- 4. emit event ---------------------------------------------------
    emit!(CartPurchased {
        store_id: purchase.store,
        buyer_id: purchase.buyer,
        product_uuids,
        quantities,
        total_paid: calculated_total,
        gas_fee,
        timestamp: purchase.ts,
    });

    Ok(())
}

/// off‑chain log
#[event]
pub struct CartPurchased {
    pub store_id: Pubkey,
    pub buyer_id: Pubkey,
    pub product_uuids: Vec<[u8; 16]>,
    pub quantities: Vec<u64>,
    pub total_paid: u64,
    pub gas_fee: u64,
    pub timestamp: i64,
}
