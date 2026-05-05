// src/components/SettingsScreen.tsx
import { useState, useEffect } from 'react';
import { useApp } from '../contexts/AppContext';

export const SettingsScreen = () => {
  const { settings, updateSetting } = useApp();
  const [minimumFee, setMinimumFee] = useState('');
  const [error, setError] = useState('');
  const [success, setSuccess] = useState('');
  const [loading, setLoading] = useState(false);

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

  return (
    <div className="flex-1 p-8">
      <h1 className="text-2xl font-bold mb-6 text-dark-text-primary">Configurações</h1>

      <div className="bg-dark-surface p-6 rounded-lg border border-dark-border max-w-2xl">
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
    </div>
  );
};
