"use client";

import { useEffect } from "react";
import { useWallet } from "@solana/wallet-adapter-react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import { useSodap } from "@/contexts/SodapContext";
import StoreSetup from "@/components/store/StoreSetup";
import AddProduct from "@/components/store/AddProduct";
import ProductList from "@/components/store/ProductList";

export default function Dashboard() {
  const { connected } = useWallet();
  const { userStore, fetchUserStore, fetchUserProducts } = useSodap();

  useEffect(() => {
    if (connected) {
      fetchUserStore();
    }
  }, [connected, fetchUserStore]);

  const handleProductSuccess = () => {
    fetchUserProducts();
  };

  return (
    <main className="min-h-screen bg-gray-100">
      <header className="bg-white shadow-md">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6 flex justify-between items-center">
          <h1 className="text-3xl font-bold text-gray-900">SoDap Dashboard</h1>
          <WalletMultiButton />
        </div>
      </header>

      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {!connected ? (
          <div className="bg-white p-12 rounded-lg shadow-md text-center">
            <h2 className="text-2xl font-bold mb-6">Welcome to SoDap</h2>
            <p className="text-gray-600 mb-8">
              Connect your Solana wallet to manage your store and products.
            </p>
            <div className="flex justify-center">
              <WalletMultiButton />
            </div>
          </div>
        ) : (
          <div className="space-y-8">
            <StoreSetup />

            {userStore && (
              <>
                <AddProduct onSuccess={handleProductSuccess} />
                <ProductList />
              </>
            )}
          </div>
        )}
      </div>
    </main>
  );
}
