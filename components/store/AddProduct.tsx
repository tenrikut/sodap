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
  NumberInput,
  NumberInputField,
  Textarea,
} from "@chakra-ui/react";

export default function AddProduct() {
  const [name, setName] = useState("");
  const [price, setPrice] = useState("");
  const [description, setDescription] = useState("");
  const [loading, setLoading] = useState(false);
  const toast = useToast();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);

    try {
      const response = await fetch("/api/products", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          name,
          price: parseFloat(price),
          description,
        }),
      });

      if (!response.ok) {
        throw new Error("Failed to add product");
      }

      toast({
        title: "Product added successfully",
        status: "success",
        duration: 3000,
        isClosable: true,
      });

      // Reset form
      setName("");
      setPrice("");
      setDescription("");
    } catch (error) {
      toast({
        title: "Error adding product",
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
        Add New Product
      </Heading>
      <form onSubmit={handleSubmit}>
        <VStack spacing={4}>
          <FormControl isRequired>
            <FormLabel>Product Name</FormLabel>
            <Input
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="Enter product name"
            />
          </FormControl>

          <FormControl isRequired>
            <FormLabel>Price</FormLabel>
            <NumberInput
              value={price}
              onChange={(value) => setPrice(value)}
              min={0}
              precision={2}
            >
              <NumberInputField placeholder="Enter price" />
            </NumberInput>
          </FormControl>

          <FormControl isRequired>
            <FormLabel>Description</FormLabel>
            <Textarea
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              placeholder="Enter product description"
            />
          </FormControl>

          <Button
            type="submit"
            colorScheme="purple"
            width="full"
            isLoading={loading}
            mt={4}
          >
            Add Product
          </Button>
        </VStack>
      </form>
    </Box>
  );
}
