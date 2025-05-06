"use client";

import { useState } from "react";
import Link from "next/link";
import StoreSetup from "../components/store/StoreSetup";
import ProductList from "../components/store/ProductList";
import AddProduct from "../components/store/AddProduct";
import { SodapProvider } from "../contexts/SodapContext";
import { motion } from "framer-motion";

export default function Dashboard() {
  const [activeTab, setActiveTab] = useState("products");
  const [isAddingProduct, setIsAddingProduct] = useState(false);

  return (
    <SodapProvider>
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
              <h1 className="text-3xl font-bold text-purple-900">Store Dashboard</h1>
            </div>
            <Link 
              href="/" 
              className="flex items-center px-4 py-2 text-purple-600 hover:text-purple-800 transition-colors rounded-lg hover:bg-purple-100"
            >
              <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" />
              </svg>
              Back to Home
            </Link>
          </motion.div>

          <motion.div 
            initial={{ opacity: 0, scale: 0.95 }}
            animate={{ opacity: 1, scale: 1 }}
            transition={{ duration: 0.5, delay: 0.2 }}
            className="bg-white shadow-xl rounded-2xl p-8 mb-6"
          >
            <div className="border-b border-gray-200 mb-8">
              <nav className="flex space-x-8">
                <button
                  onClick={() => setActiveTab("setup")}
                  className={`pb-4 px-1 relative ${
                    activeTab === "setup"
                      ? "text-purple-700 font-medium"
                      : "text-gray-500 hover:text-gray-700"
                  }`}
                >
                  <div className="flex items-center">
                    <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                    </svg>
                    Store Setup
                  </div>
                  {activeTab === "setup" && (
                    <motion.div
                      layoutId="activeTabIndicator"
                      className="absolute bottom-0 left-0 right-0 h-0.5 bg-purple-600"
                      initial={{ opacity: 0 }}
                      animate={{ opacity: 1 }}
                      transition={{ duration: 0.3 }}
                    />
                  )}
                </button>
                <button
                  onClick={() => setActiveTab("products")}
                  className={`pb-4 px-1 relative ${
                    activeTab === "products"
                      ? "text-purple-700 font-medium"
                      : "text-gray-500 hover:text-gray-700"
                  }`}
                >
                  <div className="flex items-center">
                    <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z" />
                    </svg>
                    Products
                  </div>
                  {activeTab === "products" && (
                    <motion.div
                      layoutId="activeTabIndicator"
                      className="absolute bottom-0 left-0 right-0 h-0.5 bg-purple-600"
                      initial={{ opacity: 0 }}
                      animate={{ opacity: 1 }}
                      transition={{ duration: 0.3 }}
                    />
                  )}
                </button>
              </nav>
            </div>

            <motion.div
              key={activeTab}
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.3 }}
            >
              {activeTab === "setup" && (
                <div className="bg-purple-50 p-6 rounded-xl">
                  <StoreSetup onComplete={() => {}} />
                </div>
              )}

              {activeTab === "products" && (
                <>
                  {isAddingProduct ? (
                    <AddProduct
                      onCancel={() => setIsAddingProduct(false)}
                      onSubmit={() => setIsAddingProduct(false)}
                      isOpen={isAddingProduct}
                    />
                  ) : (
                    <>
                      <div className="flex justify-end mb-6">
                        <motion.button
                          whileHover={{ scale: 1.05 }}
                          whileTap={{ scale: 0.95 }}
                          onClick={() => setIsAddingProduct(true)}
                          className="bg-purple-600 text-white py-2 px-5 rounded-lg hover:bg-purple-700 transition-colors shadow-md hover:shadow-lg flex items-center"
                        >
                          <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
                          </svg>
                          Add Product
                        </motion.button>
                      </div>
                      <div className="bg-purple-50 p-6 rounded-xl">
                        <ProductList />
                      </div>
                    </>
                  )}
                </>
              )}
            </motion.div>
          </motion.div>
        </div>
      </div>
    </SodapProvider>
  );
}
