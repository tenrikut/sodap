// Import necessary modules from the Anchor framework
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount, Transfer as SplTransfer};

// Declare the program ID
// This ID is used to identify the deployed program on the Solana blockchain
declare_id!("4eLJ3QGiNrPN6UUr2fNxq6tUZqFdBMVpXkL2MhsKNriv");

// Main program module
#[program]
pub mod sodap {
    use super::*;

    // Initialize the program
    // This function is called once when the program is deployed
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    // Register a new product
    // This function creates a new product account and initializes it with the provided details
    pub fn register_product(
        ctx: Context<RegisterProduct>,
        product_uuid: [u8; 16],
        price: u64,
        stock: u64,
        tokenized_type: TokenizedType,
        metadata_uri: String,
    ) -> Result<()> {
        let product = &mut ctx.accounts.product;
        let store_account = &ctx.accounts.store_account;

        // Ensure the price and stock are valid and the store is active
        require!(price > 0, CustomError::InvalidPrice);
        require!(stock > 0, CustomError::InvalidStock);
        require!(store_account.is_active, CustomError::StoreInactive);

        // Initialize product details
        product.product_uuid = product_uuid;
        product.store_id = ctx.accounts.store.key();
        product.price = price;
        product.stock = stock;
        product.tokenized_type = tokenized_type.clone();
        product.metadata_uri = metadata_uri;
        product.created_at = Clock::get()?.unix_timestamp;
        product.mint_status = MintStatus::NotMinted;

        // Mint SPL token if required and if we're not in test mode
        if tokenized_type == TokenizedType::SplToken {
            // In a real deployment, uncomment the following code:
            /*
            let cpi_accounts = MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.store.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            token::mint_to(CpiContext::new(cpi_program, cpi_accounts), 1)?;
            */
            product.mint_status = MintStatus::Minted;
        }

        // Emit event for product registration
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

        Ok(())
    }

