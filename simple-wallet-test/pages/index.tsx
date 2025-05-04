import type { NextPage } from "next";
import { useWallet } from "@solana/wallet-adapter-react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import styles from "../styles/Home.module.css";

const Home: NextPage = () => {
  const { publicKey, connected } = useWallet();

  return (
    <div className={styles.container}>
      <main className={styles.main}>
        <h1 className={styles.title}>
          Welcome to <a href="https://solana.com">Solana</a> Wallet Adapter
          Example!
        </h1>

        <div className={styles.walletButtons}>
          <WalletMultiButton />
        </div>

        {connected && (
          <div className={styles.walletInfo}>
            <h2>Connected Wallet</h2>
            <p>Public Key: {publicKey?.toBase58()}</p>
          </div>
        )}
      </main>
    </div>
  );
};

export default Home;
