// Store-related types
export interface Store {
  id: string;
  name: string;
  description: string;
  logoUrl?: string;
  isActive: boolean;
}

// Product-related types
export interface Product {
  id: string;
  name: string;
  description: string;
  price: number;
  inventory: number;
  imageUrl?: string;
  category?: string;
  tokenizedType: "None" | "SplToken";
  isActive: boolean;
  createdAt?: number;
}

// User-related types
export interface UserProfile {
  userId: string;
  walletAddress: string;
  loyaltyBalance: number;
  purchaseHistory: PurchaseRecord[];
  preferredStore?: string;
  deliveryAddress?: string;
  registeredAt: number;
}

export interface PurchaseRecord {
  storeId: string;
  transactionId: string;
  amount: number;
  loyaltyEarned: number;
  timestamp: number;
}

export interface Receipt {
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

// Blockchain integration types
export interface ProductUUID {
  uuid: string; // UUID in string format
}

export interface ProductWithUUID extends Product {
  uuid: string; // The on-chain UUID
}
