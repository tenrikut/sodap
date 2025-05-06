"use client";

import { useState, useEffect } from "react";
import Link from "next/link";
import { SodapProvider } from "../contexts/SodapContext";
import StoreScan from "../components/shopping/StoreScan";
import ProductScanner from "../components/shopping/ProductScanner";
import { ShoppingCartProvider } from "../contexts/ShoppingCartContext";
import { CartProvider } from "../contexts/CartContext";
import CartList from "../components/cart/CartList";
import { usePayment } from "../utils/payment";
import { motion } from "framer-motion";

export default function Marketplace() {
  const [activeStore, setActiveStore] = useState<string | null>(null);
  const [scanningMode, setScanningMode] = useState<"store" | "product" | null>(
    "store"
  );
  const [isLoading, setIsLoading] = useState(false);

  const handleStoreScanned = (storeId: string) => {
    setIsLoading(true);
    // Simulate loading for better UX
    setTimeout(() => {
      setActiveStore(storeId);
      setScanningMode("product");
      setIsLoading(false);
    }, 800);
  };

  const exitShoppingSession = () => {
    setIsLoading(true);
    // Simulate loading for better UX
    setTimeout(() => {
      setActiveStore(null);
      setScanningMode("store");
      setIsLoading(false);
    }, 500);
  };

  const { handlePayment } = usePayment();

  return (
    <SodapProvider>
      <ShoppingCartProvider>
        <CartProvider>
          <div className="min-h-screen bg-gradient-to-br from-purple-50 via-white to-purple-100">
            <div className="container mx-auto px-4 py-8">
              <motion.div 
                initial={{ opacity: 0, y: -20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.5 }}
                className="flex justify-between items-center mb-6"
              >
                <div className="flex items-center">
                  <div className="w-10 h-10 bg-purple-600 rounded-full flex items-center justify-center mr-3">
                    <span className="text-white text-xl font-bold">S</span>
                  </div>
                  <h1 className="text-3xl font-bold text-purple-900">SoDap Shopping</h1>
                </div>
                <Link 
                  href="/" 
                  className="flex items-center px-4 py-2 text-purple-600 hover:text-purple-800 transition-colors rounded-lg hover:bg-purple-100"
                >
                  <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" />
                  </svg>
                  Home
                </Link>
              </motion.div>

              <motion.div 
                initial={{ opacity: 0, scale: 0.95 }}
                animate={{ opacity: 1, scale: 1 }}
                transition={{ duration: 0.5, delay: 0.2 }}
                className="bg-white shadow-xl rounded-2xl p-8 mb-6"
              >
                {isLoading ? (
                  <div className="flex flex-col items-center justify-center py-12">
                    <div className="w-16 h-16 border-4 border-purple-500 border-t-transparent rounded-full animate-spin"></div>
                    <p className="mt-4 text-purple-700 font-medium">Loading...</p>
                  </div>
                ) : !activeStore ? (
                  <motion.div
                    initial={{ opacity: 0 }}
                    animate={{ opacity: 1 }}
                    transition={{ duration: 0.5 }}
                  >
                    <h2 className="text-2xl font-semibold mb-4 text-purple-900">
                      Start Shopping
                    </h2>
                    <p className="mb-6 text-gray-600">
                      Scan a store QR code to begin your shopping experience. Our app will help you find products and manage your cart effortlessly.
                    </p>

                    {scanningMode === "store" && (
                      <div className="bg-purple-50 p-6 rounded-xl mb-6">
                        <StoreScan onStoreScanned={handleStoreScanned} />
                      </div>
                    )}

                    {/* Demo button for easy testing */}
                    <div className="mt-6 flex justify-center">
                      <motion.button
                        whileHover={{ scale: 1.05 }}
                        whileTap={{ scale: 0.95 }}
                        onClick={() => handleStoreScanned("demo-store-123")}
                        className="bg-purple-600 text-white py-3 px-6 rounded-xl hover:bg-purple-700 transition-colors shadow-md hover:shadow-lg flex items-center"
                      >
                        <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
                        </svg>
                        Demo: Enter Store
                      </motion.button>
                    </div>
                  </motion.div>
                ) : (
                  <motion.div
                    initial={{ opacity: 0 }}
                    animate={{ opacity: 1 }}
                    transition={{ duration: 0.5 }}
                  >
                    <div className="flex justify-between items-center mb-6">
                      <div>
                        <h2 className="text-2xl font-semibold text-purple-900">
                          Shopping at
                        </h2>
                        <div className="flex items-center mt-1">
                          <div className="w-8 h-8 bg-purple-100 rounded-full flex items-center justify-center mr-2">
                            <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4 text-purple-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4" />
                            </svg>
                          </div>
                          <span className="text-lg font-medium text-purple-800">{activeStore}</span>
                        </div>
                      </div>
                      <motion.button
                        whileHover={{ scale: 1.05 }}
                        whileTap={{ scale: 0.95 }}
                        onClick={exitShoppingSession}
                        className="flex items-center text-red-600 hover:text-red-800 bg-red-50 hover:bg-red-100 py-2 px-4 rounded-lg transition-colors"
                      >
                        <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                        </svg>
                        Exit Shopping
                      </motion.button>
                    </div>

                    <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
                      <motion.div 
                        initial={{ opacity: 0, x: -20 }}
                        animate={{ opacity: 1, x: 0 }}
                        transition={{ duration: 0.5, delay: 0.2 }}
                        className="bg-purple-50 p-6 rounded-xl shadow-sm"
                      >
                        <div className="flex items-center mb-4">
                          <svg xmlns="http://www.w3.org/2000/svg" className="h-6 w-6 text-purple-600 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v1m6 11h2m-6 0h-2v4m0-11v3m0 0h.01M12 12h4.01M16 20h4M4 12h4m12 0h.01M5 8h2a1 1 0 001-1V5a1 1 0 00-1-1H5a1 1 0 00-1 1v2a1 1 0 001 1zm12 0h2a1 1 0 001-1V5a1 1 0 00-1-1h-2a1 1 0 00-1 1v2a1 1 0 001 1zM5 20h2a1 1 0 001-1v-2a1 1 0 00-1-1H5a1 1 0 00-1 1v2a1 1 0 001 1z" />
                          </svg>
                          <h3 className="text-xl font-medium text-purple-900">
                            Scan Products
                          </h3>
                        </div>
                        <p className="text-gray-600 mb-4">Scan product barcodes to add them to your cart.</p>
                        {scanningMode === "product" && (
                          <ProductScanner storeId={activeStore} />
                        )}
                      </motion.div>

                      <motion.div 
                        initial={{ opacity: 0, x: 20 }}
                        animate={{ opacity: 1, x: 0 }}
                        transition={{ duration: 0.5, delay: 0.3 }}
                        className="bg-purple-50 p-6 rounded-xl shadow-sm"
                      >
                        <div className="flex items-center mb-4">
                          <svg xmlns="http://www.w3.org/2000/svg" className="h-6 w-6 text-purple-600 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 11-4 0 2 2 0 014 0z" />
                          </svg>
                          <h3 className="text-xl font-medium text-purple-900">
                            Your Cart
                          </h3>
                        </div>
                        <CartList onConfirm={handlePayment} />
                      </motion.div>
                    </div>
                  </motion.div>
                )}
              </motion.div>
            </div>
          </div>
        </CartProvider>
      </ShoppingCartProvider>
    </SodapProvider>
  );
}
