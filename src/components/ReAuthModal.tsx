import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useApp } from '../contexts/AppContext';

export const ReAuthModal = () => {
  const { showReAuthModal, reAuthCallback, closeReAuthModal } = useApp();
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);

  // Reset state when modal opens
  useEffect(() => {
    if (showReAuthModal) {
      setPassword('');
      setError('');
      setLoading(false);
    }
  }, [showReAuthModal]);

  // Handle Escape key to close modal
  useEffect(() => {
    if (!showReAuthModal) return;

    const handleEscapeKey = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        handleCancel();
      }
    };

    document.addEventListener('keydown', handleEscapeKey);
    return () => {
      document.removeEventListener('keydown', handleEscapeKey);
    };
  }, [showReAuthModal]);

  if (!showReAuthModal) return null;

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    setLoading(true);

    try {
      // Verify password using existing auth command
      await invoke('verify_password_cmd', { password });

      // Password correct - execute callback
      const passwordCopy = password;
      closeReAuthModal();
      setPassword('');
      if (reAuthCallback) {
        reAuthCallback(passwordCopy);
      }
    } catch (err) {
      // Distinguish between password failure and system errors
      const errorMsg = err instanceof Error ? err.message : String(err);
      if (errorMsg.includes('senha') || errorMsg.includes('password') || errorMsg.includes('unauthorized')) {
        setError('Senha incorreta. Tente novamente.');
      } else {
        setError('Erro ao verificar senha. Tente novamente.');
      }
    } finally {
      setLoading(false);
    }
  };

  const handleCancel = () => {
    setPassword('');
    setError('');
    closeReAuthModal();
  };

  const handleBackdropClick = (e: React.MouseEvent) => {
    // Only close if clicking the backdrop itself, not the modal content
    if (e.target === e.currentTarget) {
      handleCancel();
    }
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50" onClick={handleBackdropClick}>
      <div className="bg-gray-800 rounded-lg p-6 w-96 shadow-xl">
        <h2 className="text-xl font-bold mb-4 text-white">Confirmar Senha</h2>
        <p className="text-gray-300 mb-4">
          Digite sua senha para continuar com a exportação.
        </p>

        <form onSubmit={handleSubmit}>
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
            />
            {error && (
              <p className="text-red-500 text-sm mt-1">{error}</p>
            )}
          </div>

          <div className="flex gap-2">
            <button
              type="button"
              onClick={handleCancel}
              className="flex-1 px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-500 disabled:opacity-50"
              disabled={loading}
            >
              Cancelar
            </button>
            <button
              type="submit"
              className="flex-1 px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-500 disabled:opacity-50"
              disabled={loading}
            >
              {loading ? 'Verificando...' : 'Confirmar'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};