    // Purchase a product
    // This function handles the purchase of a product, updating stock and transferring payment
    pub fn purchase_product(
        ctx: Context<PurchaseProduct>,
        product_uuid: [u8; 16],
        amount_paid: u64,
        gas_fee: u64,
        status: TransactionStatus,
        anomaly_flag: Option<AnomalyFlag>,
    ) -> Result<()> {
        let product = &mut ctx.accounts.product;
        let buyer = &ctx.accounts.buyer;

        // Verify the product exists and has stock
        require!(product.stock > 0, CustomError::OutOfStock);
        require!(
            amount_paid >= product.price,
            CustomError::InsufficientPayment
        );

        // Update product stock
        product.stock = product
            .stock
            .checked_sub(1)
            .ok_or(CustomError::StockUnderflow)?;

        // Transfer payment to store
        let store = &ctx.accounts.store;
        let transfer_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: buyer.to_account_info(),
                to: store.to_account_info(),
            },
        );
        anchor_lang::system_program::transfer(transfer_ctx, product.price)?;

        // Emit purchase event
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

    // Update product details
    // This function allows updating the price, stock, metadata URI, and tokenized type of a product
    pub fn update_product(
        ctx: Context<UpdateProduct>,
        product_uuid: [u8; 16],
        new_price: Option<u64>,
        new_stock: Option<u64>,
        new_metadata_uri: Option<String>,
        new_tokenized_type: Option<TokenizedType>,
    ) -> Result<()> {
        let product = &mut ctx.accounts.product;
        let store_account = &ctx.accounts.store_account;

        // Ensure the store is active and the caller is authorized
        require!(store_account.is_active, CustomError::StoreInactive);
        require!(
            product.store_id == ctx.accounts.store.key(),
            CustomError::Unauthorized
        );

        // Update product details if provided
        if let Some(price) = new_price {
            require!(price > 0, CustomError::InvalidPrice);
            product.price = price;
        }
        if let Some(stock) = new_stock {
            product.stock = stock;
        }
        if let Some(uri) = new_metadata_uri {
            product.metadata_uri = uri;
        }
        if let Some(tokenized_type) = new_tokenized_type {
            product.tokenized_type = tokenized_type;
        }

        // Emit event for product update
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

    // Deactivate a product
    // This function marks a product as deactivated, preventing further purchases
    pub fn deactivate_product(
        ctx: Context<DeactivateProduct>,
        product_uuid: [u8; 16],
    ) -> Result<()> {
        let product = &mut ctx.accounts.product;
        let store_account = &ctx.accounts.store_account;

        // Ensure the store is active and the caller is authorized
        require!(store_account.is_active, CustomError::StoreInactive);
        require!(
            product.store_id == ctx.accounts.store.key(),
            CustomError::Unauthorized
        );

        // Mark product as deactivated
        product.deactivated = true;

        // Emit event for product deactivation
        emit!(ProductDeactivated {
            product_uuid: product.product_uuid,
            store_id: product.store_id,
            deactivated_at: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    // Purchase products in a cart
    // This function processes the purchase of multiple products in a cart
    pub fn purchase_cart(
        ctx: Context<PurchaseCart>,
        product_uuids: Vec<[u8; 16]>,
        quantities: Vec<u64>,
        total_amount_paid: u64,
        gas_fee: u64,
        status: TransactionStatus,
        anomaly_flag: Option<AnomalyFlag>,
    ) -> Result<()> {
        // Verify cart is not empty and has valid quantities
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

        // Initialize cart details
        cart.buyer = buyer.key();
        cart.store = store.key();
        cart.products = Vec::new();
        cart.created_at = Clock::get()?.unix_timestamp;

        // Process each product in the cart
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

            // Verify product exists and has enough stock
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

    pub fn register_store(
        ctx: Context<RegisterStore>,
        store_id: Pubkey,
        name: String,
        description: String,
        logo_uri: String,
        loyalty_config: LoyaltyConfig,
    ) -> Result<()> {
        let store = &mut ctx.accounts.store;
        store.owner = ctx.accounts.owner.key();
        store.name = name;
        store.description = description;
        store.logo_uri = logo_uri;
        store.created_at = Clock::get()?.unix_timestamp;
        store.revenue = 0;
        store.loyalty_config = loyalty_config;
        store.admin_roles = vec![AdminRole {
            admin_pubkey: ctx.accounts.owner.key(),
            role_type: AdminRoleType::Owner,
        }];
        store.is_active = true;

        emit!(StoreRegistered {
            store_id,
            owner: ctx.accounts.owner.key(),
            name: store.name.clone(),
            created_at: store.created_at,
        });

        Ok(())
    }

    pub fn update_store(
        ctx: Context<UpdateStore>,
        store_id: Pubkey,
        name: Option<String>,
        description: Option<String>,
        logo_uri: Option<String>,
        loyalty_config: Option<LoyaltyConfig>,
    ) -> Result<()> {
        let store = &mut ctx.accounts.store;

        if let Some(name) = name {
            store.name = name;
        }
        if let Some(description) = description {
            store.description = description;
        }
        if let Some(logo_uri) = logo_uri {
            store.logo_uri = logo_uri;
        }
        if let Some(loyalty_config) = loyalty_config {
            store.loyalty_config = loyalty_config;
        }

        emit!(StoreUpdated {
            store_id,
            updated_by: ctx.accounts.owner.key(),
            updated_at: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    pub fn add_admin(
        ctx: Context<AddAdmin>,
        store_id: Pubkey,
        admin_pubkey: Pubkey,
        role_type: AdminRoleType,
    ) -> Result<()> {
        let store = &mut ctx.accounts.store;

        if store
            .admin_roles
            .iter()
            .any(|role| role.admin_pubkey == admin_pubkey)
        {
            return Err(CustomError::AdminAlreadyExists.into());
        }

        store.admin_roles.push(AdminRole {
            admin_pubkey,
            role_type: role_type.clone(),
        });

        emit!(AdminAdded {
            store_id,
            admin_pubkey,
            role_type,
            added_at: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    pub fn remove_admin(
        ctx: Context<RemoveAdmin>,
        store_id: Pubkey,
        admin_pubkey: Pubkey,
    ) -> Result<()> {
        let store = &mut ctx.accounts.store;

        if admin_pubkey == store.owner {
            return Err(CustomError::CannotRemoveOwner.into());
        }

        store
            .admin_roles
            .retain(|role| role.admin_pubkey != admin_pubkey);

        emit!(AdminRemoved {
            store_id,
            admin_pubkey,
            removed_at: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    // Create or update a user profile
    // This function creates a new user profile or updates an existing one
    pub fn create_or_update_user_profile(
        ctx: Context<CreateOrUpdateUserProfile>,
        user_id: Option<String>,
        delivery_address: Option<String>,
        preferred_store: Option<Pubkey>,
    ) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;
        let user = &ctx.accounts.user;

        // Initialize the user profile if it's new
        if user_profile.wallet_address != user.key() {
            user_profile.wallet_address = user.key();
            user_profile.user_id = user_id.unwrap_or_else(|| "".to_string());
            user_profile.registered_at = Clock::get()?.unix_timestamp;
            user_profile.loyalty_balance = 0;
            user_profile.purchase_history = Vec::new();
        } else {
            // Update optional fields if provided
            if let Some(id) = user_id {
                user_profile.user_id = id;
            }
        }

        // Update delivery address if provided
        if let Some(address) = delivery_address {
            user_profile.delivery_address = address;
        }

        // Update preferred store if provided
        if let Some(store) = preferred_store {
            user_profile.preferred_store = Some(store);
        }

        emit!(UserProfileUpdated {
            wallet_address: user_profile.wallet_address,
            user_id: user_profile.user_id.clone(),
            updated_at: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    // Scan and purchase items in a cart with loyalty rewards
    // This function processes a cart purchase and distributes loyalty rewards
    pub fn scan_and_purchase(
        ctx: Context<ScanAndPurchase>,
        product_uuids: Vec<[u8; 16]>,
        quantities: Vec<u64>,
        store_id: Pubkey,
    ) -> Result<()> {
        // Verify cart is not empty and has valid quantities
        require!(!product_uuids.is_empty(), CustomError::CartEmpty);
        require!(
            product_uuids.len() == quantities.len(),
            CustomError::InvalidCart
        );
        require!(product_uuids.len() <= 5, CustomError::CartTooLarge);

        let buyer = &ctx.accounts.buyer;
        let store = &ctx.accounts.store;
        let store_account = &mut ctx.accounts.store_account;
        let user_profile = &mut ctx.accounts.user_profile;
        let cart = &mut ctx.accounts.cart;
        let mut total_price: u64 = 0;

        // Initialize cart details
        cart.buyer = buyer.key();
        cart.store = store.key();
        cart.products = Vec::new();
        cart.created_at = Clock::get()?.unix_timestamp;

        // Process each product in the cart
        for (i, (product_uuid, quantity)) in product_uuids.iter().zip(quantities.iter()).enumerate()
        {
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
                _ => return Err(CustomError::CartTooLarge.into()),
            };

            // Verify the product belongs to the specified store
            require!(
                product.store_id == store.key(),
                CustomError::UnauthorizedStoreAccess
            );

            // Verify the product has enough stock
            require!(product.stock >= *quantity, CustomError::InsufficientStock);

            // Calculate product price
            let product_price = product
                .price
                .checked_mul(*quantity)
                .ok_or(CustomError::PriceOverflow)?;

            // Update product stock
            product.stock = product
                .stock
                .checked_sub(*quantity)
                .ok_or(CustomError::StockUnderflow)?;

            // Add product to cart
            cart.products.push(CartProduct {
                product_uuid: *product_uuid,
                quantity: *quantity,
            });

            // Update total price
            total_price = total_price
                .checked_add(product_price)
                .ok_or(CustomError::PriceOverflow)?;
        }

        // Set the cart's total price
        cart.total_price = total_price;

        // Transfer payment from buyer to store
        let transfer_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: buyer.to_account_info(),
                to: store.to_account_info(),
            },
        );
        anchor_lang::system_program::transfer(transfer_ctx, total_price)?;

        // Update store revenue
        store_account.revenue = store_account
            .revenue
            .checked_add(total_price)
            .ok_or(CustomError::PriceOverflow)?;

        // Calculate loyalty points (only if the store has loyalty and minimum purchase is met)
        let mut loyalty_points_earned: u64 = 0;
        if store_account.loyalty_config.is_active
            && total_price >= store_account.loyalty_config.minimum_purchase
        {
            // Convert SOL to USD (simplified - in a real app, would use an oracle)
            // Assuming 1 SOL = 100 USD for this example
            let usd_amount = total_price / 1_000_000_000 * 100; // Convert lamports to SOL, then to USD

            // Calculate loyalty points
            loyalty_points_earned = usd_amount
                .checked_mul(store_account.loyalty_config.points_per_dollar)
                .ok_or(CustomError::PriceOverflow)?;

            // Update user's loyalty balance
            user_profile.loyalty_balance = user_profile
                .loyalty_balance
                .checked_add(loyalty_points_earned)
                .ok_or(CustomError::PriceOverflow)?;
        }

        // Update user's purchase history
        user_profile.purchase_history.push(PurchaseRecord {
            store_id: store.key(),
            transaction_id: cart.key(),
            amount: total_price,
            loyalty_earned: loyalty_points_earned,
            timestamp: Clock::get()?.unix_timestamp,
        });

        // If preferred store is not set, set it to this store
        if user_profile.preferred_store.is_none() {
            user_profile.preferred_store = Some(store.key());
        }

        // Emit purchase event
        emit!(CartPurchased {
            product_uuids,
            quantities,
            store_id: store.key(),
            buyer_id: buyer.key(),
            total_amount_paid: total_price,
            gas_fee: 0, // Gas fee is handled by Solana and not directly trackable
            status: TransactionStatus::Success,
            anomaly_flag: None,
            timestamp: Clock::get()?.unix_timestamp,
        });

        // Emit loyalty event if points were earned
        if loyalty_points_earned > 0 {
            emit!(LoyaltyPointsEarned {
                user: buyer.key(),
                store_id: store.key(),
                points_earned: loyalty_points_earned,
                total_points: user_profile.loyalty_balance,
                timestamp: Clock::get()?.unix_timestamp,
            });
        }

        Ok(())
    }

    // Redeem loyalty points for a purchase discount
    // This function allows a user to redeem loyalty points for a discount on a purchase
    pub fn redeem_loyalty_points(
        ctx: Context<RedeemLoyaltyPoints>,
        points_to_redeem: u64,
    ) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;
        let store_account = &ctx.accounts.store_account;

        // Verify user has enough points
        require!(
            user_profile.loyalty_balance >= points_to_redeem,
            CustomError::InsufficientLoyaltyPoints
        );

        // Verify store loyalty program is active
        require!(
            store_account.loyalty_config.is_active,
            CustomError::LoyaltyProgramInactive
        );

        // Calculate discount value (simplified, real implementation would follow store's rules)
        // Here we're assuming 1 point = 0.01 SOL discount
        let discount_value = points_to_redeem * 10_000_000; // 0.01 SOL in lamports

        // Deduct points from user's balance
        user_profile.loyalty_balance = user_profile
            .loyalty_balance
            .checked_sub(points_to_redeem)
            .ok_or(CustomError::ArithmeticError)?;

        // Emit redemption event
        emit!(LoyaltyPointsRedeemed {
            user: ctx.accounts.user.key(),
            store_id: ctx.accounts.store.key(),
            points_redeemed: points_to_redeem,
            discount_value,
            remaining_points: user_profile.loyalty_balance,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    // Mint loyalty tokens as SPL tokens
    // This function mints SPL tokens to represent loyalty points
    pub fn mint_loyalty_tokens(ctx: Context<MintLoyaltyTokens>, amount: u64) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;

        // Verify the user has enough loyalty points
        require!(
            user_profile.loyalty_balance >= amount,
            CustomError::InsufficientLoyaltyPoints
        );

        // Mint SPL tokens
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };

        // Create CPI context with signer seeds
        let mint_authority_bump = ctx.bumps.mint_authority;
        let mint_authority_seeds: [&[u8]; 2] = [b"loyalty_mint", &[mint_authority_bump]];
        let signer_seeds: &[&[&[u8]]] = &[&mint_authority_seeds];
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        // Mint tokens to user
        token::mint_to(cpi_ctx, amount)?;

        // Deduct from loyalty balance
        user_profile.loyalty_balance = user_profile
            .loyalty_balance
            .checked_sub(amount)
            .ok_or(CustomError::ArithmeticError)?;

        // Emit event
        emit!(LoyaltyTokensMinted {
            user: ctx.accounts.user.key(),
            amount,
            remaining_points: user_profile.loyalty_balance,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
#[instruction(product_uuid: [u8; 16], price: u64, stock: u64, tokenized_type: TokenizedType, metadata_uri: String)]
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
    /// CHECK: This is safe because we verify the mint in the instruction logic or CPI
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    /// CHECK: This account is used only when tokenizedType is SplToken
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

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

#[account]
pub struct Cart {
    pub buyer: Pubkey,
    pub store: Pubkey,
    pub products: Vec<CartProduct>,
    pub total_price: u64,
    pub created_at: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CartProduct {
    pub product_uuid: [u8; 16],
    pub quantity: u64,
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

impl Cart {
    pub const LEN: usize = 32 + 32 + 4 + (16 + 8) * 5 + 8 + 8;
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

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum TokenizedType {
    None,
    SplToken,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum MintStatus {
    NotMinted,
    Minted,
    Failed,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum TransactionStatus {
    Success,
    Failed,
    Pending,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum AnomalyFlag {
    HighValue,
    MultiplePurchases,
    UnusualTime,
    Other,
}

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
    #[msg("Price overflow")]
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
}

#[account]
pub struct Store {
    pub owner: Pubkey,
    pub name: String,
    pub description: String,
    pub logo_uri: String,
    pub created_at: i64,
    pub revenue: u64,
    pub loyalty_config: LoyaltyConfig,
    pub admin_roles: Vec<AdminRole>,
    pub is_active: bool,
}

impl Store {
    pub const LEN: usize = 32 + 4 + 200 + 4 + 200 + 8 + 8 + 1 + 4 + (32 + 1) * 5 + 1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct AdminRole {
    pub admin_pubkey: Pubkey,
    pub role_type: AdminRoleType,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum AdminRoleType {
    SuperRootAdmin,
    PlatformAdmin,
    Owner,
    Manager,
    Cashier,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct LoyaltyConfig {
    pub points_per_dollar: u64,
    pub minimum_purchase: u64,
    pub reward_percentage: u64,
    pub is_active: bool,
}

#[derive(Accounts)]
#[instruction(store_id: Pubkey)]
pub struct RegisterStore<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + Store::LEN,
        seeds = [b"store", store_id.as_ref()],
        bump
    )]
    pub store: Account<'info, Store>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(store_id: Pubkey)]
pub struct UpdateStore<'info> {
    #[account(
        mut,
        seeds = [b"store", store_id.as_ref()],
        bump,
        constraint = store.owner == owner.key() || store.admin_roles.iter().any(|role| role.admin_pubkey == owner.key())
    )]
    pub store: Account<'info, Store>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(store_id: Pubkey, admin_pubkey: Pubkey)]
pub struct AddAdmin<'info> {
    #[account(
        mut,
        seeds = [b"store", store_id.as_ref()],
        bump,
        constraint = store.owner == owner.key()
    )]
    pub store: Account<'info, Store>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(store_id: Pubkey, admin_pubkey: Pubkey)]
pub struct RemoveAdmin<'info> {
    #[account(
        mut,
        seeds = [b"store", store_id.as_ref()],
        bump,
        constraint = store.owner == owner.key()
    )]
    pub store: Account<'info, Store>,
    pub owner: Signer<'info>,
}

#[event]
pub struct StoreRegistered {
    pub store_id: Pubkey,
    pub owner: Pubkey,
    pub name: String,
    pub created_at: i64,
}

#[event]
pub struct StoreUpdated {
    pub store_id: Pubkey,
    pub updated_by: Pubkey,
    pub updated_at: i64,
}

#[event]
pub struct AdminAdded {
    pub store_id: Pubkey,
    pub admin_pubkey: Pubkey,
    pub role_type: AdminRoleType,
    pub added_at: i64,
}

#[event]
pub struct AdminRemoved {
    pub store_id: Pubkey,
    pub admin_pubkey: Pubkey,
    pub removed_at: i64,
}

// ---------- NEW ACCOUNT STRUCTURES FOR SHOPPER PURCHASE FLOW ----------

#[account]
pub struct UserProfile {
    pub wallet_address: Pubkey,
    pub user_id: String, // Optional user identifier
    pub preferred_store: Option<Pubkey>,
    pub delivery_address: String, // Optional for physical deliveries
    pub loyalty_balance: u64,     // Loyalty points balance
    pub purchase_history: Vec<PurchaseRecord>,
    pub registered_at: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PurchaseRecord {
    pub store_id: Pubkey,
    pub transaction_id: Pubkey,
    pub amount: u64,
    pub loyalty_earned: u64,
    pub timestamp: i64,
}

impl UserProfile {
    // Max size for user profile - adjust these values as needed
    pub const MAX_USER_ID_LEN: usize = 50;
    pub const MAX_ADDRESS_LEN: usize = 200;
    pub const MAX_PURCHASE_HISTORY: usize = 10; // Store only the last 10 purchases

    pub const LEN: usize = 32 +                                     // wallet_address
        4 + Self::MAX_USER_ID_LEN +              // user_id (string)
        1 + 32 +                                 // preferred_store (Option<Pubkey>)
        4 + Self::MAX_ADDRESS_LEN +              // delivery_address
        8 +                                      // loyalty_balance
        4 + (32 + 32 + 8 + 8 + 8) * Self::MAX_PURCHASE_HISTORY + // purchase_history
        8; // registered_at
}

#[derive(Accounts)]
pub struct CreateOrUpdateUserProfile<'info> {
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + UserProfile::LEN,
        seeds = [b"user_profile", user.key().as_ref()],
        bump
    )]
    pub user_profile: Account<'info, UserProfile>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ScanAndPurchase<'info> {
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
    #[account(
        mut,
        seeds = [b"user_profile", buyer.key().as_ref()],
        bump
    )]
    pub user_profile: Account<'info, UserProfile>,
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
pub struct RedeemLoyaltyPoints<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"user_profile", user.key().as_ref()],
        bump
    )]
    pub user_profile: Account<'info, UserProfile>,
    /// CHECK: This is safe because we're only checking its key
    pub store: AccountInfo<'info>,
    #[account(
        seeds = [b"store", store.key().as_ref()],
        bump,
        constraint = store_account.is_active && store_account.loyalty_config.is_active
    )]
    pub store_account: Account<'info, Store>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintLoyaltyTokens<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"user_profile", user.key().as_ref()],
        bump
    )]
    pub user_profile: Account<'info, UserProfile>,
    #[account(
        mut,
        constraint = mint.decimals == 0 // Loyalty tokens have 0 decimals
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user
    )]
    pub token_account: Account<'info, TokenAccount>,
    /// CHECK: We're using PDA for mint authority
    #[account(
        seeds = [b"loyalty_mint"],
        bump
    )]
    pub mint_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

// ---------- NEW EVENTS FOR SHOPPER PURCHASE FLOW ----------

#[event]
pub struct UserProfileUpdated {
    pub wallet_address: Pubkey,
    pub user_id: String,
    pub updated_at: i64,
}

#[event]
pub struct LoyaltyPointsEarned {
    pub user: Pubkey,
    pub store_id: Pubkey,
    pub points_earned: u64,
    pub total_points: u64,
    pub timestamp: i64,
}

#[event]
pub struct LoyaltyPointsRedeemed {
    pub user: Pubkey,
    pub store_id: Pubkey,
    pub points_redeemed: u64,
    pub discount_value: u64,
    pub remaining_points: u64,
    pub timestamp: i64,
}

#[event]
pub struct LoyaltyTokensMinted {
    pub user: Pubkey,
    pub amount: u64,
    pub remaining_points: u64,
    pub timestamp: i64,
}

// --- Super Root Admin Logic ---
const SUPER_ROOT_ADMIN_USERNAME: &str = "super-admin";
const SUPER_ROOT_ADMIN_PASSWORD: &str = "sodap*root";

// For demonstration, hardcode a public key for the super root admin (replace with your real key in production)
const SUPER_ROOT_ADMIN_PUBKEY: &str = "11111111111111111111111111111111"; // Replace with real pubkey

#[account]
pub struct PlatformAdmins {
    pub admins: Vec<Pubkey>,
}

impl PlatformAdmins {
    pub const LEN: usize = 4 + 32 * 10; // Up to 10 platform admins
}

// Utility function to check if signer is Super Root Admin
fn is_super_root_admin(signer: &Pubkey) -> bool {
    signer.to_string() == SUPER_ROOT_ADMIN_PUBKEY
}

// Utility function to check root password (for demonstration only)
fn check_root_password(username: &str, password: &str) -> bool {
    username == SUPER_ROOT_ADMIN_USERNAME && password == SUPER_ROOT_ADMIN_PASSWORD
}

#[derive(Accounts)]
pub struct AddPlatformAdmin<'info> {
    #[account(mut, seeds = [b"platform_admins"], bump)]
    pub platform_admins: Account<'info, PlatformAdmins>,
    #[account(signer)]
    pub signer: AccountInfo<'info>,
}

// Instruction: Add a Platform Admin (only callable by Super Root Admin)
pub fn add_platform_admin(
    ctx: Context<AddPlatformAdmin>,
    new_admin: Pubkey,
    username: String,
    password: String,
) -> Result<()> {
    require!(
        is_super_root_admin(ctx.accounts.signer.key),
        CustomError::Unauthorized
    );
    require!(
        check_root_password(&username, &password),
        CustomError::Unauthorized
    );
    let admins = &mut ctx.accounts.platform_admins;
    if admins.admins.contains(&new_admin) {
        return Err(CustomError::AdminAlreadyExists.into());
    }
    admins.admins.push(new_admin);
    emit!(PlatformAdminAdded {
        admin_pubkey: new_admin,
        added_at: Clock::get()?.unix_timestamp,
    });
    Ok(())
}

// Utility function to check if signer is a Platform Admin
fn is_platform_admin(signer: &Pubkey, admins: &PlatformAdmins) -> bool {
    admins.admins.contains(signer)
}

#[event]
pub struct PlatformAdminAdded {
    pub admin_pubkey: Pubkey,
    pub added_at: i64,
}

#[event]
pub struct PlatformAdminRemoved {
    pub admin_pubkey: Pubkey,
    pub removed_at: i64,
}

#[derive(Accounts)]
pub struct RemovePlatformAdmin<'info> {
    #[account(mut, seeds = [b"platform_admins"], bump)]
    pub platform_admins: Account<'info, PlatformAdmins>,
    #[account(signer)]
    pub signer: AccountInfo<'info>,
}

// Instruction: Remove a Platform Admin (only callable by Super Root Admin)
pub fn remove_platform_admin(
    ctx: Context<RemovePlatformAdmin>,
    admin_pubkey: Pubkey,
    username: String,
    password: String,
) -> Result<()> {
    require!(
        is_super_root_admin(ctx.accounts.signer.key),
        CustomError::Unauthorized
    );
    require!(
        check_root_password(&username, &password),
        CustomError::Unauthorized
    );
    let admins = &mut ctx.accounts.platform_admins;
    if !admins.admins.contains(&admin_pubkey) {
        return Err(CustomError::AdminAlreadyExists.into());
    }
    admins.admins.retain(|a| a != &admin_pubkey);
    emit!(PlatformAdminRemoved {
        admin_pubkey,
        removed_at: Clock::get()?.unix_timestamp,
    });
    Ok(())
}
