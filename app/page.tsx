"use client";

import {
  Box,
  Button,
  Container,
  Heading,
  Text,
  VStack,
  HStack,
  useColorModeValue,
} from "@chakra-ui/react";
import Link from "next/link";
import { useContext } from "react";
import { SodapContext } from "@/contexts/SodapContext";
import { AuthProvider } from "@/contexts/AuthContext";
import AuthForm from "@/components/AuthForm";

export default function Home() {
  const { walletConnected } = useContext(SodapContext);

  return (
    <AuthProvider>
      <AuthForm />
      <Container maxW="container.xl" py={10}>
        <VStack spacing={10} align="start">
          <Box textAlign="center" w="full" py={10}>
            <Heading
              as="h1"
              size="2xl"
              mb={4}
              bgGradient="linear(to-r, purple.500, blue.500)"
              bgClip="text"
            >
              Welcome to SoDap
            </Heading>
            <Text fontSize="xl" maxW="container.md" mx="auto" mb={8}>
              Solana Decentralized Shopping Application - Connecting retailers
              and shoppers with blockchain technology
            </Text>

            {!walletConnected && (
              <Box mb={6}>
                <Text color="orange.500" mb={2}>
                  Please connect your wallet to access all features
                </Text>
              </Box>
            )}

            <HStack spacing={6} justify="center">
              <Link href="/marketplace" passHref>
                <Button as="a" colorScheme="purple" size="lg" px={8}>
                  Explore Marketplace
                </Button>
              </Link>
              <Link href="/admin/products" passHref>
                <Button as="a" colorScheme="blue" size="lg" px={8}>
                  Manage Products
                </Button>
              </Link>
            </HStack>
          </Box>

          <Box w="full">
            <Heading as="h2" size="lg" mb={6}>
              Key Features
            </Heading>

            <HStack spacing={8} flexWrap="wrap" justifyContent="center">
              {[
                {
                  title: "Product Management",
                  description:
                    "Add, update, and manage products on the Solana blockchain with secure UUIDs and PDAs",
                  link: "/admin/products",
                },
                {
                  title: "QR Code Scanning",
                  description:
                    "Scan products in-store for quick and seamless purchases",
                  link: "/marketplace",
                },
                {
                  title: "Loyalty Rewards",
                  description:
                    "Earn and redeem loyalty points for discounts on future purchases",
                  link: "/profile",
                },
              ].map((feature, index) => (
                <Box
                  key={index}
                  p={6}
                  bg={useColorModeValue("white", "gray.800")}
                  borderRadius="lg"
                  boxShadow="md"
                  w={{ base: "full", md: "30%" }}
                  mb={{ base: 4, md: 0 }}
                >
                  <Heading as="h3" size="md" mb={4}>
                    {feature.title}
                  </Heading>
                  <Text mb={4}>{feature.description}</Text>
                  <Link href={feature.link} passHref>
                    <Button
                      as="a"
                      colorScheme="purple"
                      variant="outline"
                      size="sm"
                    >
                      Learn More
                    </Button>
                  </Link>
                </Box>
              ))}
            </HStack>
          </Box>
        </VStack>
      </Container>
    </AuthProvider>
  );
}
