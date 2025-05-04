"use client";

import { useState } from "react";
import Link from "next/link";
import { SodapProvider } from "../../contexts/SodapContext";
import StoreSetup from "../../components/store/StoreSetup";
import ProductList from "../../components/store/ProductList";
import AddProduct from "../../components/store/AddProduct";

export default function Dashboard() {
  const [activeTab, setActiveTab] = useState("products");
  const [isAddingProduct, setIsAddingProduct] = useState(false);

  return (
    <SodapProvider>
      <div className="container mx-auto px-4 py-8">
        <div className="flex justify-between items-center mb-6">
          <h1 className="text-3xl font-bold">Store Dashboard</h1>
          <Link href="/" className="text-blue-600 hover:underline">
            Back to Home
          </Link>
        </div>

        <div className="bg-white shadow-md rounded-lg p-6">
          <div className="border-b border-gray-200 mb-6">
            <nav className="flex space-x-8">
              <button
                onClick={() => setActiveTab("setup")}
                className={`pb-4 px-1 ${
                  activeTab === "setup"
                    ? "border-b-2 border-blue-500 font-medium text-blue-600"
                    : "text-gray-500 hover:text-gray-700"
                }`}
              >
                Store Setup
              </button>
              <button
                onClick={() => setActiveTab("products")}
                className={`pb-4 px-1 ${
                  activeTab === "products"
                    ? "border-b-2 border-blue-500 font-medium text-blue-600"
                    : "text-gray-500 hover:text-gray-700"
                }`}
              >
                Products
              </button>
            </nav>
          </div>

          {activeTab === "setup" && <StoreSetup />}

          {activeTab === "products" && (
            <>
              {isAddingProduct ? (
                <AddProduct onCancel={() => setIsAddingProduct(false)} />
              ) : (
                <>
                  <div className="flex justify-end mb-4">
                    <button
                      onClick={() => setIsAddingProduct(true)}
                      className="bg-blue-600 text-white py-2 px-4 rounded hover:bg-blue-700 transition"
                    >
                      Add Product
                    </button>
                  </div>
                  <ProductList />
                </>
              )}
            </>
          )}
        </div>
      </div>
    </SodapProvider>
  );
}
