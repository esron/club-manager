import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useAuth } from '../contexts/AuthContext';
import type { ChartData } from '../types';
import { formatCurrency } from '../types';
import {
  BarChart,
  Bar,
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
} from 'recharts';

export const DashboardCharts = () => {
  const { password } = useAuth();
  const [chartData, setChartData] = useState<ChartData | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  useEffect(() => {
    const loadChartData = async () => {
      if (!password) return;

      setLoading(true);
      setError('');
      try {
        const data = await invoke<ChartData>('get_dashboard_chart_data_cmd', { password });
        setChartData(data);
      } catch (err) {
        console.error('Error loading chart data:', err);
        setError(String(err));
      } finally {
        setLoading(false);
      }
    };

    loadChartData();
  }, [password]);

  if (loading) {
    return (
      <div className="bg-dark-surface border border-dark-border rounded-lg p-6">
        <p className="text-dark-text-secondary">Carregando gráficos...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="bg-dark-error/10 border border-dark-error text-dark-error p-4 rounded">
        Erro ao carregar gráficos: {error}
      </div>
    );
  }

  if (!chartData || chartData.months.length === 0) {
    return null;
  }

  return (
    <div className="space-y-6">
      {/* Payment Trend Chart */}
      <div className="bg-dark-surface border border-dark-border rounded-lg p-6">
        <h2 className="text-lg font-semibold text-dark-text-primary mb-4">
          Pagamentos Mensais (últimos 6 meses)
        </h2>
        <ResponsiveContainer width="100%" height={300}>
          <BarChart data={chartData.months}>
            <CartesianGrid strokeDasharray="3 3" stroke="#374151" />
            <XAxis
              dataKey="month_display"
              stroke="#9CA3AF"
              style={{ fontSize: '14px' }}
            />
            <YAxis
              stroke="#9CA3AF"
              style={{ fontSize: '14px' }}
              tickFormatter={(value) => `R$ ${value}`}
            />
            <Tooltip
              contentStyle={{
                backgroundColor: '#1F2937',
                border: '1px solid #374151',
                borderRadius: '0.5rem',
                color: '#F3F4F6',
              }}
              formatter={(value: number) => [formatCurrency(value), 'Total']}
            />
            <Bar dataKey="total_payments" fill="#10B981" radius={[4, 4, 0, 0]} />
          </BarChart>
        </ResponsiveContainer>
      </div>

      {/* Debt Evolution Chart */}
      <div className="bg-dark-surface border border-dark-border rounded-lg p-6">
        <h2 className="text-lg font-semibold text-dark-text-primary mb-4">
          Evolução da Dívida Total (últimos 6 meses)
        </h2>
        <ResponsiveContainer width="100%" height={300}>
          <LineChart data={chartData.months}>
            <CartesianGrid strokeDasharray="3 3" stroke="#374151" />
            <XAxis
              dataKey="month_display"
              stroke="#9CA3AF"
              style={{ fontSize: '14px' }}
            />
            <YAxis
              stroke="#9CA3AF"
              style={{ fontSize: '14px' }}
              tickFormatter={(value) => `R$ ${value}`}
            />
            <Tooltip
              contentStyle={{
                backgroundColor: '#1F2937',
                border: '1px solid #374151',
                borderRadius: '0.5rem',
                color: '#F3F4F6',
              }}
              formatter={(value: number) => [formatCurrency(value), 'Dívida Total']}
            />
            <Line
              type="monotone"
              dataKey="total_debt"
              stroke="#EF4444"
              strokeWidth={2}
              dot={{ fill: '#EF4444', r: 4 }}
              activeDot={{ r: 6 }}
            />
          </LineChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
};
