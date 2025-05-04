import "../globals.css";
import { Inter } from "next/font/google";
import { SodapProvider } from "@/contexts/SodapContext";
import SolanaWalletProvider from "../../providers/WalletProvider";
import { ChakraProvider, Box } from "@chakra-ui/react";
import NavBar from "../../components/NavBar";

const inter = Inter({ subsets: ["latin"] });

export const metadata = {
  title: "SoDap - Solana Decentralized Application",
  description: "Decentralized shopping powered by Solana",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <ChakraProvider>
          <SolanaWalletProvider>
            <SodapProvider>
              <NavBar />
              <Box as="main" pt="70px" px={4} maxW="container.xl" mx="auto">
                {children}
              </Box>
            </SodapProvider>
          </SolanaWalletProvider>
        </ChakraProvider>
      </body>
    </html>
  );
}
