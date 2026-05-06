import { useState } from 'react';
import { ReportType, ExportFormat } from '../types/reports';

export const ReportsScreen = () => {
  const [reportType, setReportType] = useState<ReportType>('debt_status');
  const [format, setFormat] = useState<ExportFormat>('csv');
  const [anonymize, setAnonymize] = useState(false);
  const [includeInactive, setIncludeInactive] = useState(false);
  const [startDate, setStartDate] = useState('');
  const [endDate, setEndDate] = useState('');
  const [dateError, setDateError] = useState('');

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

  const handleExport = () => {
    if (!validateDates()) return;
    // TODO: Will implement export logic in later tasks
    console.log('Export requested');
  };

  return (
    <div className="p-6">
      <h1 className="text-2xl font-bold mb-6 text-white">Relatórios</h1>

      {/* Export Controls */}
      <div className="bg-gray-800 rounded-lg p-6 mb-6">
        <h2 className="text-lg font-semibold mb-4 text-white">Configuração do Relatório</h2>

        {/* Report Type Selection */}
        <div className="mb-4">
          <label className="block text-sm font-medium mb-2 text-gray-300">
            Tipo de Relatório
          </label>
          <div className="space-y-2">
            <label className="flex items-center text-white">
              <input
                type="radio"
                value="debt_status"
                checked={reportType === 'debt_status'}
                onChange={(e) => setReportType(e.target.value as ReportType)}
                className="mr-2"
              />
              Status de Dívidas
            </label>
            <label className="flex items-center text-white">
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
            <label className="flex items-center text-white">
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
              <label className="block text-sm font-medium mb-2 text-gray-300">
                Data Inicial
              </label>
              <input
                type="date"
                value={startDate}
                onChange={(e) => setStartDate(e.target.value)}
                className="px-3 py-2 bg-gray-700 border border-gray-600 rounded text-white"
              />
            </div>
            <div>
              <label className="block text-sm font-medium mb-2 text-gray-300">
                Data Final
              </label>
              <input
                type="date"
                value={endDate}
                onChange={(e) => setEndDate(e.target.value)}
                className="px-3 py-2 bg-gray-700 border border-gray-600 rounded text-white"
              />
            </div>
            {dateError && (
              <p className="text-red-500 text-sm">{dateError}</p>
            )}
          </div>
        )}

        {/* Format Selection */}
        <div className="mb-4">
          <label className="block text-sm font-medium mb-2 text-gray-300">
            Formato
          </label>
          <div className="space-y-2">
            <label className="flex items-center text-white">
              <input
                type="radio"
                value="csv"
                checked={format === 'csv'}
                onChange={(e) => setFormat(e.target.value as ExportFormat)}
                className="mr-2"
              />
              CSV
            </label>
            <label className="flex items-center text-white">
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
          <label className="flex items-center text-white">
            <input
              type="checkbox"
              checked={anonymize}
              onChange={(e) => setAnonymize(e.target.checked)}
              className="mr-2"
            />
            Relatório anônimo (Membro #1, Membro #2, ...)
          </label>
        </div>

        {/* Export Button */}
        <button
          onClick={handleExport}
          className="px-6 py-2 bg-blue-600 text-white rounded hover:bg-blue-500"
        >
          Exportar
        </button>
      </div>

      {/* Preview Area - Placeholder */}
      <div className="bg-gray-800 rounded-lg p-6">
        <h2 className="text-lg font-semibold mb-4 text-white">Visualização</h2>
        <div className="text-gray-400 text-center py-8">
          Configuração pronta. A visualização será implementada nas próximas etapas.
        </div>
      </div>
    </div>
  );
};
