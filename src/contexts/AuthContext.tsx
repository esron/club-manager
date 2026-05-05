import { createContext, useContext, useState, ReactNode } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface AuthContextType {
  isAuthenticated: boolean;
  password: string | null;
  databaseMissing: boolean;
  checkFirstLaunch: () => Promise<boolean>;
  setupPassword: (pwd: string) => Promise<void>;
  login: (pwd: string) => Promise<boolean>;
  logout: () => void;
  initializeDatabase: () => Promise<void>;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const AuthProvider = ({ children }: { children: ReactNode }) => {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [password, setPassword] = useState<string | null>(null);
  const [databaseMissing, setDatabaseMissing] = useState(false);

  const checkFirstLaunch = async (): Promise<boolean> => {
    try {
      const result = await invoke<boolean>('check_first_launch');
      return result;
    } catch (err) {
      console.error('Error checking first launch:', err);
      throw err;
    }
  };

  const setupPassword = async (pwd: string): Promise<void> => {
    try {
      await invoke('setup_password', { password: pwd });
      setPassword(pwd);
      setIsAuthenticated(true);
    } catch (err) {
      console.error('Error in setupPassword:', err);
      throw err;
    }
  };

  const login = async (pwd: string): Promise<boolean> => {
    try {
      const isValid = await invoke<boolean>('verify_password_cmd', { password: pwd });
      if (isValid) {
        setPassword(pwd);

        // Check if database is initialized
        const dbInitialized = await invoke<boolean>('check_database_initialized', { password: pwd });

        if (!dbInitialized) {
          setDatabaseMissing(true);
        }

        setIsAuthenticated(true);
      }
      return isValid;
    } catch (err) {
      console.error('Error in login:', err);
      throw err;
    }
  };

  const initializeDatabase = async (): Promise<void> => {
    if (!password) throw new Error('Not authenticated');
    try {
      await invoke('initialize_database', { password });
      setDatabaseMissing(false);
    } catch (err) {
      console.error('Error initializing database:', err);
      throw err;
    }
  };

  const logout = () => {
    setPassword(null);
    setIsAuthenticated(false);
  };

  return (
    <AuthContext.Provider value={{ isAuthenticated, password, databaseMissing, checkFirstLaunch, setupPassword, login, logout, initializeDatabase }}>
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (!context) throw new Error('useAuth must be used within AuthProvider');
  return context;
};
