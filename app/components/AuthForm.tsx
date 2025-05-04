import React, { useState } from "react";
import { useAuth } from "@/contexts/AuthContext";
import {
  Box,
  Button,
  Input,
  VStack,
  Heading,
  Text,
  Link,
  useToast,
} from "@chakra-ui/react";

const AuthForm: React.FC = () => {
  const { register, login, resetPassword, loading, error } = useAuth();
  const [mode, setMode] = useState<"login" | "register" | "reset">("login");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [message, setMessage] = useState("");
  const toast = useToast();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setMessage("");
    try {
      if (mode === "register") {
        await register(email, password);
        setMessage(
          "Registration successful! Please check your email to confirm."
        );
      } else if (mode === "login") {
        await login(email, password);
        setMessage("Login successful!");
      } else if (mode === "reset") {
        await resetPassword(email);
        setMessage("Password reset email sent!");
      }
    } catch (e) {
      toast({
        title: "Auth error",
        description: (e as Error).message,
        status: "error",
      });
    }
  };

  return (
    <Box maxW="sm" mx="auto" mt={10} p={6} borderWidth={1} borderRadius="md">
      <Heading size="md" mb={4} textAlign="center">
        {mode === "login"
          ? "Login"
          : mode === "register"
          ? "Register"
          : "Reset Password"}
      </Heading>
      <form onSubmit={handleSubmit}>
        <VStack spacing={4} align="stretch">
          <Input
            type="email"
            placeholder="Email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            required
          />
          {mode !== "reset" && (
            <Input
              type="password"
              placeholder="Password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              required
            />
          )}
          {error && <Text color="red.500">{error.message}</Text>}
          {message && <Text color="green.500">{message}</Text>}
          <Button type="submit" colorScheme="blue" isLoading={loading} w="full">
            {mode === "login"
              ? "Login"
              : mode === "register"
              ? "Register"
              : "Send Reset Email"}
          </Button>
        </VStack>
      </form>
      <VStack spacing={2} mt={4}>
        {mode !== "login" && (
          <Link color="blue.500" onClick={() => setMode("login")}>
            Back to Login
          </Link>
        )}
        {mode !== "register" && (
          <Link color="blue.500" onClick={() => setMode("register")}>
            Create an Account
          </Link>
        )}
        {mode !== "reset" && (
          <Link color="blue.500" onClick={() => setMode("reset")}>
            Forgot Password?
          </Link>
        )}
      </VStack>
    </Box>
  );
};

export default AuthForm;
