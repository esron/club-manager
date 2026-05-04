import { useEffect, useState } from 'react';
import { AuthProvider, useAuth } from './contexts/AuthContext';
import { AppProvider } from './contexts/AppContext';
import { CreatePasswordScreen } from './components/CreatePasswordScreen';
import { LoginScreen } from './components/LoginScreen';
import { MainLayout } from './components/MainLayout';
import './App.css';

function AppContent() {
  const { isAuthenticated, checkFirstLaunch } = useAuth();
  const [isFirstLaunch, setIsFirstLaunch] = useState<boolean | null>(null);

  useEffect(() => {
    checkFirstLaunch().then(setIsFirstLaunch);
  }, []);

  if (isFirstLaunch === null) {
    return <div className="min-h-screen bg-dark-bg flex items-center justify-center text-dark-text-primary">Carregando...</div>;
  }

  if (isFirstLaunch) {
    return <CreatePasswordScreen />;
  }

  if (!isAuthenticated) {
    return <LoginScreen />;
  }

  return <MainLayout />;
}

function App() {
  return (
    <AuthProvider>
      <AppProvider>
        <AppContent />
      </AppProvider>
    </AuthProvider>
  );
}

export default App;
