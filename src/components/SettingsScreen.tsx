import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useApp } from '../contexts/AppContext';

export const SettingsScreen = () => {
  const { settings, updateSetting } = useApp();
  const [minimumFee, setMinimumFee] = useState('');
  const [error, setError] = useState('');
  const [success, setSuccess] = useState('');
  const [loading, setLoading] = useState(false);

  // Password change state
  const [currentPassword, setCurrentPassword] = useState('');
  const [newPassword, setNewPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [passwordError, setPasswordError] = useState('');
  const [passwordSuccess, setPasswordSuccess] = useState('');
  const [passwordLoading, setPasswordLoading] = useState(false);

  useEffect(() => {
    setMinimumFee(settings.minimumFee);
  }, [settings]);

  const handleSave = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    setSuccess('');
    setLoading(true);

    try {
      await updateSetting('minimum_fee_brl', minimumFee);
      setSuccess('Configurações salvas com sucesso');
      setTimeout(() => setSuccess(''), 3000);
    } catch (err) {
      console.error('Error saving settings:', err);
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  const handlePasswordChange = async (e: React.FormEvent) => {
    e.preventDefault();
    setPasswordError('');
    setPasswordSuccess('');

    // Validation
    if (newPassword.length < 8) {
      setPasswordError('A nova senha deve ter no mínimo 8 caracteres');
      return;
    }

    if (newPassword !== confirmPassword) {
      setPasswordError('As senhas não coincidem');
      return;
    }

    if (newPassword === currentPassword) {
      setPasswordError('A nova senha deve ser diferente da senha atual');
      return;
    }

    setPasswordLoading(true);

    try {
      await invoke('change_password', {
        currentPassword,
        newPassword,
      });

      setPasswordSuccess('Senha alterada com sucesso');
      setCurrentPassword('');
      setNewPassword('');
      setConfirmPassword('');
      setTimeout(() => setPasswordSuccess(''), 3000);
    } catch (err) {
      console.error('Error changing password:', err);
      setPasswordError(String(err));
    } finally {
      setPasswordLoading(false);
    }
  };

  return (
    <div className="flex-1 p-8">
      <h1 className="text-2xl font-bold mb-6 text-dark-text-primary">Configurações</h1>

      {/* Minimum Fee Settings */}
      <div className="bg-dark-surface p-6 rounded-lg border border-dark-border max-w-2xl mb-6">
        <h2 className="text-lg font-semibold mb-4 text-dark-text-primary">Mensalidade</h2>
        <form onSubmit={handleSave}>
          <div className="mb-6">
            <label className="block mb-2 text-dark-text-secondary">
              Mensalidade Mínima (R$)
            </label>
            <input
              type="text"
              value={minimumFee}
              onChange={(e) => setMinimumFee(e.target.value)}
              className="w-full bg-dark-bg border border-dark-border text-dark-text-primary rounded px-3 py-2"
              placeholder="15.00"
              required
            />
            {error && <p className="text-dark-error text-sm mt-2">{error}</p>}
            {success && <p className="text-green-500 text-sm mt-2">{success}</p>}
          </div>

          <button
            type="submit"
            disabled={loading}
            className="bg-dark-accent text-white px-6 py-2 rounded hover:opacity-90 disabled:opacity-50"
          >
            {loading ? 'Salvando...' : 'Salvar'}
          </button>
        </form>
      </div>

      {/* Password Change */}
      <div className="bg-dark-surface p-6 rounded-lg border border-dark-border max-w-2xl">
        <h2 className="text-lg font-semibold mb-4 text-dark-text-primary">Alterar Senha</h2>
        <form onSubmit={handlePasswordChange}>
          <div className="mb-4">
            <label className="block mb-2 text-dark-text-secondary">
              Senha Atual
            </label>
            <input
              type="password"
              value={currentPassword}
              onChange={(e) => setCurrentPassword(e.target.value)}
              className="w-full bg-dark-bg border border-dark-border text-dark-text-primary rounded px-3 py-2"
              required
            />
          </div>

          <div className="mb-4">
            <label className="block mb-2 text-dark-text-secondary">
              Nova Senha (mínimo 8 caracteres)
            </label>
            <input
              type="password"
              value={newPassword}
              onChange={(e) => setNewPassword(e.target.value)}
              className="w-full bg-dark-bg border border-dark-border text-dark-text-primary rounded px-3 py-2"
              minLength={8}
              required
            />
          </div>

          <div className="mb-6">
            <label className="block mb-2 text-dark-text-secondary">
              Confirmar Nova Senha
            </label>
            <input
              type="password"
              value={confirmPassword}
              onChange={(e) => setConfirmPassword(e.target.value)}
              className="w-full bg-dark-bg border border-dark-border text-dark-text-primary rounded px-3 py-2"
              minLength={8}
              required
            />
          </div>

          {passwordError && <p className="text-dark-error text-sm mb-4">{passwordError}</p>}
          {passwordSuccess && <p className="text-green-500 text-sm mb-4">{passwordSuccess}</p>}

          <button
            type="submit"
            disabled={passwordLoading}
            className="bg-dark-accent text-white px-6 py-2 rounded hover:opacity-90 disabled:opacity-50"
          >
            {passwordLoading ? 'Alterando...' : 'Alterar Senha'}
          </button>
        </form>
      </div>
    </div>
  );
};
