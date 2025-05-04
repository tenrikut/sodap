"use client";

import {
  createContext,
  useContext,
  useCallback,
  useState,
  useEffect,
  ReactNode,
} from "react";
import { useAnchorWallet, useConnection } from "@solana/wallet-adapter-react";
import {
  Program,
  AnchorProvider,
  Idl,
  setProvider,
  web3,
} from "@coral-xyz/anchor";
import { PublicKey, Keypair } from "@solana/web3.js";
import { v4 as uuidv4 } from "uuid";

// We'll import the IDL from the target directory later
// For now we'll use a placeholder
const PROGRAM_ID = new PublicKey(
  "4eLJ3QGiNrPN6UUr2fNxq6tUZqFdBMVpXkL2MhsKNriv"
);

interface Product {
  productUuid: Uint8Array;
  storeId: PublicKey;
  price: number;
  stock: number;
  tokenizedType: { none?: {} } | { splToken?: {} };
  metadataUri: string;
  createdAt: number;
  mintStatus: { notMinted?: {} } | { minted?: {} } | { failed?: {} };
  deactivated: boolean;
}

interface Store {
  owner: PublicKey;
  name: string;
  description: string;
  logoUri: string;
  createdAt: number;
  revenue: number;
  loyaltyConfig: {
    pointsPerDollar: number;
    minimumPurchase: number;
    rewardPercentage: number;
    isActive: boolean;
  };
  adminRoles: {
    adminPubkey: PublicKey;
    roleType: { owner?: {} } | { manager?: {} } | { cashier?: {} };
  }[];
  isActive: boolean;
}

interface SodapContextType {
  program: Program | null;
  loading: boolean;
  userStore: Store | null;
  userProducts: Product[];
  registerStore: (
    name: string,
    description: string,
    logoUri: string
  ) => Promise<void>;
  updateStore: (
    name?: string,
    description?: string,
    logoUri?: string
  ) => Promise<void>;
  addProduct: (
    price: number,
    stock: number,
    tokenizedType: "None" | "SplToken",
    metadataUri: string
  ) => Promise<void>;
  updateProduct: (
    productUuid: Uint8Array,
    price?: number,
    stock?: number,
    metadataUri?: string,
    tokenizedType?: "None" | "SplToken"
  ) => Promise<void>;
  deactivateProduct: (productUuid: Uint8Array) => Promise<void>;
  fetchUserStore: () => Promise<void>;
  fetchUserProducts: () => Promise<void>;
}

// Create the context
const SodapContext = createContext<SodapContextType | null>(null);

// Helper function to convert a UUID string to bytes
function uuidToBytes(uuid: string): Uint8Array {
  const bytes = new Uint8Array(16);
  const parts = uuid.replace(/-/g, "").match(/.{2}/g) || [];
  for (let i = 0; i < 16; i++) {
    bytes[i] = parseInt(parts[i], 16);
  }
  return bytes;
}

