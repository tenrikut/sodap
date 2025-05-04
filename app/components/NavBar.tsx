"use client";
import {
  Box,
  Flex,
  HStack,
  Link,
  Text,
  useColorModeValue,
} from "@chakra-ui/react";
import NextLink from "next/link";
import WalletConnect from "./WalletConnect";

const NavBar = () => {
  return (
    <Box
      as="nav"
      bg={useColorModeValue("white", "gray.800")}
      color={useColorModeValue("gray.600", "white")}
      borderBottom="1px"
      borderBottomColor={useColorModeValue("gray.200", "gray.700")}
      px={4}
      py={3}
      position="fixed"
      width="100%"
      top={0}
      zIndex={10}
    >
      <Flex
        alignItems="center"
        justifyContent="space-between"
        maxW="container.xl"
        mx="auto"
      >
        <HStack spacing={8} alignItems="center">
          <Link as={NextLink} href="/" _hover={{ textDecoration: "none" }}>
            <Text
              fontSize="xl"
              fontWeight="bold"
              color={useColorModeValue("purple.600", "purple.300")}
            >
              SoDap
            </Text>
          </Link>
          <HStack as="nav" spacing={4} display={{ base: "none", md: "flex" }}>
            <Link as={NextLink} href="/marketplace" px={2} py={1} rounded="md">
              Marketplace
            </Link>
            <Link
              as={NextLink}
              href="/admin/products"
              px={2}
              py={1}
              rounded="md"
            >
              Admin
            </Link>
          </HStack>
        </HStack>
        <WalletConnect />
      </Flex>
    </Box>
  );
};

export default NavBar;
 