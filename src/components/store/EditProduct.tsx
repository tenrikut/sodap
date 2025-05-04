"use client";

import { useState, useEffect } from "react";
import { useSodap } from "@/contexts/SodapContext";

interface Product {
  productUuid: Uint8Array;
  storeId: any;
  price: number;
  stock: number;
  tokenizedType: { none?: {} } | { splToken?: {} };
  metadataUri: string;
  createdAt: number;
  mintStatus: { notMinted?: {} } | { minted?: {} } | { failed?: {} };
  deactivated: boolean;
}

interface EditProductProps {
  product: Product;
  onClose: () => void;
  onSuccess: () => void;
}

export default function EditProduct({
  product,
  onClose,
  onSuccess,
}: EditProductProps) {
  const { updateProduct, loading } = useSodap();

  const [price, setPrice] = useState<number>(0);
  const [stock, setStock] = useState<number>(0);
  const [tokenizedType, setTokenizedType] = useState<"None" | "SplToken">(
    "None"
  );
  const [metadataUri, setMetadataUri] = useState<string>("");

  useEffect(() => {
    // Initialize form with product data
    setPrice(product.price);
    setStock(product.stock);
    setTokenizedType(product.tokenizedType.none ? "None" : "SplToken");
    setMetadataUri(product.metadataUri);
  }, [product]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    await updateProduct(
      product.productUuid,
      price !== product.price ? price : undefined,
      stock !== product.stock ? stock : undefined,
      metadataUri !== product.metadataUri ? metadataUri : undefined,
      tokenizedType !== (product.tokenizedType.none ? "None" : "SplToken")
        ? tokenizedType
        : undefined
    );

    onSuccess();
    onClose();
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white rounded-lg p-8 w-full max-w-md">
        <div className="flex justify-between items-center mb-6">
          <h2 className="text-2xl font-bold">Edit Product</h2>
          <button
            onClick={onClose}
            className="text-gray-500 hover:text-gray-700"
          >
            ✕
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
              min="0"
              value={stock}
              onChange={(e) => setStock(parseInt(e.target.value))}
              className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
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
              required
            />
          </div>

          <div className="flex justify-end space-x-4">
            <button
              type="button"
              onClick={onClose}
              className="bg-gray-300 hover:bg-gray-400 text-gray-800 font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
            >
              Cancel
            </button>
            <button
              type="submit"
              className="bg-purple-500 hover:bg-purple-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
              disabled={loading}
            >
              {loading ? "Updating..." : "Update Product"}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
