import { Connection, PublicKey } from "@solana/web3.js";
import { AnchorProvider, Program, web3 } from "@project-serum/anchor";
import { Product } from "@/types/sodap";

// Define the program ID (replace with actual deployed program ID)
export const PROGRAM_ID = new PublicKey(
  "4eLJ3QGiNrPN6UUr2fNxq6tUZqFdBMVpXkL2MhsKNriv"
);

// Utility for connecting to the Solana network
export async function getConnection() {
  return new Connection(
    process.env.NEXT_PUBLIC_SOLANA_RPC_URL || "https://api.devnet.solana.com"
  );
}

// Utility to convert a standard UUID to byte array for on-chain storage
export function uuidToBytes(uuid: string): Uint8Array {
  // Remove hyphens from UUID
  const hexString = uuid.replace(/-/g, "");

  // Convert hex string to byte array
  const byteArray = new Uint8Array(16);
  for (let i = 0; i < 16; i++) {
    byteArray[i] = parseInt(hexString.substring(i * 2, i * 2 + 2), 16);
  }

  return byteArray;
}

// Utility to convert byte array from on-chain storage to UUID string
export function bytesToUuid(bytes: Uint8Array): string {
  // Convert byte array to hex string
  let hexString = "";
  for (let i = 0; i < bytes.length; i++) {
    const hex = bytes[i].toString(16).padStart(2, "0");
    hexString += hex;
  }

  // Format as UUID
  return [
    hexString.substring(0, 8),
    hexString.substring(8, 12),
    hexString.substring(12, 16),
    hexString.substring(16, 20),
    hexString.substring(20, 32),
  ].join("-");
}

// Function to derive the product PDA address
export async function findProductAddress(
  productUuid: Uint8Array
): Promise<[PublicKey, number]> {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("product"), Buffer.from(productUuid)],
    PROGRAM_ID
  );
}

// Function to derive the store PDA address
export async function findStoreAddress(
  storeId: PublicKey
): Promise<[PublicKey, number]> {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("store"), storeId.toBuffer()],
    PROGRAM_ID
  );
}

// Function to register a product on-chain
export async function registerProduct(
  program: Program,
  storeKeypair: web3.Keypair,
  productData: Omit<Product, "id">,
  productUuid: Uint8Array
): Promise<string> {
  try {
    // Convert price from SOL to lamports (1 SOL = 10^9 lamports)
    const priceInLamports = Math.floor(productData.price * 1_000_000_000);

    // Find the PDA for the product
    const [productPda, _] = await findProductAddress(productUuid);

    // Find the PDA for the store
    const storeId = storeKeypair.publicKey;
    const [storePda, __] = await findStoreAddress(storeId);

    // Call the smart contract to register the product
    const tx = await program.methods
      .registerProduct(
        productUuid,
        new web3.BN(priceInLamports),
        new web3.BN(productData.inventory),
        { none: {} }, // TokenizedType (None or SplToken)
        JSON.stringify({
          name: productData.name,
          description: productData.description,
          imageUrl: productData.imageUrl,
          category: productData.category,
        }) // Metadata URI - we're storing JSON directly in this example
      )
      .accounts({
        product: productPda,
        store: storeKeypair.publicKey,
        storeAccount: storePda,
        mint: web3.PublicKey.default, // Not using SPL tokens in this example
        tokenAccount: web3.PublicKey.default, // Not using SPL tokens in this example
        tokenProgram: web3.PublicKey.default, // Not using SPL tokens in this example
        associatedTokenProgram: web3.PublicKey.default, // Not using SPL tokens in this example
        systemProgram: web3.SystemProgram.programId,
        rent: web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([storeKeypair])
      .rpc();

    console.log("Transaction signature:", tx);
    return tx;
  } catch (error) {
    console.error("Error registering product:", error);
    throw error;
  }
}

// Function to update a product on-chain
export async function updateProduct(
  program: Program,
  storeKeypair: web3.Keypair,
  productUuid: Uint8Array,
  updates: Partial<Omit<Product, "id">>
): Promise<string> {
  try {
    // Find the PDA for the product
    const [productPda, _] = await findProductAddress(productUuid);

    // Find the PDA for the store
    const storeId = storeKeypair.publicKey;
    const [storePda, __] = await findStoreAddress(storeId);

    // Prepare update parameters
    const newPrice = updates.price
      ? new web3.BN(Math.floor(updates.price * 1_000_000_000))
      : null;

    const newStock = updates.inventory ? new web3.BN(updates.inventory) : null;

    let newMetadataUri = null;
    if (
      updates.name ||
      updates.description ||
      updates.imageUrl ||
      updates.category
    ) {
      // Fetch current product to merge with updates
      const productAccount = await program.account.product.fetch(productPda);
      const currentMetadata = JSON.parse(productAccount.metadataUri);

      const updatedMetadata = {
        name: updates.name || currentMetadata.name,
        description: updates.description || currentMetadata.description,
        imageUrl: updates.imageUrl || currentMetadata.imageUrl,
        category: updates.category || currentMetadata.category,
      };

      newMetadataUri = JSON.stringify(updatedMetadata);
    }

    // Call the smart contract to update the product
    const tx = await program.methods
      .updateProduct(
        productUuid,
        newPrice,
        newStock,
        newMetadataUri,
        null // No tokenized type update
      )
      .accounts({
        product: productPda,
        store: storeKeypair.publicKey,
        storeAccount: storePda,
      })
      .signers([storeKeypair])
      .rpc();

    console.log("Update transaction signature:", tx);
    return tx;
  } catch (error) {
    console.error("Error updating product:", error);
    throw error;
  }
}

// Function to deactivate a product on-chain
export async function deactivateProduct(
  program: Program,
  storeKeypair: web3.Keypair,
  productUuid: Uint8Array
): Promise<string> {
  try {
    // Find the PDA for the product
    const [productPda, _] = await findProductAddress(productUuid);

    // Find the PDA for the store
    const storeId = storeKeypair.publicKey;
    const [storePda, __] = await findStoreAddress(storeId);

    // Call the smart contract to deactivate the product
    const tx = await program.methods
      .deactivateProduct(productUuid)
      .accounts({
        product: productPda,
        store: storeKeypair.publicKey,
        storeAccount: storePda,
      })
      .signers([storeKeypair])
      .rpc();

    console.log("Deactivate transaction signature:", tx);
    return tx;
  } catch (error) {
    console.error("Error deactivating product:", error);
    throw error;
  }
}

// Function to fetch a product from chain
export async function fetchProduct(
  program: Program,
  productUuid: Uint8Array
): Promise<Product> {
  try {
    // Find the PDA for the product
    const [productPda, _] = await findProductAddress(productUuid);

    // Fetch the product data from the blockchain
    const productAccount = await program.account.product.fetch(productPda);

    // Parse the metadata URI
    const metadata = JSON.parse(productAccount.metadataUri);

    // Create a Product object with all required attributes
    return {
      id: bytesToUuid(productAccount.productUuid),
      name: metadata.name,
      description: metadata.description,
      price: productAccount.price.toNumber() / 1_000_000_000, // Convert lamports to SOL
      inventory: productAccount.stock.toNumber(),
      imageUrl: metadata.imageUrl,
      category: metadata.category,
      tokenizedType: productAccount.tokenizedType.none ? "None" : "SplToken",
      isActive: !productAccount.deactivated,
      createdAt: productAccount.createdAt.toNumber() * 1000, // Convert to JS timestamp
    };
  } catch (error) {
    console.error("Error fetching product:", error);
    throw error;
  }
}
