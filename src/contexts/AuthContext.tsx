import { createContext, useContext, useState, ReactNode } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface AuthContextType {
  isAuthenticated: boolean;
  password: string | null;
  checkFirstLaunch: () => Promise<boolean>;
  setupPassword: (pwd: string) => Promise<void>;
  login: (pwd: string) => Promise<boolean>;
  logout: () => void;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const AuthProvider = ({ children }: { children: ReactNode }) => {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [password, setPassword] = useState<string | null>(null);

  const checkFirstLaunch = async (): Promise<boolean> => {
    return await invoke('check_first_launch');
  };

  const setupPassword = async (pwd: string): Promise<void> => {
    await invoke('setup_password', { password: pwd });
    setPassword(pwd);
    setIsAuthenticated(true);
  };

  const login = async (pwd: string): Promise<boolean> => {
    const isValid = await invoke<boolean>('verify_password_cmd', { password: pwd });
    if (isValid) {
      setPassword(pwd);
      setIsAuthenticated(true);
    }
    return isValid;
  };

  const logout = () => {
    setPassword(null);
    setIsAuthenticated(false);
  };

  return (
    <AuthContext.Provider value={{ isAuthenticated, password, checkFirstLaunch, setupPassword, login, logout }}>
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (!context) throw new Error('useAuth must be used within AuthProvider');
  return context;
};
