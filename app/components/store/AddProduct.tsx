"use client";

import { useState } from "react";
import {
  Box,
  Button,
  FormControl,
  FormLabel,
  Input,
  NumberInput,
  NumberInputField,
  VStack,
  Heading,
  useToast,
} from "@chakra-ui/react";

export default function AddProduct() {
  const [formData, setFormData] = useState({
    name: "",
    description: "",
    price: "",
    inventory: "",
  });
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
          ...formData,
          price: parseFloat(formData.price),
          inventory: parseInt(formData.inventory),
        }),
      });

      if (!response.ok) {
        throw new Error("Failed to add product");
      }

      toast({
        title: "Product added successfully",
        status: "success",
        duration: 3000,
      });

      setFormData({
        name: "",
        description: "",
        price: "",
        inventory: "",
      });
    } catch (error) {
      toast({
        title: "Error adding product",
        description: (error as Error).message,
        status: "error",
        duration: 3000,
      });
    } finally {
      setLoading(false);
    }
  };

  const handleChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    const { name, value } = e.target;
    setFormData((prev) => ({
      ...prev,
      [name]: value,
    }));
  };

  return (
    <Box maxW="md" mx="auto" mt={8}>
      <Heading size="lg" mb={6}>
        Add New Product
      </Heading>
      <form onSubmit={handleSubmit}>
        <VStack spacing={4}>
          <FormControl isRequired>
            <FormLabel>Product Name</FormLabel>
            <Input
              name="name"
              value={formData.name}
              onChange={handleChange}
              placeholder="Enter product name"
            />
          </FormControl>
          <FormControl>
            <FormLabel>Description</FormLabel>
            <Input
              name="description"
              value={formData.description}
              onChange={handleChange}
              placeholder="Enter product description"
            />
          </FormControl>
          <FormControl isRequired>
            <FormLabel>Price (SOL)</FormLabel>
            <NumberInput min={0}>
              <NumberInputField
                name="price"
                value={formData.price}
                onChange={handleChange}
                placeholder="Enter price in SOL"
              />
            </NumberInput>
          </FormControl>
          <FormControl isRequired>
            <FormLabel>Inventory</FormLabel>
            <NumberInput min={0}>
              <NumberInputField
                name="inventory"
                value={formData.inventory}
                onChange={handleChange}
                placeholder="Enter inventory amount"
              />
            </NumberInput>
          </FormControl>
          <Button
            type="submit"
            colorScheme="purple"
            width="full"
            isLoading={loading}
          >
            Add Product
          </Button>
        </VStack>
      </form>
    </Box>
  );
}
