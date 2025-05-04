"use client";

import { useState } from "react";
import {
  Box,
  Button,
  FormControl,
  FormLabel,
  Input,
  VStack,
  Heading,
  useToast,
} from "@chakra-ui/react";

interface StoreSetupProps {
  onComplete: () => void;
}

export default function StoreSetup({ onComplete }: StoreSetupProps) {
  const [storeName, setStoreName] = useState("");
  const [loading, setLoading] = useState(false);
  const toast = useToast();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);

    try {
      const response = await fetch("/api/store/setup", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ name: storeName }),
      });

      if (!response.ok) {
        throw new Error("Failed to setup store");
      }

      toast({
        title: "Store setup successful",
        status: "success",
        duration: 3000,
        isClosable: true,
      });

      onComplete();
    } catch (error) {
      toast({
        title: "Store setup failed",
        description:
          error instanceof Error ? error.message : "An error occurred",
        status: "error",
        duration: 5000,
        isClosable: true,
      });
    } finally {
      setLoading(false);
    }
  };

  return (
    <Box maxW="md" mx="auto" mt={8} p={6} borderWidth="1px" borderRadius="lg">
      <Heading size="lg" mb={6} textAlign="center">
        Setup Your Store
      </Heading>
      <form onSubmit={handleSubmit}>
        <VStack spacing={4}>
          <FormControl isRequired>
            <FormLabel>Store Name</FormLabel>
            <Input
              value={storeName}
              onChange={(e) => setStoreName(e.target.value)}
              placeholder="Enter store name"
            />
          </FormControl>

          <Button
            type="submit"
            colorScheme="purple"
            width="full"
            isLoading={loading}
            mt={4}
          >
            Setup Store
          </Button>
        </VStack>
      </form>
    </Box>
  );
}
