"use client";

import { useState } from 'react';
import { useCart } from '../contexts/CartContext';

export interface PaymentResult {
  success: boolean;
  transactionId?: string;
  error?: string;
}

export function usePayment() {
  const [isProcessing, setIsProcessing] = useState(false);
  const [paymentResult, setPaymentResult] = useState<PaymentResult | null>(null);
  const { items, clearCart } = useCart();

  const handlePayment = async (): Promise<PaymentResult> => {
    setIsProcessing(true);
    setPaymentResult(null);
    
    try {
      // Calculate total amount
      const totalAmount = items.reduce((sum: number, item: any) => sum + item.price * item.quantity, 0);
      
      // For demo purposes, simulate a payment process
      await new Promise(resolve => setTimeout(resolve, 1500));
      
      // Generate a mock transaction ID
      const transactionId = `tx-${Math.random().toString(36).substring(2, 15)}`;
      
      // Simulate successful payment
      const result: PaymentResult = {
        success: true,
        transactionId
      };
      
      setPaymentResult(result);
      
      // Clear the cart after successful payment
      clearCart();
      
      return result;
    } catch (error) {
      // Handle payment errors
      const errorResult: PaymentResult = {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown payment error'
      };
      
      setPaymentResult(errorResult);
      return errorResult;
    } finally {
      setIsProcessing(false);
    }
  };

  return {
    handlePayment,
    isProcessing,
    paymentResult
  };
}
