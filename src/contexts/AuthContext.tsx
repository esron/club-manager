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
    try {
      const result = await invoke<boolean>('check_first_launch');
      console.log('First launch check:', result);
      return result;
    } catch (err) {
      console.error('Error checking first launch:', err);
      throw err;
    }
  };

  const setupPassword = async (pwd: string): Promise<void> => {
    try {
      console.log('Setting up password...');
      await invoke('setup_password', { password: pwd });
      console.log('Password setup successful');
      setPassword(pwd);
      setIsAuthenticated(true);
    } catch (err) {
      console.error('Error in setupPassword:', err);
      throw err;
    }
  };

  const login = async (pwd: string): Promise<boolean> => {
    try {
      console.log('Attempting login...');
      const isValid = await invoke<boolean>('verify_password_cmd', { password: pwd });
      console.log('Login result:', isValid);
      if (isValid) {
        setPassword(pwd);
        setIsAuthenticated(true);
      }
      return isValid;
    } catch (err) {
      console.error('Error in login:', err);
      throw err;
    }
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
