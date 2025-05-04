import Link from "next/link";

<div className="text-center text-sm text-purple-600">
  <p>
    Don't have an account?{" "}
    <Link
      href="/register"
      className="font-medium text-purple-700 hover:text-purple-800"
    >
      Sign up
    </Link>
  </p>
  <p className="mt-2">
    <Link
      href="/forgot-password"
      className="font-medium text-purple-700 hover:text-purple-800"
    >
      Forgot your password?
    </Link>
  </p>
</div>;
 