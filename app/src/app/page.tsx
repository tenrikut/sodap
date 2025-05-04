import Link from "next/link";
import { SodapProvider } from "../contexts/SodapContext";

export default function Home() {
  return (
    <SodapProvider>
      <div className="container mx-auto px-4 py-8">
        <h1 className="text-3xl font-bold mb-6">
          SoDap - Solana Decentralized Marketplace
        </h1>

        <div className="bg-white shadow-md rounded-lg p-6 mb-6">
          <h2 className="text-2xl font-semibold mb-4">Welcome to SoDap</h2>
          <p className="mb-4">
            Your decentralized shopping platform on Solana blockchain.
          </p>

          <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mt-6">
            <Link
              href="/dashboard"
              className="bg-blue-600 text-white py-3 px-6 rounded-lg text-center hover:bg-blue-700 transition"
            >
              Store Dashboard
            </Link>
            <Link
              href="/marketplace"
              className="bg-purple-600 text-white py-3 px-6 rounded-lg text-center hover:bg-purple-700 transition"
            >
              Shop Now
            </Link>
            <Link
              href="#"
              className="bg-green-600 text-white py-3 px-6 rounded-lg text-center hover:bg-green-700 transition opacity-60"
            >
              Loyalty Rewards (Coming Soon)
            </Link>
          </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div className="bg-white shadow-md rounded-lg p-6">
            <h2 className="text-xl font-semibold mb-4">For Retailers</h2>
            <p className="text-gray-600 mb-4">
              Create your store, add products, and start selling with Solana
              blockchain.
            </p>
            <ul className="list-disc list-inside text-gray-600 space-y-1">
              <li>Manage your store profile</li>
              <li>List products with images and details</li>
              <li>Track inventory and sales</li>
              <li>Create loyalty programs</li>
            </ul>
          </div>

          <div className="bg-white shadow-md rounded-lg p-6">
            <h2 className="text-xl font-semibold mb-4">For Shoppers</h2>
            <p className="text-gray-600 mb-4">
              Shop in-store or online with your Solana wallet and earn rewards.
            </p>
            <ul className="list-disc list-inside text-gray-600 space-y-1">
              <li>Scan QR codes for easy shopping</li>
              <li>Pay with Solana</li>
              <li>Earn loyalty tokens</li>
              <li>Access exclusive NFT rewards</li>
            </ul>
          </div>
        </div>
      </div>
    </SodapProvider>
  );
}
