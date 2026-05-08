import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { AuthProvider, useAuth } from './contexts/AuthContext';
import { AppProvider, useApp } from './contexts/AppContext';
import { CreatePasswordScreen } from './components/CreatePasswordScreen';
import { LoginScreen } from './components/LoginScreen';
import { MainLayout } from './components/MainLayout';
import { DatabaseMissingModal } from './components/DatabaseMissingModal';
import { MigrationModal } from './components/MigrationModal';
import { DevTools } from './components/DevTools';
import './App.css';

function AppContent() {
  const { isAuthenticated, databaseMissing, checkFirstLaunch } = useAuth();
  const { initialLoading } = useApp();
  const [isFirstLaunch, setIsFirstLaunch] = useState<boolean | null>(null);
  const [showMigration, setShowMigration] = useState(false);

  useEffect(() => {
    checkFirstLaunch().then((firstLaunch) => {
      setIsFirstLaunch(firstLaunch);
      if (!firstLaunch) {
        checkMigrationStatus();
      }
    });
  }, []);

  const checkMigrationStatus = async () => {
    try {
      const needsMigration = await invoke<boolean>('needs_migration');
      setShowMigration(needsMigration);
    } catch (error) {
      console.error('Error checking migration status:', error);
    }
  };

  const handleMigrationComplete = () => {
    setShowMigration(false);
  };

  if (isFirstLaunch === null) {
    return <div className="min-h-screen bg-dark-bg flex items-center justify-center text-dark-text-primary">Carregando...</div>;
  }

  if (isFirstLaunch) {
    return <CreatePasswordScreen />;
  }

  if (showMigration) {
    return <MigrationModal onComplete={handleMigrationComplete} />;
  }

  if (!isAuthenticated) {
    return <LoginScreen />;
  }

  if (initialLoading) {
    return (
      <div className="min-h-screen bg-dark-bg flex items-center justify-center">
        <div className="text-center">
          <div className="mb-4">
            <div className="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-dark-accent"></div>
          </div>
          <p className="text-dark-text-primary">Carregando dados...</p>
        </div>
      </div>
    );
  }

  return (
    <>
      <MainLayout />
      {databaseMissing && <DatabaseMissingModal />}
      <DevTools />
    </>
  );
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
