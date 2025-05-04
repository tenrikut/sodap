"use client";

import { useState } from "react";
import Link from "next/link";
import { SodapProvider } from "@/contexts/SodapContext";
import StoreScan from "../src/components/shopping/StoreScan";
import ShoppingCart from "../src/components/shopping/ShoppingCart";
import ProductScanner from "../src/components/shopping/ProductScanner";
import { ShoppingCartProvider } from "../src/contexts/ShoppingCartContext";

export default function Marketplace() {
  const [activeStore, setActiveStore] = useState(null);
  const [scanningMode, setScanningMode] = useState<"store" | "product" | null>(
    "store"
  );

  const handleStoreScanned = (storeId: string) => {
    setActiveStore(storeId);
    setScanningMode("product");
  };

  const exitShoppingSession = () => {
    setActiveStore(null);
    setScanningMode("store");
  };

  return (
    <SodapProvider>
      <ShoppingCartProvider>
        <div className="container mx-auto px-4 py-8">
          <div className="flex justify-between items-center mb-6">
            <h1 className="text-3xl font-bold">SoDap Shopping</h1>
            <Link href="/" className="text-blue-600 hover:underline">
              Back to Home
            </Link>
          </div>

          <div className="bg-white shadow-md rounded-lg p-6 mb-6">
            {!activeStore ? (
              <>
                <h2 className="text-2xl font-semibold mb-4">Start Shopping</h2>
                <p className="mb-6">
                  Scan a store QR code to begin your shopping experience.
                </p>

                {scanningMode === "store" && (
                  <StoreScan onStoreScanned={handleStoreScanned} />
                )}

                {/* Demo button for easy testing */}
                <div className="mt-6">
                  <button
                    onClick={() => handleStoreScanned("demo-store-123")}
                    className="bg-purple-600 text-white py-2 px-4 rounded hover:bg-purple-700 transition-colors"
                  >
                    Demo: Enter Store
                  </button>
                </div>
              </>
            ) : (
              <>
                <div className="flex justify-between items-center mb-4">
                  <h2 className="text-2xl font-semibold">
                    Shopping at Store: {activeStore}
                  </h2>
                  <button
                    onClick={exitShoppingSession}
                    className="text-red-600 hover:text-red-800"
                  >
                    Exit Shopping Session
                  </button>
                </div>

                <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                  <div>
                    <h3 className="text-xl font-medium mb-3">Scan Products</h3>
                    {scanningMode === "product" && (
                      <ProductScanner storeId={activeStore} />
                    )}
                  </div>

                  <div>
                    <h3 className="text-xl font-medium mb-3">Your Cart</h3>
                    <ShoppingCart storeId={activeStore} />
                  </div>
                </div>
              </>
            )}
          </div>
        </div>
      </ShoppingCartProvider>
    </SodapProvider>
  );
}
