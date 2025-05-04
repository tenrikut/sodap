import React, {
  createContext,
  useContext,
  useState,
  useEffect,
  ReactNode,
} from "react";
import {
  User as AuthUser,
  Session,
  SupabaseClient,
  AuthChangeEvent,
} from "@supabase/supabase-js";
import { createClient } from "@supabase/supabase-js";
import { AuthContextType, AuthError, AuthSession } from "types/sodap";

const supabaseUrl = process.env.NEXT_PUBLIC_SUPABASE_URL ?? "";
const supabaseAnonKey = process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY ?? "";
const supabase: SupabaseClient = createClient(supabaseUrl, supabaseAnonKey);

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const AuthProvider = ({ children }: { children: ReactNode }) => {
  const [user, setUser] = useState<AuthUser | null>(null);
  const [session, setSession] = useState<AuthSession | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<AuthError | null>(null);

  useEffect(() => {
    const { data: listener } = supabase.auth.onAuthStateChange(
      (event: AuthChangeEvent, session: Session | null) => {
        if (session?.user) {
          setUser(session.user);
          setSession({
            user: {
              id: session.user.id,
              email: session.user.email!,
              createdAt: new Date(session.user.created_at!).getTime(),
            },
            accessToken: session.access_token,
            refreshToken: session.refresh_token!,
            expiresAt: session.expires_at ? session.expires_at * 1000 : 0,
          });
        } else {
          setUser(null);
          setSession(null);
        }
        setLoading(false);
      }
    );
    return () => {
      listener?.subscription.unsubscribe();
    };
  }, []);

  const register = async (email: string, password: string) => {
    setLoading(true);
    setError(null);
    const { error, data } = await supabase.auth.signUp({ email, password });
    if (error) {
      setError({ message: error.message });
    }
    setLoading(false);
  };

  const login = async (email: string, password: string) => {
    setLoading(true);
    setError(null);
    const { error, data } = await supabase.auth.signInWithPassword({
      email,
      password,
    });
    if (error) {
      setError({ message: error.message });
    }
    setLoading(false);
  };

  const logout = async () => {
    setLoading(true);
    setError(null);
    await supabase.auth.signOut();
    setLoading(false);
  };

  const resetPassword = async (email: string) => {
    setLoading(true);
    setError(null);
    const { error } = await supabase.auth.resetPasswordForEmail(email);
    if (error) {
      setError({ message: error.message });
    }
    setLoading(false);
  };

  return (
    <AuthContext.Provider
      value={{
        user: user
          ? {
              id: user.id,
              email: user.email || "",
              createdAt: new Date(user.created_at!).getTime(),
            }
          : null,
        session,
        loading,
        error,
        register,
        login,
        logout,
        resetPassword,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error("useAuth must be used within an AuthProvider");
  }
  return context;
};
