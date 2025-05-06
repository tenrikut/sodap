"use client";

import { useState } from "react";
import { useWallet } from "@solana/wallet-adapter-react";
import { useShoppingCart } from "../../contexts/ShoppingCartContext";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import Receipt from "./Receipt";

interface ShoppingCartProps {
  storeId: string;
}

export default function ShoppingCart({ storeId }: ShoppingCartProps) {
  const [isCheckingOut, setIsCheckingOut] = useState(false);
  const [receipt, setReceipt] = useState(null);
  const { publicKey, connected } = useWallet();

  const {
    cartItems,
    subtotal,
    totalItems,
    updateQuantity,
    removeFromCart,
    checkout,
    loyaltyBalance,
  } = useShoppingCart();

  const handleCheckout = async () => {
    if (!connected) {
      alert("Please connect your wallet first");
      return;
    }

    if (cartItems.length === 0) {
      alert("Your cart is empty");
      return;
    }

    setIsCheckingOut(true);
    try {
      const receiptData = await checkout();
      if (receiptData) {
        setReceipt(receiptData);
      }
    } catch (error) {
      console.error("Checkout failed:", error);
      alert("Checkout failed. Please try again.");
    } finally {
      setIsCheckingOut(false);
    }
  };

  if (receipt) {
    return <Receipt receipt={receipt} onClose={() => setReceipt(null)} />;
  }

  return (
    <div className="border rounded-lg p-4 bg-white">
      <div className="flex justify-between items-center mb-4">
        <h3 className="text-lg font-medium">Your Cart ({totalItems} items)</h3>

        <div className="text-sm bg-blue-100 text-blue-800 px-2 py-1 rounded-md">
          Your Loyalty Points: {loyaltyBalance}
        </div>
      </div>

      {!connected ? (
        <div className="text-center py-6">
          <p className="text-gray-600 mb-4">Connect your wallet to checkout</p>
          <WalletMultiButton />
        </div>
      ) : cartItems.length === 0 ? (
        <div className="text-center py-8">
          <p className="text-gray-500">Your cart is empty</p>
          <p className="text-gray-500 mt-2">
            Scan products to add them to your cart
          </p>
        </div>
      ) : (
        <>
          <div className="max-h-60 overflow-y-auto mb-4">
            {cartItems.map((item) => (
              <div
                key={item.id}
                className="flex items-center justify-between py-2 border-b"
              >
                <div className="flex items-center">
                  {item.imageUrl && (
                    <img
                      src={item.imageUrl}
                      alt={item.name}
                      className="w-10 h-10 object-cover rounded-md mr-3"
                    />
                  )}
                  <div>
                    <div className="font-medium">{item.name}</div>
                    <div className="text-sm text-gray-500">
                      {item.price} SOL each
                    </div>
                  </div>
                </div>

                <div className="flex items-center">
                  <div className="flex items-center border rounded-md mr-3">
                    <button
                      onClick={() => updateQuantity(item.id, item.quantity - 1)}
                      className="px-2 py-1 text-gray-600 hover:bg-gray-100"
                    >
                      -
                    </button>
                    <span className="px-2">{item.quantity}</span>
                    <button
                      onClick={() => updateQuantity(item.id, item.quantity + 1)}
                      className="px-2 py-1 text-gray-600 hover:bg-gray-100"
                    >
                      +
                    </button>
                  </div>

                  <button
                    onClick={() => removeFromCart(item.id)}
                    className="text-red-500 hover:text-red-700"
                  >
                    ×
                  </button>
                </div>
              </div>
            ))}
          </div>

          <div className="border-t pt-4">
            <div className="flex justify-between mb-2">
              <span>Subtotal:</span>
              <span className="font-medium">{subtotal.toFixed(3)} SOL</span>
            </div>

            <div className="flex justify-between mb-4">
              <span>Loyalty Rewards:</span>
              <span className="text-green-600">
                +{Math.floor(subtotal * 10)} points
              </span>
            </div>

            <button
              onClick={handleCheckout}
              disabled={isCheckingOut}
              className="w-full bg-green-600 hover:bg-green-700 text-white py-3 rounded-md font-medium transition disabled:opacity-50"
            >
              {isCheckingOut
                ? "Processing..."
                : `Checkout (${subtotal.toFixed(3)} SOL)`}
            </button>
          </div>
        </>
      )}
    </div>
  );
}
