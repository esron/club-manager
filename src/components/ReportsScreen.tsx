import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { DebtStatusReport, PaymentHistoryReport, ReportType, ExportFormat } from '../types/reports';
import { ReportPreviewTable } from './ReportPreviewTable';
import { useApp } from '../contexts/AppContext';

export const ReportsScreen = () => {
  const { requestReAuth } = useApp();
  const [reportType, setReportType] = useState<ReportType>('debt_status');
  const [format, setFormat] = useState<ExportFormat>('csv');
  const [anonymize, setAnonymize] = useState(false);
  const [includeInactive, setIncludeInactive] = useState(false);
  const [startDate, setStartDate] = useState('');
  const [endDate, setEndDate] = useState('');
  const [dateError, setDateError] = useState('');
  const [debtReport, setDebtReport] = useState<DebtStatusReport | undefined>(undefined);
  const [paymentReport, setPaymentReport] = useState<PaymentHistoryReport | undefined>(undefined);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  const validateDates = (): boolean => {
    if (reportType === 'payment_history') {
      if (!startDate || !endDate) {
        setDateError('Data inicial e final são obrigatórias');
        return false;
      }
      if (new Date(endDate) < new Date(startDate)) {
        setDateError('A data final deve ser posterior à data inicial');
        return false;
      }
    }
    setDateError('');
    return true;
  };

  const loadDebtStatusReport = async (password: string) => {
    setLoading(true);
    setError('');
    try {
      const report = await invoke<DebtStatusReport>('get_debt_status_report_cmd', {
        password,
        includeInactive,
      });
      setDebtReport(report);
      setPaymentReport(undefined);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  const handleExport = () => {
    if (!validateDates()) return;
    // TODO: Will implement export logic in later tasks
    console.log('Export requested');
  };

  return (
    <div className="flex-1 p-8">
      <h1 className="text-2xl font-bold mb-6 text-dark-text-primary">Relatórios</h1>

      {/* Export Controls */}
      <div className="bg-dark-surface rounded-lg p-6 mb-6">
        <h2 className="text-lg font-semibold mb-4 text-dark-text-primary">Configuração do Relatório</h2>

        {/* Report Type Selection */}
        <div className="mb-4">
          <label className="block text-sm font-medium mb-2 text-dark-text-secondary">
            Tipo de Relatório
          </label>
          <div className="space-y-2">
            <label className="flex items-center text-dark-text-primary">
              <input
                type="radio"
                value="debt_status"
                checked={reportType === 'debt_status'}
                onChange={(e) => setReportType(e.target.value as ReportType)}
                className="mr-2"
              />
              Status de Dívidas
            </label>
            <label className="flex items-center text-dark-text-primary">
              <input
                type="radio"
                value="payment_history"
                checked={reportType === 'payment_history'}
                onChange={(e) => setReportType(e.target.value as ReportType)}
                className="mr-2"
              />
              Histórico de Pagamentos
            </label>
          </div>
        </div>

        {/* Conditional Configuration */}
        {reportType === 'debt_status' && (
          <div className="mb-4">
            <label className="flex items-center text-dark-text-primary">
              <input
                type="checkbox"
                checked={includeInactive}
                onChange={(e) => setIncludeInactive(e.target.checked)}
                className="mr-2"
              />
              Incluir membros inativos
            </label>
          </div>
        )}

        {reportType === 'payment_history' && (
          <div className="mb-4 space-y-3">
            <div>
              <label htmlFor="startDate" className="block text-sm font-medium mb-2 text-dark-text-secondary">
                Data Inicial
              </label>
              <input
                id="startDate"
                type="date"
                value={startDate}
                onChange={(e) => {
                  setDateError('');
                  setStartDate(e.target.value);
                }}
                className="px-3 py-2 bg-dark-bg border border-dark-border rounded text-dark-text-primary"
              />
            </div>
            <div>
              <label htmlFor="endDate" className="block text-sm font-medium mb-2 text-dark-text-secondary">
                Data Final
              </label>
              <input
                id="endDate"
                type="date"
                value={endDate}
                onChange={(e) => {
                  setDateError('');
                  setEndDate(e.target.value);
                }}
                className="px-3 py-2 bg-dark-bg border border-dark-border rounded text-dark-text-primary"
              />
            </div>
            {dateError && (
              <p className="text-red-500 text-sm">{dateError}</p>
            )}
          </div>
        )}

        {/* Format Selection */}
        <div className="mb-4">
          <label className="block text-sm font-medium mb-2 text-dark-text-secondary">
            Formato
          </label>
          <div className="space-y-2">
            <label className="flex items-center text-dark-text-primary">
              <input
                type="radio"
                value="csv"
                checked={format === 'csv'}
                onChange={(e) => setFormat(e.target.value as ExportFormat)}
                className="mr-2"
              />
              CSV
            </label>
            <label className="flex items-center text-dark-text-primary">
              <input
                type="radio"
                value="xlsx"
                checked={format === 'xlsx'}
                onChange={(e) => setFormat(e.target.value as ExportFormat)}
                className="mr-2"
              />
              XLSX
            </label>
          </div>
        </div>

        {/* Anonymize Checkbox */}
        <div className="mb-4">
          <label className="flex items-center text-dark-text-primary">
            <input
              type="checkbox"
              checked={anonymize}
              onChange={(e) => setAnonymize(e.target.checked)}
              className="mr-2"
            />
            Relatório anônimo (Membro #1, Membro #2, ...)
          </label>
        </div>

        {/* Action Buttons */}
        <div className="flex gap-3">
          <button
            onClick={() => {
              if (reportType === 'debt_status') {
                requestReAuth((password) => loadDebtStatusReport(password));
              }
            }}
            disabled={reportType === 'payment_history' && !validateDates()}
            className="px-4 py-2 bg-dark-accent text-dark-text-primary rounded hover:bg-opacity-80 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            Visualizar
          </button>
          <button
            onClick={handleExport}
            className="px-6 py-2 bg-dark-accent text-dark-text-primary rounded hover:bg-dark-accent/90 disabled:opacity-50"
          >
            Exportar
          </button>
        </div>
      </div>

      {/* Preview Area */}
      <div className="bg-dark-surface rounded-lg p-6">
        <h2 className="text-lg font-semibold mb-4 text-dark-text-primary">Visualização</h2>
        {loading && <div className="text-dark-text-primary">Carregando...</div>}
        {error && <div className="text-red-500">{error}</div>}
        {(debtReport || paymentReport) && (
          <div className="mt-8">
            <h3 className="text-xl font-semibold text-dark-text-primary mb-4">
              Prévia do Relatório
            </h3>
            <ReportPreviewTable
              debtReport={debtReport}
              paymentReport={paymentReport}
            />
          </div>
        )}
        {!loading && !error && !debtReport && !paymentReport && (
          <div className="text-dark-text-secondary text-center py-8">
            Configuração pronta. Clique em "Visualizar" para ver a prévia do relatório.
          </div>
        )}
      </div>
    </div>
  );
};
