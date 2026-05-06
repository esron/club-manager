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

    return (
      <div>
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

    return (
      <div>
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
