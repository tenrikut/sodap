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
import { useContext, useEffect, useState } from "react";
import { SodapContext } from "../contexts/SodapContext";

const NavBar = () => {
  const context = useContext(SodapContext);
  const userRole = context?.userRole;
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
  }, []);

  if (!mounted) {
    return null;
  }

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
            {/* Admin links, role-based gating */}
            {(userRole === "platform_admin" ||
              userRole === "super_root_admin" ||
              userRole === "store_owner" ||
              userRole === "store_admin") && (
              <>
                {/* Product Management for all admin roles */}
                <Link
                  as={NextLink}
                  href="/admin/products"
                  px={2}
                  py={1}
                  rounded="md"
                >
                  Product Management
                </Link>
                {/* Platform Admin Management for platform_admin or super_root_admin */}
                {(userRole === "platform_admin" ||
                  userRole === "super_root_admin") && (
                  <Link
                    as={NextLink}
                    href="/admin/platform-admins"
                    px={2}
                    py={1}
                    rounded="md"
                  >
                    Platform Admin Management
                  </Link>
                )}
                {/* Store Owner Management for platform_admin only */}
                {userRole === "platform_admin" && (
                  <Link
                    as={NextLink}
                    href="/admin/store-owners"
                    px={2}
                    py={1}
                    rounded="md"
                  >
                    Store Owner Management
                  </Link>
                )}
                {/* Store Admin Management for store_owner only */}
                {userRole === "store_owner" && (
                  <Link
                    as={NextLink}
                    href="/admin/store-admins"
                    px={2}
                    py={1}
                    rounded="md"
                  >
                    Store Admin Management
                  </Link>
                )}
              </>
            )}
          </HStack>
        </HStack>
        <WalletConnect />
      </Flex>
    </Box>
  );
};

export default NavBar;
