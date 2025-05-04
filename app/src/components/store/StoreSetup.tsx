"use client";

import { useState, useContext } from "react";
import { SodapContext } from "../../contexts/SodapContext";

export default function StoreSetup() {
  const { createStore, store } = useContext(SodapContext);
  const [storeName, setStoreName] = useState(store?.name || "");
  const [storeDescription, setStoreDescription] = useState(
    store?.description || ""
  );
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState("");
  const [success, setSuccess] = useState("");

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsLoading(true);
    setError("");
    setSuccess("");

    try {
      await createStore(storeName, storeDescription);
      setSuccess("Store created successfully!");
    } catch (error) {
      setError("Failed to create store. Please try again.");
      console.error(error);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div>
      <h2 className="text-xl font-semibold mb-4">Store Setup</h2>

      {error && (
        <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
          {error}
        </div>
      )}

      {success && (
        <div className="bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded mb-4">
          {success}
        </div>
      )}

      <form onSubmit={handleSubmit} className="space-y-4">
        <div>
          <label
            htmlFor="storeName"
            className="block text-sm font-medium text-gray-700 mb-1"
          >
            Store Name
          </label>
          <input
            type="text"
            id="storeName"
            value={storeName}
            onChange={(e) => setStoreName(e.target.value)}
            className="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
            required
          />
        </div>

        <div>
          <label
            htmlFor="storeDescription"
            className="block text-sm font-medium text-gray-700 mb-1"
          >
            Store Description
          </label>
          <textarea
            id="storeDescription"
            value={storeDescription}
            onChange={(e) => setStoreDescription(e.target.value)}
            rows={4}
            className="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
          />
        </div>

        <button
          type="submit"
          className="bg-blue-600 text-white py-2 px-4 rounded hover:bg-blue-700 transition disabled:opacity-50"
          disabled={isLoading}
        >
          {isLoading ? "Processing..." : "Save Store Settings"}
        </button>
      </form>
    </div>
  );
}
