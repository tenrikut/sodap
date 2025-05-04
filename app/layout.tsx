"use client";

import { ChakraProvider, Box } from "@chakra-ui/react";
import { SodapProvider } from "@/contexts/SodapContext";
import SolanaWalletProvider from "@/providers/WalletProvider";
import NavBar from "@/components/NavBar";
import { Inter } from "next/font/google";
import "./globals.css";

const inter = Inter({ subsets: ["latin"] });

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
