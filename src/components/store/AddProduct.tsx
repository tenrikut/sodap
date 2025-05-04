"use client";

import { useState } from "react";
import { useSodap } from "@/contexts/SodapContext";
import { useWallet } from "@solana/wallet-adapter-react";

interface AddProductProps {
  onSuccess?: () => void;
}

export default function AddProduct({ onSuccess }: AddProductProps) {
  const { addProduct, loading } = useSodap();
  const { connected } = useWallet();

  const [price, setPrice] = useState<number>(0);
  const [stock, setStock] = useState<number>(0);
  const [tokenizedType, setTokenizedType] = useState<"None" | "SplToken">(
    "None"
  );
  const [metadataUri, setMetadataUri] = useState<string>("");
  const [showForm, setShowForm] = useState<boolean>(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    await addProduct(price, stock, tokenizedType, metadataUri);

    // Reset form
    setPrice(0);
    setStock(0);
    setTokenizedType("None");
    setMetadataUri("");
    setShowForm(false);

    if (onSuccess) {
      onSuccess();
    }
  };

  if (!connected) {
    return null;
  }

  return (
    <div className="bg-white p-8 rounded-lg shadow-md mb-8">
      {!showForm ? (
        <div className="flex justify-between items-center">
          <h2 className="text-2xl font-bold">Products</h2>
          <button
            onClick={() => setShowForm(true)}
            className="px-4 py-2 bg-purple-500 text-white rounded hover:bg-purple-600 transition"
          >
            Add New Product
          </button>
        </div>
      ) : (
        <>
          <div className="flex justify-between items-center mb-6">
            <h2 className="text-2xl font-bold">Add New Product</h2>
            <button
              onClick={() => setShowForm(false)}
              className="text-gray-500 hover:text-gray-700"
            >
              Cancel
            </button>
          </div>

          <form onSubmit={handleSubmit}>
            <div className="mb-4">
              <label
                className="block text-gray-700 text-sm font-bold mb-2"
                htmlFor="price"
              >
                Price (SOL)
              </label>
              <input
                id="price"
                type="number"
                step="0.000000001"
                min="0"
                value={price}
                onChange={(e) => setPrice(parseFloat(e.target.value))}
                className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                placeholder="0.0"
                required
              />
            </div>

            <div className="mb-4">
              <label
                className="block text-gray-700 text-sm font-bold mb-2"
                htmlFor="stock"
              >
                Stock Quantity
              </label>
              <input
                id="stock"
                type="number"
                min="1"
                value={stock}
                onChange={(e) => setStock(parseInt(e.target.value))}
                className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                placeholder="1"
                required
              />
            </div>

            <div className="mb-4">
              <label
                className="block text-gray-700 text-sm font-bold mb-2"
                htmlFor="tokenizedType"
              >
                Product Type
              </label>
              <select
                id="tokenizedType"
                value={tokenizedType}
                onChange={(e) =>
                  setTokenizedType(e.target.value as "None" | "SplToken")
                }
                className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                required
              >
                <option value="None">Standard Product</option>
                <option value="SplToken">SPL Token</option>
              </select>
              <p className="mt-1 text-sm text-gray-500">
                SPL Tokens are used for NFT-backed products
              </p>
            </div>

            <div className="mb-6">
              <label
                className="block text-gray-700 text-sm font-bold mb-2"
                htmlFor="metadataUri"
              >
                Metadata URI
              </label>
              <input
                id="metadataUri"
                type="url"
                value={metadataUri}
                onChange={(e) => setMetadataUri(e.target.value)}
                className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                placeholder="https://example.com/metadata.json"
                required
              />
              <p className="mt-1 text-sm text-gray-500">
                URI to JSON metadata containing product details (name,
                description, image)
              </p>
            </div>

            <div className="flex justify-end">
              <button
                type="submit"
                className="bg-purple-500 hover:bg-purple-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                disabled={loading}
              >
                {loading ? "Processing..." : "Add Product"}
              </button>
            </div>
          </form>
        </>
      )}
    </div>
  );
}
