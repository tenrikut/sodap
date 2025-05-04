# SoDap - Solana Decentralized Shopping

## Product Entity (SDP-PROD-001)

This implementation provides the functionality for store owners to register, manage, and track products on the Solana blockchain.

### Core Features

#### Backend (Smart Contract)

- **Product Registration**: Store products with unique UUIDs mapped to PDAs
- **Metadata Management**: Store product details including name, description, and imagery
- **Inventory Tracking**: Track product availability and stock levels
- **Tokenization Support**: Optional SPL token association for products
- **Update & Deactivation**: Allow product information updates and deactivation

#### Frontend (Next.js)

- **Admin Dashboard**: Product management interface for store owners
- **UUID Generation**: Create unique identifiers for each product
- **QR Scanning**: Scan product QR codes to view or purchase
- **Blockchain Integration**: Connect UI to on-chain product data

### Architecture

#### Smart Contract

The core product functionality is implemented in `programs/sodap/src/lib.rs` with the following instructions:

- `register_product`: Create a new product account with UUID, price, stock, and metadata
- `update_product`: Update product details
- `deactivate_product`: Mark products as inactive
- `purchase_product`: Process product purchases

#### Frontend Components

- **Product Management UI**: `/app/components/ProductManagement.tsx`
- **Type Definitions**: `/app/types/sodap.ts`
- **Admin Dashboard**: `/app/admin/products/page.tsx`
- **Blockchain Context**: `/app/contexts/SodapContext.tsx`
- **Solana Utils**: `/app/utils/solana.ts`

### Data Model

#### On-Chain Product Data

```rust
#[account]
pub struct Product {
    pub product_uuid: [u8; 16],  // Unique identifier stored as byte array
    pub store_id: Pubkey,        // Store that owns this product
    pub price: u64,              // Price in lamports (SOL)
    pub stock: u64,              // Current inventory level
    pub tokenized_type: TokenizedType, // None or SPL token
    pub metadata_uri: String,    // Off-chain data URI or inline JSON
    pub created_at: i64,         // Creation timestamp
    pub mint_status: MintStatus, // Status for tokenized products
    pub deactivated: bool,       // Product active status
}
```

#### Frontend Product Interface

```typescript
export interface Product {
  id: string; // UUID in string format
  name: string; // Product name
  description: string; // Product description
  price: number; // Price in SOL
  inventory: number; // Stock quantity
  imageUrl?: string; // Product image
  category?: string; // Product category
  tokenizedType: "None" | "SplToken"; // Tokenization type
  isActive: boolean; // Product status
  createdAt?: number; // Creation timestamp
}
```

### Usage

1. **Register Products**:

   - Generate a UUID using `generateUUID()`
   - Fill in product details (name, description, price, etc.)
   - Call `addProduct()` to store on-chain

2. **Update Products**:

   - Modify product details using the admin interface
   - Updates are committed to the blockchain

3. **Scan Products**:

   - QR codes contain the product's UUID
   - Scanning retrieves on-chain data and metadata

4. **Deactivate Products**:
   - Products can be marked inactive without deletion

### Development

To run the development environment:

```bash
cd app
npm install
npm run dev
```

Access the admin dashboard at `/admin/products` to manage products.

# SODAP Anchor Program

## Modular Structure

This Anchor program is organized for clarity, maintainability, and extensibility. Each module maps to a major business/user story feature:

- **product.rs**: Product registration, purchase, update, deactivation, and cart logic for store owners.
- **store.rs**: Store registration, update, admin management, and store account logic.
- **loyalty.rs**: Loyalty points, token minting, and redemption logic for customer rewards.
- **admin.rs**: Platform admin and super root admin management.
- **user.rs**: User profile creation, update, and purchase history.
- **error.rs**: Centralized custom error types for all instructions.
- **types.rs**: Shared enums and structs (e.g., roles, statuses, configs).
- **utils.rs**: Helper functions for role checks and admin authentication.

## How to Run Tests

1. **Install Anchor** (if not already):
   ```sh
   cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked
   ```
2. **Build the program:**
   ```sh
   anchor build
   ```
3. **Run tests:**
   ```sh
   anchor test
   ```

## Extending the Program

- Add new features by creating new modules or extending existing ones.
- Each module is self-contained and only exposes what is needed for the main program.
- Use the `types.rs` and `error.rs` modules for all shared types and errors.

## User Story Mapping

- **Platform Admin Management:** See `admin.rs` and `store.rs`.
- **Store Owner Product Management:** See `product.rs` and `store.rs`.
- **Loyalty & Rewards:** See `loyalty.rs` and `user.rs`.
- **User Profiles & Analytics:** See `user.rs` and `product.rs` (for events).
- **Transaction Monitoring:** All major instructions emit events for off-chain analytics.

---

For more details, see comments at the top of each module.
