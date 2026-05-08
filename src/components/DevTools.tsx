import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useAuth } from '../contexts/AuthContext';

export const DevTools = () => {
  const { password } = useAuth();
  const [isSeeding, setIsSeeding] = useState(false);
  const [isClearing, setIsClearing] = useState(false);
  const [result, setResult] = useState('');

  const handleClear = async () => {
    if (!password) {
      setResult('Error: Not authenticated');
      return;
    }

    if (!confirm('Are you sure you want to clear all members and payments?')) {
      return;
    }

    setIsClearing(true);
    setResult('');

    try {
      const res = await invoke<string>('clear_database', { password });
      setResult(res);
    } catch (err) {
      console.error('Clear error:', err);
      setResult(`Error: ${err}`);
    } finally {
      setIsClearing(false);
    }
  };

  const handleSeed = async () => {
    if (!password) {
      setResult('Error: Not authenticated');
      return;
    }

    setIsSeeding(true);
    setResult('');

    try {
      const res = await invoke<string>('seed_database', { password });
      setResult(res);
    } catch (err) {
      console.error('Seed error:', err);
      setResult(`Error: ${err}`);
    } finally {
      setIsSeeding(false);
    }
  };

  // Only show in development
  if (import.meta.env.PROD) {
    return null;
  }

  return (
    <div className="fixed bottom-4 right-4 bg-dark-warning text-black p-4 rounded shadow-lg border border-black max-w-md">
      <h3 className="font-bold mb-2">🛠️ Dev Tools</h3>
      <div className="space-y-2">
        <button
          onClick={handleClear}
          disabled={isClearing || isSeeding}
          className="w-full bg-red-600 text-white px-3 py-1 rounded text-sm hover:bg-red-700 disabled:opacity-50"
        >
          {isClearing ? 'Clearing...' : 'Clear Database'}
        </button>
        <button
          onClick={handleSeed}
          disabled={isSeeding || isClearing}
          className="w-full bg-black text-dark-warning px-3 py-1 rounded text-sm hover:opacity-80 disabled:opacity-50"
        >
          {isSeeding ? 'Seeding...' : 'Seed: 100 members, YTD payments, 25% debt'}
        </button>
      </div>
      {result && (
        <pre className="mt-2 text-xs bg-black text-dark-warning p-2 rounded whitespace-pre-wrap max-h-32 overflow-y-auto">
          {result}
        </pre>
      )}
      <p className="text-xs mt-2">After seeding/clearing, refresh the page</p>
    </div>
  );
};
