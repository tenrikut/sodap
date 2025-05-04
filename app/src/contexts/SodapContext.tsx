"use client";

import { createContext, useState, useEffect, ReactNode } from "react";

// Mock data interfaces
interface Product {
  id: string;
  name: string;
  description: string;
  price: number;
  inventory: number;
  imageUrl?: string;
  isActive: boolean;
}

interface Store {
  id: string;
  name: string;
  description: string;
  logoUrl?: string;
  isActive: boolean;
}

interface UserProfile {
  userId: string;
  walletAddress: string;
  loyaltyBalance: number;
  purchaseHistory: PurchaseRecord[];
  preferredStore?: string;
  deliveryAddress?: string;
  registeredAt: number;
}

interface PurchaseRecord {
  storeId: string;
  transactionId: string;
  amount: number;
  loyaltyEarned: number;
  timestamp: number;
}

interface Receipt {
  items: Array<{
    id: string;
    name: string;
    price: number;
    quantity: number;
  }>;
  total: number;
  transactionId: string;
  timestamp: number;
  loyaltyPointsEarned: number;
}

interface SodapContextType {
  store: Store | null;
  userProfile: UserProfile | null;
  createStore: (name: string, description: string) => Promise<void>;
  updateStore: (name: string, description: string) => Promise<void>;
  getProducts: () => Promise<Product[]>;
  addProduct: (product: Omit<Product, "id">) => Promise<void>;
  updateProduct: (id: string, product: Partial<Product>) => Promise<void>;
  removeProduct: (id: string) => Promise<void>;
  scanStore: (storeId: string) => Promise<Store>;
  scanProduct: (productId: string) => Promise<Product>;
  purchaseCart: (
    items: Array<{ productId: string; quantity: number }>
  ) => Promise<Receipt>;
  getLoyaltyBalance: () => Promise<number>;
  redeemLoyaltyPoints: (amount: number) => Promise<void>;
  createOrUpdateUserProfile: (
    userId?: string,
    deliveryAddress?: string,
    preferredStore?: string
  ) => Promise<void>;
}

// Create the context with a default value
export const SodapContext = createContext<SodapContextType>({
  store: null,
  userProfile: null,
  createStore: async () => {},
  updateStore: async () => {},
  getProducts: async () => [],
  addProduct: async () => {},
  updateProduct: async () => {},
  removeProduct: async () => {},
  scanStore: async () => ({
    id: "",
    name: "",
    description: "",
    isActive: false,
  }),
  scanProduct: async () => ({
    id: "",
    name: "",
    description: "",
    price: 0,
    inventory: 0,
    isActive: false,
  }),
  purchaseCart: async () => ({
    items: [],
    total: 0,
    transactionId: "",
    timestamp: 0,
    loyaltyPointsEarned: 0,
  }),
  getLoyaltyBalance: async () => 0,
  redeemLoyaltyPoints: async () => {},
  createOrUpdateUserProfile: async () => {},
});

// Mock data for demo
const MOCK_PRODUCTS: Product[] = [
  {
    id: "1",
    name: "Solana T-Shirt",
    description: "Limited edition Solana-branded t-shirt",
    price: 0.5,
    inventory: 100,
    imageUrl: "https://via.placeholder.com/150",
    isActive: true,
  },
  {
    id: "2",
    name: "Crypto Coffee Mug",
    description: "Ceramic coffee mug with crypto designs",
    price: 0.2,
    inventory: 50,
    imageUrl: "https://via.placeholder.com/150",
    isActive: true,
  },
];

