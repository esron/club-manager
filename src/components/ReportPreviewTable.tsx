import { DebtStatusReport, PaymentHistoryReport } from '../types/reports';
import { formatCurrency } from '../types';

const PREVIEW_ROW_LIMIT = 100;

interface ReportPreviewTableProps {
  debtReport?: DebtStatusReport;
  paymentReport?: PaymentHistoryReport;
}

export const ReportPreviewTable = ({ debtReport, paymentReport }: ReportPreviewTableProps) => {
  if (!debtReport && !paymentReport) {
    return (
      <div className="text-gray-400 text-center py-8">
        Configure o relatório acima e clique em visualizar
      </div>
    );
  }

  if (debtReport) {
    const displayRows = debtReport.members.slice(0, PREVIEW_ROW_LIMIT);
    const hasMore = debtReport.members.length > PREVIEW_ROW_LIMIT;

    // Calculate total debt
    const totalDebt = debtReport.members.reduce((sum, member) => sum + member.total_debt, 0);

    return (
      <div>
        {/* Summary Section */}
        <div className="mb-4 p-4 bg-dark-surface rounded border border-dark-border">
          <h4 className="text-sm font-semibold text-dark-text-secondary mb-2">Resumo</h4>
          <div className="text-lg font-bold text-dark-text-primary">
            Dívida Total: <span className="text-red-400">{formatCurrency(totalDebt)}</span>
          </div>
          <div className="text-sm text-dark-text-secondary mt-1">
            {debtReport.members.length} {debtReport.members.length === 1 ? 'membro' : 'membros'}
          </div>
        </div>

        {hasMore && (
          <div className="mb-2 text-sm text-yellow-500">
            Mostrando primeiras {PREVIEW_ROW_LIMIT} linhas de {debtReport.members.length}
          </div>
        )}
        <div className="overflow-x-auto">
          <table className="min-w-full bg-gray-800 rounded">
            <thead>
              <tr className="bg-gray-700">
                <th scope="col" className="px-4 py-2 text-left text-white">Nome do Membro</th>
                <th scope="col" className="px-4 py-2 text-left text-white">Dívida Total (R$)</th>
                <th scope="col" className="px-4 py-2 text-left text-white">Meses Não Pagos</th>
              </tr>
            </thead>
            <tbody>
              {displayRows.map((row) => (
                <tr key={row.member_id} className="border-t border-gray-700">
                  <td className="px-4 py-2 text-white">{row.member_name}</td>
                  <td className="px-4 py-2 text-white">{formatCurrency(row.total_debt)}</td>
                  <td className="px-4 py-2 text-white">{row.unpaid_month_count}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    );
  }

  if (paymentReport) {
    const displayRows = paymentReport.members.slice(0, PREVIEW_ROW_LIMIT);
    const hasMore = paymentReport.members.length > PREVIEW_ROW_LIMIT;

    // Calculate totals for each month
    const monthTotals = new Map<string, number>();
    paymentReport.month_columns.forEach((col) => {
      let total = 0;
      paymentReport.members.forEach((member) => {
        const paymentStr = member.payments[col.key] || '';
        // Extract numeric value from "R$ 15,00" format
        const match = paymentStr.match(/R\$\s*([\d.]+,\d{2})/);
        if (match) {
          const value = parseFloat(match[1].replace('.', '').replace(',', '.'));
          total += value;
        }
      });
      monthTotals.set(col.key, total);
    });

    // Calculate grand total
    const grandTotal = Array.from(monthTotals.values()).reduce((sum, val) => sum + val, 0);

    return (
      <div>
        {/* Summary Section */}
        <div className="mb-4 p-4 bg-dark-surface rounded border border-dark-border">
          <h4 className="text-sm font-semibold text-dark-text-secondary mb-2">Resumo - Total por Mês</h4>
          <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-3">
            {paymentReport.month_columns.map((col) => {
              const total = monthTotals.get(col.key) || 0;
              return (
                <div key={col.key} className="text-sm">
                  <div className="text-dark-text-secondary">{col.display}</div>
                  <div className="font-semibold text-dark-text-primary">
                    {total > 0 ? formatCurrency(total) : '-'}
                  </div>
                </div>
              );
            })}
          </div>
          <div className="mt-3 pt-3 border-t border-dark-border">
            <div className="text-lg font-bold text-dark-text-primary">
              Total Geral: <span className="text-green-400">{formatCurrency(grandTotal)}</span>
            </div>
          </div>
        </div>

        {hasMore && (
          <div className="mb-2 text-sm text-yellow-500">
            Mostrando primeiras {PREVIEW_ROW_LIMIT} linhas de {paymentReport.members.length}
          </div>
        )}
        <div className="overflow-x-auto">
          <table className="min-w-full bg-gray-800 rounded">
            <thead>
              <tr className="bg-gray-700">
                <th scope="col" className="px-4 py-2 text-left text-white sticky left-0 bg-gray-700">
                  Nome do Membro
                </th>
                <th scope="col" className="px-4 py-2 text-left text-white">Início</th>
                {paymentReport.month_columns.map((col) => (
                  <th key={col.key} scope="col" className="px-4 py-2 text-left text-white">
                    {col.display}
                  </th>
                ))}
              </tr>
            </thead>
            <tbody>
              {displayRows.map((row) => (
                <tr key={row.member_id} className="border-t border-gray-700">
                  <td className="px-4 py-2 text-white sticky left-0 bg-gray-800">
                    {row.member_name}
                  </td>
                  <td className="px-4 py-2 text-white">{row.start_date}</td>
                  {paymentReport.month_columns.map((col) => (
                    <td key={col.key} className="px-4 py-2 text-white">
                      {row.payments[col.key] || ''}
                    </td>
                  ))}
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    );
  }

  return null;
};
