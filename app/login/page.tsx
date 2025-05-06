"use client";

import Link from "next/link";
import Image from "next/image";
import { useState } from "react";
import { useRouter } from "next/navigation";

export default function DashboardLoginPage() {
  const [role, setRole] = useState("admin");
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState("");
  const router = useRouter();

  const handleLogin = (e: React.FormEvent) => {
    e.preventDefault();
    setIsLoading(true);
    setTimeout(() => {
      if (
        (role === "admin" &&
          username === "admin" &&
          password === "adminpass") ||
        (role === "manager" &&
          username === "manager" &&
          password === "managerpass")
      ) {
        router.push("/dashboard");
      } else {
        setError("Invalid credentials for selected role.");
        setIsLoading(false);
      }
    }, 800);
  };

  return (
    <div className="min-h-screen w-full flex flex-col items-center justify-center bg-gradient-to-br from-gray-900 via-blue-900 to-gray-800 relative">
      {/* Background image with overlay, always behind form */}
      <div className="absolute inset-0 z-0">
        <Image
          src="/sodap.webp"
          alt="Background"
          fill
          priority
          className="object-cover w-full h-full opacity-40"
        />
        <div className="absolute inset-0 bg-blue-900/60"></div>
      </div>
      <div className="relative z-10 w-full max-w-md px-6 flex flex-col items-center">
        <div className="mb-8">
          <div className="w-12 h-12">
            <svg
              viewBox="0 0 24 24"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
              className="w-full h-full"
            >
              <circle cx="12" cy="12" r="10" fill="#2563eb" />
              <path
                d="M8 15h8v1a3 3 0 01-3 3h-2a3 3 0 01-3-3v-1z"
                fill="#fff"
              />
              <ellipse cx="12" cy="11" rx="4" ry="3" fill="#fff" />
            </svg>
          </div>
        </div>
        <div className="text-center mb-8">
          <h1 className="text-[28px] font-bold text-white tracking-tight">
            Dashboard Login
          </h1>
          <p className="text-blue-200 mt-1">
            Platform Admin & Store Manager/Owner
          </p>
        </div>
        <div className="w-full bg-white/90 backdrop-blur-lg rounded-lg shadow-2xl overflow-hidden">
          <form onSubmit={handleLogin} className="p-6 space-y-4">
            {/* Role selector */}
            <div>
              <label className="block mb-1 text-sm font-medium text-gray-700">
                Login as:
              </label>
              <select
                value={role}
                onChange={(e) => setRole(e.target.value)}
                className="w-full px-3 py-2 rounded-md border border-gray-300 focus:border-blue-500 focus:outline-none text-base bg-white"
              >
                <option value="admin">Platform Admin</option>
                <option value="manager">Store Manager/Owner</option>
              </select>
            </div>
            {/* Username */}
            <div>
              <input
                id="username"
                name="username"
                type="text"
                value={username}
                onChange={(e) => setUsername(e.target.value)}
                placeholder="Username"
                className="w-full px-3 py-3 border-b border-gray-300 focus:border-blue-500 focus:outline-none text-[17px] bg-transparent"
                required
              />
            </div>
            {/* Password */}
            <div>
              <input
                id="password"
                name="password"
                type="password"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                placeholder="Password"
                className="w-full px-3 py-3 border-b border-gray-300 focus:border-blue-500 focus:outline-none text-[17px] bg-transparent"
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
                className="w-full flex items-center justify-center px-4 py-3 bg-blue-700 text-white rounded-md hover:bg-blue-800 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed"
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
        {/* Demo credentials */}
        <div className="mt-8 text-center text-[12px] text-blue-200">
          <p>
            Admin: <span className="font-medium">admin</span> /{" "}
            <span className="font-medium">adminpass</span> <br />
            Manager: <span className="font-medium">manager</span> /{" "}
            <span className="font-medium">managerpass</span>
          </p>
        </div>
      </div>
      {/* Footer */}
      <div className="absolute bottom-4 w-full text-center text-[11px] text-blue-200">
        <p>Dashboard login &copy; 2025 SoDap Inc.</p>
      </div>
    </div>
  );
}
