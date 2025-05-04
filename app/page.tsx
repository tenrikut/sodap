"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import Link from "next/link";

export default function LoginPage() {
  const [username, setUsername] = useState("sodap");
  const [password, setPassword] = useState("sodap");
  const [error, setError] = useState("");
  const router = useRouter();

  const handleLogin = async (e: React.FormEvent) => {
    e.preventDefault();
    if (username === "sodap" && password === "sodap") {
      console.log("Login successful with demo credentials");
      router.push("/marketplace");
    } else {
      setError(
        "Invalid credentials. Use 'sodap' for both username and password."
      );
    }
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-purple-50 via-white to-purple-100">
      <div className="absolute inset-0 overflow-hidden">
        <div className="absolute -top-40 -right-40 w-80 h-80 bg-purple-300 rounded-full opacity-20 blur-3xl"></div>
        <div className="absolute -bottom-40 -left-40 w-80 h-80 bg-purple-400 rounded-full opacity-20 blur-3xl"></div>
      </div>

      <div className="relative w-full max-w-md p-8 space-y-8 bg-white/80 backdrop-blur-sm rounded-2xl shadow-xl">
        <div className="text-center">
          <div className="flex justify-center mb-4">
            <div className="w-16 h-16 bg-purple-600 rounded-full flex items-center justify-center">
              <span className="text-white text-2xl font-bold">S</span>
            </div>
          </div>
          <h1 className="text-4xl font-bold text-purple-900 mb-2">
            Welcome to SoDap
          </h1>
          <p className="text-purple-600 text-lg">
            SoDap -- Your Digital Shopping Assistant
          </p>
        </div>

        <form className="mt-8 space-y-6" onSubmit={handleLogin}>
          <div className="space-y-4">
            <div>
              <input
                id="username"
                name="username"
                type="text"
                required
                className="w-full px-4 py-3 rounded-xl border border-purple-200 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent transition-all duration-200"
                placeholder="Username"
                value={username}
                onChange={(e) => setUsername(e.target.value)}
              />
            </div>
            <div>
              <input
                id="password"
                name="password"
                type="password"
                required
                className="w-full px-4 py-3 rounded-xl border border-purple-200 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent transition-all duration-200"
                placeholder="Password"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
              />
            </div>
          </div>

          {error && (
            <div className="p-3 bg-red-50 text-red-500 text-sm rounded-lg text-center">
              {error}
            </div>
          )}

          <div>
            <button
              type="submit"
              className="w-full py-3 px-4 bg-purple-600 hover:bg-purple-700 text-white font-medium rounded-xl transition-all duration-200 transform hover:scale-[1.02] active:scale-[0.98] shadow-lg hover:shadow-purple-200"
            >
              Sign In
            </button>
          </div>
        </form>

        <div className="text-center text-sm text-purple-600">
          <p>
            Don't have an account?{" "}
            <Link
              href="/register"
              className="font-medium text-purple-700 hover:text-purple-800"
            >
              Create one
            </Link>
          </p>
        </div>

        <div className="text-center text-sm text-purple-600 bg-purple-50 p-3 rounded-lg">
          <p>Demo credentials: username: sodap, password: sodap</p>
        </div>
      </div>
    </div>
  );
}
