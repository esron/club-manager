import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useAuth } from '../contexts/AuthContext';

export const DevTools = () => {
  const { password } = useAuth();
  const [isSeeding, setIsSeeding] = useState(false);
  const [result, setResult] = useState('');

  const handleSeed = async () => {
    if (!password) {
      setResult('Error: Not authenticated');
      return;
    }

    setIsSeeding(true);
    setResult('');

    try {
      const res = await invoke<string>('seed_database', {
        password,
        memberCount: 100,
        paymentsPerMember: 5
      });
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
    <div className="fixed bottom-4 right-4 bg-dark-warning text-black p-4 rounded shadow-lg border border-black">
      <h3 className="font-bold mb-2">🛠️ Dev Tools</h3>
      <button
        onClick={handleSeed}
        disabled={isSeeding}
        className="bg-black text-dark-warning px-3 py-1 rounded text-sm hover:opacity-80 disabled:opacity-50"
      >
        {isSeeding ? 'Seeding...' : 'Seed Database (100 members)'}
      </button>
      {result && (
        <pre className="mt-2 text-xs bg-black text-dark-warning p-2 rounded whitespace-pre-wrap">
          {result}
        </pre>
      )}
      <p className="text-xs mt-2">After seeding, refresh the page</p>
    </div>
  );
};