// Provider component
export const SodapProvider = ({ children }: { children: ReactNode }) => {
  const [store, setStore] = useState<Store | null>(null);
  const [products, setProducts] = useState<Product[]>(MOCK_PRODUCTS);
  const [userProfile, setUserProfile] = useState<UserProfile | null>(null);

  // Simulate loading store data from blockchain
  useEffect(() => {
    // For demo purposes, we'll just set a mock store after a delay
    const loadStore = async () => {
      await new Promise((resolve) => setTimeout(resolve, 500));
      setStore({
        id: "1",
        name: "Crypto Collectibles",
        description: "Your one-stop shop for crypto merchandise",
        isActive: true,
      });

      // Also load a mock user profile
      setUserProfile({
        userId: "user123",
        walletAddress: "5FHwkrdxD5AKmwHxx8JT7mQEjzUZBCk4wKQfujwBTuKW",
        loyaltyBalance: 150,
        purchaseHistory: [
          {
            storeId: "1",
            transactionId: "tx-abc123",
            amount: 0.7,
            loyaltyEarned: 70,
            timestamp: Date.now() - 86400000, // 1 day ago
          },
        ],
        registeredAt: Date.now() - 2592000000, // 30 days ago
      });
    };

    loadStore();
  }, []);

  // Create a new store
  const createStore = async (name: string, description: string) => {
    await new Promise((resolve) => setTimeout(resolve, 1000)); // Simulate blockchain delay
    setStore({
      id: "1",
      name,
      description,
      isActive: true,
    });
  };

  // Update store details
  const updateStore = async (name: string, description: string) => {
    await new Promise((resolve) => setTimeout(resolve, 1000)); // Simulate blockchain delay
    if (store) {
      setStore({
        ...store,
        name,
        description,
      });
    }
  };

  // Get all products for the store
  const getProducts = async () => {
    await new Promise((resolve) => setTimeout(resolve, 500)); // Simulate blockchain delay
    return products;
  };

  // Add a new product
  const addProduct = async (product: Omit<Product, "id">) => {
    await new Promise((resolve) => setTimeout(resolve, 1000)); // Simulate blockchain delay
    const newProduct: Product = {
      ...product,
      id: Math.random().toString(36).substr(2, 9), // Generate random ID
    };
    setProducts([...products, newProduct]);
  };

  // Update an existing product
  const updateProduct = async (id: string, productUpdate: Partial<Product>) => {
    await new Promise((resolve) => setTimeout(resolve, 1000)); // Simulate blockchain delay
    setProducts(
      products.map((product) =>
        product.id === id ? { ...product, ...productUpdate } : product
      )
    );
  };

  // Remove a product
  const removeProduct = async (id: string) => {
    await new Promise((resolve) => setTimeout(resolve, 1000)); // Simulate blockchain delay
    setProducts(products.filter((product) => product.id !== id));
  };

  // Scan a store QR code
  const scanStore = async (storeId: string) => {
    await new Promise((resolve) => setTimeout(resolve, 800)); // Simulate blockchain delay
    return {
      id: storeId,
      name: "Scanned Store",
      description: "This store was scanned via QR code",
      isActive: true,
    };
  };

  // Scan a product QR code
  const scanProduct = async (productId: string) => {
    await new Promise((resolve) => setTimeout(resolve, 500)); // Simulate blockchain delay
    const product = products.find((p) => p.id === productId);

    if (product) {
      return product;
    }

    // Return a mock product if not found
    return {
      id: productId,
      name: "Scanned Product",
      description: "This product was scanned via QR code",
      price: 0.35,
      inventory: 25,
      imageUrl: "https://via.placeholder.com/150",
      isActive: true,
    };
  };

  // Purchase items in a cart
  const purchaseCart = async (
    items: Array<{ productId: string; quantity: number }>
  ) => {
    await new Promise((resolve) => setTimeout(resolve, 2000)); // Simulate blockchain delay

    let total = 0;
    const purchasedItems = [];

    // Calculate total and prepare receipt items
    for (const item of items) {
      const product = products.find((p) => p.id === item.productId);
      if (product) {
        total += product.price * item.quantity;
        purchasedItems.push({
          id: product.id,
          name: product.name,
          price: product.price,
          quantity: item.quantity,
        });

        // Update inventory
        updateProduct(product.id, {
          inventory: Math.max(0, product.inventory - item.quantity),
        });
      }
    }

    // Calculate loyalty points (10 points per 0.1 SOL)
    const loyaltyPointsEarned = Math.floor(total * 100);

    // Update user's loyalty balance and purchase history
    if (userProfile) {
      setUserProfile({
        ...userProfile,
        loyaltyBalance: userProfile.loyaltyBalance + loyaltyPointsEarned,
        purchaseHistory: [
          {
            storeId: store?.id || "unknown",
            transactionId: `tx-${Math.random().toString(36).substring(2, 10)}`,
            amount: total,
            loyaltyEarned: loyaltyPointsEarned,
            timestamp: Date.now(),
          },
          ...userProfile.purchaseHistory,
        ],
      });
    }

    // Create and return receipt
    return {
      items: purchasedItems,
      total,
      transactionId: `tx-${Math.random().toString(36).substring(2, 10)}`,
      timestamp: Date.now(),
      loyaltyPointsEarned,
    };
  };

  // Get user's loyalty balance
  const getLoyaltyBalance = async () => {
    await new Promise((resolve) => setTimeout(resolve, 300)); // Simulate blockchain delay
    return userProfile?.loyaltyBalance || 0;
  };

  // Redeem loyalty points
  const redeemLoyaltyPoints = async (amount: number) => {
    await new Promise((resolve) => setTimeout(resolve, 1500)); // Simulate blockchain delay

    if (!userProfile) {
      throw new Error("User profile not found");
    }

    if (userProfile.loyaltyBalance < amount) {
      throw new Error("Insufficient loyalty points");
    }

    setUserProfile({
      ...userProfile,
      loyaltyBalance: userProfile.loyaltyBalance - amount,
    });
  };

  // Create or update user profile
  const createOrUpdateUserProfile = async (
    userId?: string,
    deliveryAddress?: string,
    preferredStore?: string
  ) => {
    await new Promise((resolve) => setTimeout(resolve, 1000)); // Simulate blockchain delay

    if (!userProfile) {
      // Create new profile
      setUserProfile({
        userId: userId || `user-${Math.random().toString(36).substring(2, 10)}`,
        walletAddress: "5FHwkrdxD5AKmwHxx8JT7mQEjzUZBCk4wKQfujwBTuKW",
        loyaltyBalance: 0,
        purchaseHistory: [],
        deliveryAddress,
        preferredStore,
        registeredAt: Date.now(),
      });
    } else {
      // Update existing profile
      setUserProfile({
        ...userProfile,
        userId: userId || userProfile.userId,
        deliveryAddress: deliveryAddress || userProfile.deliveryAddress,
        preferredStore: preferredStore || userProfile.preferredStore,
      });
    }
  };

  return (
    <SodapContext.Provider
      value={{
        store,
        userProfile,
        createStore,
        updateStore,
        getProducts,
        addProduct,
        updateProduct,
        removeProduct,
        scanStore,
        scanProduct,
        purchaseCart,
        getLoyaltyBalance,
        redeemLoyaltyPoints,
        createOrUpdateUserProfile,
      }}
    >
      {children}
    </SodapContext.Provider>
  );
};