export const SodapProvider = ({ children }: { children: ReactNode }) => {
  const { connection } = useConnection();
  const wallet = useAnchorWallet();
  const [program, setProgram] = useState<Program | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [userStore, setUserStore] = useState<Store | null>(null);
  const [userProducts, setUserProducts] = useState<Product[]>([]);

  // Initialize the Anchor program
  useEffect(() => {
    if (wallet && connection) {
      try {
        const provider = new AnchorProvider(
          connection,
          wallet,
          AnchorProvider.defaultOptions()
        );
        setProvider(provider);

        // We'll need to import the IDL when it's available
        // For now, we're assuming it would be loaded
        // const idl = JSON.parse(fs.readFileSync('./target/idl/sodap.json', 'utf8'));
        // const program = new Program(idl, PROGRAM_ID, provider);

        // setProgram(program);
        setLoading(false);
      } catch (error) {
        console.error("Failed to initialize program:", error);
        setLoading(false);
      }
    }
  }, [wallet, connection]);

  // Fetch the user's store
  const fetchUserStore = useCallback(async () => {
    if (!program || !wallet) return;

    try {
      setLoading(true);

      // Find the store PDA for the current user
      const [storePda] = web3.PublicKey.findProgramAddressSync(
        [Buffer.from("store"), wallet.publicKey.toBuffer()],
        program.programId
      );

      // Fetch the store data
      const storeAccount = await program.account.store.fetch(storePda);
      setUserStore(storeAccount as unknown as Store);
    } catch (error) {
      console.error("Error fetching store:", error);
      setUserStore(null);
    } finally {
      setLoading(false);
    }
  }, [program, wallet]);

  // Fetch all products for the user's store
  const fetchUserProducts = useCallback(async () => {
    if (!program || !userStore) return;

    try {
      setLoading(true);

      // Get all products where store_id matches the user's store
      const products = await program.account.product.all([
        {
          memcmp: {
            offset: 16, // After the product_uuid (16 bytes)
            bytes: userStore.owner.toBase58(),
          },
        },
      ]);

      setUserProducts(products.map((p) => p.account) as unknown as Product[]);
    } catch (error) {
      console.error("Error fetching products:", error);
      setUserProducts([]);
    } finally {
      setLoading(false);
    }
  }, [program, userStore]);

  // Register a new store
  const registerStore = useCallback(
    async (name: string, description: string, logoUri: string) => {
      if (!program || !wallet) return;

      try {
        setLoading(true);

        const storeId = wallet.publicKey;

        // Find the store PDA
        const [storePda] = web3.PublicKey.findProgramAddressSync(
          [Buffer.from("store"), storeId.toBuffer()],
          program.programId
        );

        // Default loyalty configuration
        const loyaltyConfig = {
          pointsPerDollar: new web3.BN(10),
          minimumPurchase: new web3.BN(100),
          rewardPercentage: new web3.BN(5),
          isActive: true,
        };

        // Call the registerStore instruction
        await program.methods
          .registerStore(storeId, name, description, logoUri, loyaltyConfig)
          .accounts({
            store: storePda,
            owner: wallet.publicKey,
            systemProgram: web3.SystemProgram.programId,
          })
          .rpc();

        // Update local state
        await fetchUserStore();
      } catch (error) {
        console.error("Error registering store:", error);
      } finally {
        setLoading(false);
      }
    },
    [program, wallet, fetchUserStore]
  );

  // Update store information
  const updateStore = useCallback(
    async (name?: string, description?: string, logoUri?: string) => {
      if (!program || !wallet || !userStore) return;

      try {
        setLoading(true);

        const storeId = wallet.publicKey;

        // Find the store PDA
        const [storePda] = web3.PublicKey.findProgramAddressSync(
          [Buffer.from("store"), storeId.toBuffer()],
          program.programId
        );

        // Call the updateStore instruction
        await program.methods
          .updateStore(
            storeId,
            name || null,
            description || null,
            logoUri || null,
            null // Keep the current loyalty config
          )
          .accounts({
            store: storePda,
            owner: wallet.publicKey,
          })
          .rpc();

        // Update local state
        await fetchUserStore();
      } catch (error) {
        console.error("Error updating store:", error);
      } finally {
        setLoading(false);
      }
    },
    [program, wallet, userStore, fetchUserStore]
  );

  // Add a new product
  const addProduct = useCallback(
    async (
      price: number,
      stock: number,
      tokenizedType: "None" | "SplToken",
      metadataUri: string
    ) => {
      if (!program || !wallet || !userStore) return;

      try {
        setLoading(true);

        // Generate a unique product UUID
        const uuid = uuidv4();
        const productUuid = uuidToBytes(uuid);

        // Find the product PDA
        const [productPda] = web3.PublicKey.findProgramAddressSync(
          [Buffer.from("product"), productUuid],
          program.programId
        );

        // Find the store PDA
        const [storePda] = web3.PublicKey.findProgramAddressSync(
          [Buffer.from("store"), wallet.publicKey.toBuffer()],
          program.programId
        );

        // Create a dummy mint for now (in a real app we'd create this properly)
        const mintKeypair = Keypair.generate();
        const tokenAccount = Keypair.generate();

        const tokenizedTypeValue =
          tokenizedType === "None" ? { none: {} } : { splToken: {} };

        // Call the registerProduct instruction
        await program.methods
          .registerProduct(
            Array.from(productUuid),
            new web3.BN(price),
            new web3.BN(stock),
            tokenizedTypeValue,
            metadataUri
          )
          .accounts({
            product: productPda,
            store: wallet.publicKey,
            storeAccount: storePda,
            mint: mintKeypair.publicKey,
            tokenAccount: tokenAccount.publicKey,
            tokenProgram: new PublicKey(
              "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            ),
            associatedTokenProgram: new PublicKey(
              "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
            ),
            systemProgram: web3.SystemProgram.programId,
            rent: web3.SYSVAR_RENT_PUBKEY,
          })
          .signers([])
          .rpc();

        // Update local state
        await fetchUserProducts();
      } catch (error) {
        console.error("Error adding product:", error);
      } finally {
        setLoading(false);
      }
    },
    [program, wallet, userStore, fetchUserProducts]
  );

  // Update an existing product
  const updateProduct = useCallback(
    async (
      productUuid: Uint8Array,
      price?: number,
      stock?: number,
      metadataUri?: string,
      tokenizedType?: "None" | "SplToken"
    ) => {
      if (!program || !wallet || !userStore) return;

      try {
        setLoading(true);

        // Find the product PDA
        const [productPda] = web3.PublicKey.findProgramAddressSync(
          [Buffer.from("product"), productUuid],
          program.programId
        );

        // Find the store PDA
        const [storePda] = web3.PublicKey.findProgramAddressSync(
          [Buffer.from("store"), wallet.publicKey.toBuffer()],
          program.programId
        );

        const tokenizedTypeValue =
          tokenizedType === "None"
            ? { none: {} }
            : tokenizedType === "SplToken"
            ? { splToken: {} }
            : null;

        // Call the updateProduct instruction
        await program.methods
          .updateProduct(
            Array.from(productUuid),
            price ? new web3.BN(price) : null,
            stock ? new web3.BN(stock) : null,
            metadataUri || null,
            tokenizedTypeValue
          )
          .accounts({
            product: productPda,
            store: wallet.publicKey,
            storeAccount: storePda,
          })
          .rpc();

        // Update local state
        await fetchUserProducts();
      } catch (error) {
        console.error("Error updating product:", error);
      } finally {
        setLoading(false);
      }
    },
    [program, wallet, userStore, fetchUserProducts]
  );

  // Deactivate a product
  const deactivateProduct = useCallback(
    async (productUuid: Uint8Array) => {
      if (!program || !wallet || !userStore) return;

      try {
        setLoading(true);

        // Find the product PDA
        const [productPda] = web3.PublicKey.findProgramAddressSync(
          [Buffer.from("product"), productUuid],
          program.programId
        );

        // Find the store PDA
        const [storePda] = web3.PublicKey.findProgramAddressSync(
          [Buffer.from("store"), wallet.publicKey.toBuffer()],
          program.programId
        );

        // Call the deactivateProduct instruction
        await program.methods
          .deactivateProduct(Array.from(productUuid))
          .accounts({
            product: productPda,
            store: wallet.publicKey,
            storeAccount: storePda,
          })
          .rpc();

        // Update local state
        await fetchUserProducts();
      } catch (error) {
        console.error("Error deactivating product:", error);
      } finally {
        setLoading(false);
      }
    },
    [program, wallet, userStore, fetchUserProducts]
  );

  const value = {
    program,
    loading,
    userStore,
    userProducts,
    registerStore,
    updateStore,
    addProduct,
    updateProduct,
    deactivateProduct,
    fetchUserStore,
    fetchUserProducts,
  };

  return (
    <SodapContext.Provider value={value}>{children}</SodapContext.Provider>
  );
};

export const useSodap = () => {
  const context = useContext(SodapContext);
  if (!context) {
    throw new Error("useSodap must be used within a SodapProvider");
  }
  return context;
};
