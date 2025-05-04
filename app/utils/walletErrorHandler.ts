import {
  WalletError,
  WalletNotConnectedError,
  WalletNotReadyError,
} from "@solana/wallet-adapter-base";

/**
 * Helper function to handle wallet errors and provide user-friendly messages
 */
export function handleWalletError(error: unknown): string {
  console.error("Wallet error:", error);

  if (error instanceof WalletNotConnectedError) {
    return "Please connect your wallet to continue.";
  }

  if (error instanceof WalletNotReadyError) {
    return "Wallet not ready. Please check if Phantom is installed and unlocked.";
  }

  if (error instanceof WalletError) {
    return `Wallet error: ${error.message}`;
  }

  if (error instanceof Error) {
    if (error.message.includes("User rejected")) {
      return "Connection rejected. Please approve the connection request in your wallet.";
    }

    if (error.message.includes("timeout")) {
      return "Connection timed out. Please try again.";
    }

    return `Error: ${error.message}`;
  }

  return "An unexpected error occurred. Please try again or use a different wallet.";
}

/**
 * Helper function to detect if Phantom wallet is installed
 */
export function isPhantomInstalled(): boolean {
  const phantom = (window as any)?.phantom;
  return phantom && phantom.solana && phantom.solana.isPhantom;
}

/**
 * Helper to get wallet adapter network name
 */
export function getNetworkName(network: string): string {
  switch (network) {
    case "mainnet-beta":
      return "Mainnet";
    case "testnet":
      return "Testnet";
    case "devnet":
      return "Devnet";
    case "localnet":
    default:
      return "Local Network";
  }
}
