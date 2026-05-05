import { useState, useEffect, useCallback } from 'react';
import { useApp } from '../contexts/AppContext';
import { formatCurrency } from '../types';
import type { MemberDebtInfo } from '../types';

export const DashboardScreen = () => {
  const { members, getAllDebts } = useApp();
  const [debts, setDebts] = useState<MemberDebtInfo[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  const loadDebts = useCallback(async () => {
    setLoading(true);
    setError('');
    try {
      const data = await getAllDebts();
      setDebts(data);
    } catch (err) {
      console.error('Error loading debts:', err);
      setError(String(err));
    } finally {
      setLoading(false);
    }
  }, [getAllDebts]);

  useEffect(() => {
    loadDebts();
  }, [loadDebts]);

  const totalDebt = debts.reduce((sum, d) => sum + d.total_debt, 0);
  const activeMembers = members.filter(m => m.active).length;

  return (
    <div className="flex-1 p-8">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold text-dark-text-primary">Dashboard</h1>
        <button
          onClick={loadDebts}
          disabled={loading}
          className="bg-dark-accent text-white px-4 py-2 rounded hover:opacity-90 disabled:opacity-50"
        >
          {loading ? 'Carregando...' : 'Atualizar'}
        </button>
      </div>

      {error && (
        <div className="bg-dark-error/10 border border-dark-error text-dark-error p-4 rounded mb-6">
          {error}
        </div>
      )}

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* Total Debt Card */}
        <div className={`bg-dark-surface border rounded-lg p-6 ${totalDebt > 0 ? 'border-dark-error' : 'border-dark-border'}`}>
          <h2 className="text-dark-text-secondary mb-4">Dívida Total do Clube</h2>
          {loading ? (
            <p className="text-dark-text-secondary">Calculando...</p>
          ) : (
            <p className={`text-4xl font-bold ${totalDebt > 0 ? 'text-dark-error' : 'text-dark-text-primary'}`}>
              {formatCurrency(totalDebt)}
            </p>
          )}
        </div>

        {/* Active Members Card */}
        <div className="bg-dark-surface border border-dark-border rounded-lg p-6">
          <h2 className="text-dark-text-secondary mb-4">Membros Ativos</h2>
          <p className="text-4xl font-bold text-green-500">
            {activeMembers}
          </p>
        </div>
      </div>
    </div>
  );
};
