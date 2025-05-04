"use client";

import { useState, useEffect } from "react";
import { useSodap } from "@/contexts/SodapContext";
import { useWallet } from "@solana/wallet-adapter-react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";

export default function StoreSetup() {
  const { userStore, registerStore, updateStore, loading } = useSodap();
  const { connected } = useWallet();

  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [logoUri, setLogoUri] = useState("");
  const [isEditing, setIsEditing] = useState(false);

  useEffect(() => {
    if (userStore) {
      setName(userStore.name);
      setDescription(userStore.description);
      setLogoUri(userStore.logoUri);
    }
  }, [userStore]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (isEditing && userStore) {
      await updateStore(name, description, logoUri);
      setIsEditing(false);
    } else {
      await registerStore(name, description, logoUri);
    }
  };

  if (!connected) {
    return (
      <div className="flex flex-col items-center justify-center p-8 bg-gray-50 rounded-lg shadow-md">
        <h2 className="text-2xl font-bold mb-6">
          Connect your wallet to continue
        </h2>
        <WalletMultiButton />
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

  if (userStore && !isEditing) {
    return (
      <div className="bg-white p-8 rounded-lg shadow-md">
        <div className="flex justify-between items-center mb-6">
          <h2 className="text-2xl font-bold">Store Information</h2>
          <button
            onClick={() => setIsEditing(true)}
            className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition"
          >
            Edit Store
          </button>
        </div>

        <div className="mb-6">
          <div className="flex items-center mb-4">
            {logoUri && (
              <img
                src={logoUri}
                alt={name}
                className="w-16 h-16 rounded-full mr-4 object-cover"
              />
            )}
            <div>
              <h3 className="text-xl font-semibold">{name}</h3>
              <p className="text-gray-500">Store Owner</p>
            </div>
          </div>

          <div className="mt-4">
            <h4 className="text-lg font-medium mb-2">Description</h4>
            <p className="text-gray-700">{description}</p>
          </div>

          <div className="mt-4">
            <h4 className="text-lg font-medium mb-2">Store Statistics</h4>
            <div className="grid grid-cols-2 gap-4">
              <div className="bg-gray-50 p-4 rounded">
                <p className="text-gray-500">Total Revenue</p>
                <p className="text-xl font-bold">{userStore.revenue} SOL</p>
              </div>
              <div className="bg-gray-50 p-4 rounded">
                <p className="text-gray-500">Products</p>
                <p className="text-xl font-bold">0</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="bg-white p-8 rounded-lg shadow-md">
      <h2 className="text-2xl font-bold mb-6">
        {isEditing ? "Edit Store" : "Register Your Store"}
      </h2>

      <form onSubmit={handleSubmit}>
        <div className="mb-4">
          <label
            className="block text-gray-700 text-sm font-bold mb-2"
            htmlFor="name"
          >
            Store Name
          </label>
          <input
            id="name"
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
            className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
            placeholder="Enter store name"
            required
          />
        </div>

        <div className="mb-4">
          <label
            className="block text-gray-700 text-sm font-bold mb-2"
            htmlFor="description"
          >
            Description
          </label>
          <textarea
            id="description"
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
            placeholder="Enter store description"
            rows={3}
            required
          />
        </div>

        <div className="mb-6">
          <label
            className="block text-gray-700 text-sm font-bold mb-2"
            htmlFor="logoUri"
          >
            Logo URL
          </label>
          <input
            id="logoUri"
            type="url"
            value={logoUri}
            onChange={(e) => setLogoUri(e.target.value)}
            className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
            placeholder="https://example.com/logo.png"
            required
          />
        </div>

        <div className="flex items-center justify-between">
          {isEditing && (
            <button
              type="button"
              onClick={() => setIsEditing(false)}
              className="bg-gray-300 hover:bg-gray-400 text-gray-800 font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
            >
              Cancel
            </button>
          )}
          <button
            type="submit"
            className="bg-purple-500 hover:bg-purple-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
            disabled={loading}
          >
            {loading
              ? "Processing..."
              : isEditing
              ? "Update Store"
              : "Register Store"}
          </button>
        </div>
      </form>
    </div>
  );
}
