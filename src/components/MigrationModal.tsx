import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface MigrationModalProps {
  onComplete: () => void;
}

export const MigrationModal = ({ onComplete }: MigrationModalProps) => {
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);

  const handleMigrate = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    setLoading(true);

    try {
      await invoke('migrate_to_master_key', { password });
      onComplete();
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-gray-800 rounded-lg p-6 w-96 shadow-xl">
        <h2 className="text-xl font-bold mb-4 text-white">Atualização Necessária</h2>
        <p className="text-gray-300 mb-4">
          O aplicativo foi atualizado. Digite sua senha para continuar.
        </p>

        <form onSubmit={handleMigrate}>
          <div className="mb-4">
            <label className="block text-sm font-medium mb-2 text-gray-300">
              Senha
            </label>
            <input
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              className="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
              autoFocus
              disabled={loading}
              required
            />
            {error && (
              <p className="text-red-500 text-sm mt-1">{error}</p>
            )}
          </div>

          <button
            type="submit"
            className="w-full px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-500 disabled:opacity-50"
            disabled={loading}
          >
            {loading ? 'Atualizando...' : 'Continuar'}
          </button>
        </form>
      </div>
    </div>
  );
};
