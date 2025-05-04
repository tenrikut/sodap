import Link from "next/link";

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-center p-6 md:p-24 bg-gradient-to-b from-purple-50 to-blue-50">
      <div className="text-center">
        <h1 className="text-4xl md:text-6xl font-bold mb-6 text-purple-600">
          Welcome to SoDap
        </h1>
        <p className="text-lg md:text-xl text-gray-700 mb-8 max-w-2xl">
          A decentralized shopping platform powered by Solana that connects
          customers and retailers through blockchain technology.
        </p>

        <div className="flex flex-col sm:flex-row justify-center gap-4">
          <a
            href="/dashboard"
            className="px-8 py-3 rounded-md bg-purple-600 text-white font-medium hover:bg-purple-700 transition-colors"
          >
            Store Dashboard
          </a>
          <a
            href="#"
            className="px-8 py-3 rounded-md bg-white border border-gray-300 text-gray-700 font-medium hover:bg-gray-50 transition-colors"
          >
            Shop Now
          </a>
        </div>
      </div>

      <div className="mt-16 grid grid-cols-1 md:grid-cols-3 gap-8 max-w-6xl">
        <div className="bg-white p-6 rounded-lg shadow-md">
          <h2 className="text-xl font-semibold mb-4 text-purple-700">
            For Retailers
          </h2>
          <p className="text-gray-600 mb-4">
            Easily add products to the SoDap platform and let customers scan and
            purchase them directly in-store.
          </p>
          <ul className="list-disc list-inside text-gray-600 space-y-2">
            <li>Manage your store</li>
            <li>Add product listings</li>
            <li>Choose NFT or token format</li>
            <li>Set prices and stock</li>
            <li>Track revenue</li>
          </ul>
        </div>

        <div className="bg-white p-6 rounded-lg shadow-md">
          <h2 className="text-xl font-semibold mb-4 text-blue-700">
            For Customers
          </h2>
          <p className="text-gray-600 mb-4">
            Scan products, add them to your cart, and pay using your Solana
            wallet for a seamless shopping experience.
          </p>
          <ul className="list-disc list-inside text-gray-600 space-y-2">
            <li>Scan items in-store</li>
            <li>Add to cart</li>
            <li>Pay with Solana</li>
            <li>Track purchase history</li>
            <li>Earn loyalty points</li>
          </ul>
        </div>

        <div className="bg-white p-6 rounded-lg shadow-md">
          <h2 className="text-xl font-semibold mb-4 text-green-700">
            For Developers
          </h2>
          <p className="text-gray-600 mb-4">
            Built on Solana with an open architecture that's easy to integrate
            with and extend.
          </p>
          <ul className="list-disc list-inside text-gray-600 space-y-2">
            <li>Anchor framework</li>
            <li>SPL token integration</li>
            <li>Public API</li>
            <li>PDA-based data storage</li>
            <li>Phantom wallet support</li>
          </ul>
        </div>
      </div>
    </main>
  );
}
