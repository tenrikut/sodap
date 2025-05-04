"use client";

import { useState, useEffect } from "react";
import { useSodap } from "@/contexts/SodapContext";
import { useWallet } from "@solana/wallet-adapter-react";
import EditProduct from "./EditProduct";

export default function ProductList() {
  const { userProducts, fetchUserProducts, loading, deactivateProduct } =
    useSodap();
  const { connected } = useWallet();
  const [selectedProduct, setSelectedProduct] = useState<any | null>(null);

  useEffect(() => {
    if (connected) {
      fetchUserProducts();
    }
  }, [connected, fetchUserProducts]);

  const handleDeactivate = async (productUuid: Uint8Array) => {
    if (confirm("Are you sure you want to deactivate this product?")) {
      await deactivateProduct(productUuid);
    }
  };

  const handleEdit = (product: any) => {
    setSelectedProduct(product);
  };

  const handleCloseEdit = () => {
    setSelectedProduct(null);
  };

  const handleEditSuccess = () => {
    fetchUserProducts();
  };

  if (!connected) {
    return (
      <div className="bg-white p-8 rounded-lg shadow-md">
        <h2 className="text-2xl font-bold mb-4">Products</h2>
        <p className="text-gray-500">Connect your wallet to view products</p>
      </div>
    );
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center p-8">
        <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-purple-500"></div>
      </div>
    );
  }

  if (userProducts.length === 0) {
    return (
      <div className="bg-white p-8 rounded-lg shadow-md">
        <h2 className="text-2xl font-bold mb-4">Products</h2>
        <p className="text-gray-500">
          No products found. Add your first product.
        </p>
      </div>
    );
  }

  return (
    <div className="bg-white p-8 rounded-lg shadow-md">
      <h2 className="text-2xl font-bold mb-6">Your Products</h2>

      <div className="overflow-x-auto">
        <table className="min-w-full divide-y divide-gray-200">
          <thead className="bg-gray-50">
            <tr>
              <th
                scope="col"
                className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                Product
              </th>
              <th
                scope="col"
                className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                Price
              </th>
              <th
                scope="col"
                className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                Stock
              </th>
              <th
                scope="col"
                className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                Type
              </th>
              <th
                scope="col"
                className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                Status
              </th>
              <th
                scope="col"
                className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                Actions
              </th>
            </tr>
          </thead>
          <tbody className="bg-white divide-y divide-gray-200">
            {userProducts.map((product, index) => (
              <tr key={index}>
                <td className="px-6 py-4 whitespace-nowrap">
                  <div className="flex items-center">
                    <div className="flex-shrink-0 h-10 w-10 rounded-full bg-gray-200 flex items-center justify-center">
                      <span className="text-sm font-medium text-gray-500">
                        IMG
                      </span>
                    </div>
                    <div className="ml-4">
                      <div className="text-sm font-medium text-gray-900">
                        Product {index + 1}
                      </div>
                      <div className="text-sm text-gray-500 truncate max-w-xs">
                        {product.metadataUri}
                      </div>
                    </div>
                  </div>
                </td>
                <td className="px-6 py-4 whitespace-nowrap">
                  <div className="text-sm text-gray-900">
                    {product.price} SOL
                  </div>
                </td>
                <td className="px-6 py-4 whitespace-nowrap">
                  <div className="text-sm text-gray-900">{product.stock}</div>
                </td>
                <td className="px-6 py-4 whitespace-nowrap">
                  <span className="px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-green-100 text-green-800">
                    {product.tokenizedType.none ? "Standard" : "SPL Token"}
                  </span>
                </td>
                <td className="px-6 py-4 whitespace-nowrap">
                  <span
                    className={`px-2 inline-flex text-xs leading-5 font-semibold rounded-full ${
                      product.deactivated
                        ? "bg-red-100 text-red-800"
                        : "bg-green-100 text-green-800"
                    }`}
                  >
                    {product.deactivated ? "Inactive" : "Active"}
                  </span>
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm font-medium">
                  <button
                    className="text-indigo-600 hover:text-indigo-900 mr-4"
                    onClick={() => handleEdit(product)}
                  >
                    Edit
                  </button>
                  {!product.deactivated && (
                    <button
                      className="text-red-600 hover:text-red-900"
                      onClick={() => handleDeactivate(product.productUuid)}
                    >
                      Deactivate
                    </button>
                  )}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {selectedProduct && (
        <EditProduct
          product={selectedProduct}
          onClose={handleCloseEdit}
          onSuccess={handleEditSuccess}
        />
      )}
    </div>
  );
}
