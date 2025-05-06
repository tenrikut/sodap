"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import Link from "next/link";
import Image from "next/image";

export default function LoginPage() {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState("");
  const router = useRouter();

  const handleLogin = (e: React.FormEvent) => {
    e.preventDefault();
    setIsLoading(true);

    // Simulate authentication process
    setTimeout(() => {
      if (username === "sodap" && password === "sodap") {
        router.push("/marketplace");
      } else {
        setError("Your SoDap ID or password was incorrect.");
        setIsLoading(false);
      }
    }, 800);
  };

  return (
    <div className="min-h-screen w-full flex flex-col items-center justify-center bg-[#f5f5f7] relative">
      {/* Background image with overlay, always behind form */}
      <div className="absolute inset-0 z-0">
        <Image
          src="/sodap.webp"
          alt="Background"
          fill
          priority
          className="object-cover w-full h-full"
        />
        <div className="absolute inset-0 bg-black/20"></div>
      </div>

      {/* Content container, always above background */}
      <div className="relative z-10 w-full max-w-md px-6 flex flex-col items-center">
        {/* Logo */}
        <div className="mb-8">
          <div className="w-10 h-10">
            <svg
              viewBox="0 0 24 24"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
              className="w-full h-full"
            >
              <path
                d="M12 2C6.48 2 2 6.48 2 12C2 17.52 6.48 22 12 22C17.52 22 22 17.52 22 12C22 6.48 17.52 2 12 2ZM12 20C7.59 20 4 16.41 4 12C4 7.59 7.59 4 12 4C16.41 4 20 7.59 20 12C20 16.41 16.41 20 12 20Z"
                fill="#000000"
              />
              <path
                d="M12 17C14.7614 17 17 14.7614 17 12C17 9.23858 14.7614 7 12 7C9.23858 7 7 9.23858 7 12C7 14.7614 9.23858 17 12 17Z"
                fill="#000000"
              />
            </svg>
          </div>
        </div>

        {/* Sign in text */}
        <div className="text-center mb-8">
          <h1 className="text-[32px] font-medium text-gray-900 tracking-tight">
            Sign in with your SoDap ID
          </h1>
        </div>

        {/* Login form - solid white, no blur, dark text */}
        <div className="w-full bg-white rounded-lg shadow-2xl overflow-hidden">
          <form onSubmit={handleLogin} className="p-6 space-y-4">
            {/* Username field */}
            <div>
              <input
                id="username"
                name="username"
                type="text"
                value={username}
                onChange={(e) => setUsername(e.target.value)}
                placeholder="SoDap ID"
                className="w-full px-3 py-3 border-b border-gray-300 focus:border-blue-500 focus:outline-none text-[17px] bg-transparent text-gray-900"
                required
              />
            </div>

            {/* Password field */}
            <div>
              <input
                id="password"
                name="password"
                type="password"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                placeholder="Password"
                className="w-full px-3 py-3 border-b border-gray-300 focus:border-blue-500 focus:outline-none text-[17px] bg-transparent text-gray-900"
                required
              />
            </div>

            {/* Error message */}
            {error && (
              <div className="text-red-500 text-[13px] text-center">
                {error}
              </div>
            )}

            {/* Login button */}
            <div className="pt-4">
              <button
                type="submit"
                disabled={isLoading || !username || !password}
                className="w-full flex items-center justify-center px-4 py-3 bg-blue-500 text-white rounded-md hover:bg-blue-600 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed"
              >
                <span className="text-[17px] font-medium">
                  {isLoading ? (
                    <div className="flex items-center">
                      <svg
                        className="animate-spin h-4 w-4 mr-2"
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                      >
                        <circle
                          className="opacity-25"
                          cx="12"
                          cy="12"
                          r="10"
                          stroke="currentColor"
                          strokeWidth="4"
                        ></circle>
                        <path
                          className="opacity-75"
                          fill="currentColor"
                          d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                        ></path>
                      </svg>
                      Signing in...
                    </div>
                  ) : (
                    "Sign in"
                  )}
                </span>
              </button>
            </div>
          </form>
        </div>

        {/* Forgot ID or password */}
        <div className="mt-4 text-center">
          <Link
            href="/forgot-password"
            className="text-[13px] text-blue-500 hover:underline"
          >
            Forgot SoDap ID or password?
          </Link>
        </div>

        {/* Divider */}
        <div className="my-8 w-full flex items-center">
          <div className="flex-grow border-t border-gray-300"></div>
        </div>

        {/* Create account */}
        <div className="text-center">
          <p className="text-[14px] text-gray-600 mb-2">
            Don't have a SoDap ID?
          </p>
          <Link
            href="/register"
            className="text-[14px] text-blue-500 hover:underline"
          >
            Create one now
          </Link>
        </div>

        {/* Demo credentials */}
        <div className="mt-8 text-center text-[12px] text-gray-500">
          <p>
            Demo credentials: SoDap ID:{" "}
            <span className="font-medium">sodap</span>, Password:{" "}
            <span className="font-medium">sodap</span>
          </p>
        </div>
      </div>

      {/* Footer */}
      <div className="absolute bottom-4 w-full text-center text-[11px] text-gray-500">
        <p>Copyright 2025 SoDap Inc. All rights reserved.</p>
      </div>
    </div>
  );
}
