// src/components/MemberDetailView.tsx
import { useState, useEffect } from 'react';
import { useApp } from '../contexts/AppContext';
import { formatCurrency, formatDate, MONTH_NAMES_PT } from '../types';
import type { Member, Payment, MemberDebtInfo } from '../types';

interface MemberDetailViewProps {
  memberId: number;
  onBack: () => void;
}

export const MemberDetailView = ({ memberId, onBack }: MemberDetailViewProps) => {
  const { members, payments, getMemberDebt, updateMemberActive, updateMemberName, deletePayment } = useApp();
  const [debtInfo, setDebtInfo] = useState<MemberDebtInfo | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const [editingName, setEditingName] = useState(false);
  const [newName, setNewName] = useState('');
  const [memberPaymentsPage, setMemberPaymentsPage] = useState(1);
  const [memberPaymentsPageSize, setMemberPaymentsPageSize] = useState(15);

  const member = members.find(m => m.id === memberId);

  useEffect(() => {
    if (member) {
      setNewName(member.name);
      loadDebt();
    }
  }, [memberId, member]);

  const loadDebt = async () => {
    setLoading(true);
    setError('');
    try {
      const data = await getMemberDebt(memberId);
      setDebtInfo(data);
    } catch (err) {
      console.error('Error loading debt:', err);
      setError('Erro ao calcular dívida');
    } finally {
      setLoading(false);
    }
  };

  const handleSaveName = async () => {
    try {
      await updateMemberName(memberId, newName);
      setEditingName(false);
      await loadDebt();
    } catch (err) {
      console.error('Error updating name:', err);
      alert(String(err));
    }
  };

  const handleDeactivate = async () => {
    if (confirm('Tem certeza que deseja desativar este membro?')) {
      await updateMemberActive(memberId, false);
      onBack();
    }
  };

  const handleDeletePayment = async (paymentId: number) => {
    if (confirm('Tem certeza que deseja excluir este pagamento?')) {
      await deletePayment(paymentId);
      await loadDebt();
    }
  };

  if (!member) {
    return (
      <div className="flex-1 p-8">
        <p className="text-dark-text-secondary">Membro não encontrado</p>
        <button onClick={onBack} className="mt-4 text-dark-accent">
          ← Voltar para Membros
        </button>
      </div>
    );
  }

  const memberPayments = payments
    .filter(p => p.member_id === memberId)
    .sort((a, b) => new Date(b.payment_date).getTime() - new Date(a.payment_date).getTime());

  const paginatedMemberPayments = memberPayments.slice(
    (memberPaymentsPage - 1) * memberPaymentsPageSize,
    memberPaymentsPage * memberPaymentsPageSize
  );
  const memberPaymentsTotalPages = Math.ceil(memberPayments.length / memberPaymentsPageSize);

  return (
    <div className="flex-1 p-8">
      {/* Back button */}
      <button onClick={onBack} className="mb-4 text-dark-accent hover:underline">
        ← Voltar para Membros
      </button>

      {/* Header */}
      <div className="bg-dark-surface border border-dark-border rounded-lg p-6 mb-6">
        {editingName ? (
          <div className="flex items-center gap-2">
            <input
              type="text"
              value={newName}
              onChange={(e) => setNewName(e.target.value)}
              className="bg-dark-bg border border-dark-border text-dark-text-primary rounded px-2 py-1 text-2xl font-bold"
            />
            <button onClick={handleSaveName} className="text-green-500 px-3 py-1">Salvar</button>
            <button onClick={() => { setEditingName(false); setNewName(member.name); }} className="text-dark-text-secondary px-3 py-1">Cancelar</button>
          </div>
        ) : (
          <h1 className="text-2xl font-bold text-dark-text-primary">{member.name}</h1>
        )}
        <p className="text-dark-text-secondary mt-2">Membro desde {formatDate(member.start_date)}</p>
        <div className="flex gap-2 mt-4">
          <button
            onClick={() => setEditingName(true)}
            className="bg-dark-accent text-white px-3 py-1 rounded text-sm hover:opacity-90"
          >
            Editar Nome
          </button>
          <button
            onClick={handleDeactivate}
            className="bg-dark-error text-white px-3 py-1 rounded text-sm hover:opacity-90"
          >
            Desativar
          </button>
        </div>
      </div>

      {/* Debt Summary Card */}
      {loading ? (
        <div className="bg-dark-surface border border-dark-border rounded-lg p-6 mb-6">
          <p className="text-dark-text-secondary">Calculando dívida...</p>
        </div>
      ) : error ? (
        <div className="bg-dark-error/10 border border-dark-error text-dark-error rounded-lg p-6 mb-6">
          {error}
        </div>
      ) : debtInfo ? (
        <>
          <div className={`border rounded-lg p-6 mb-6 ${debtInfo.total_debt > 0 ? 'bg-dark-error/10 border-dark-error' : 'bg-dark-surface border-dark-border'}`}>
            <h2 className="text-dark-text-secondary mb-2">Dívida Atual</h2>
            <p className={`text-3xl font-bold ${debtInfo.total_debt > 0 ? 'text-dark-error' : 'text-green-500'}`}>
              {formatCurrency(debtInfo.total_debt)}
            </p>
            <p className="text-dark-text-secondary mt-2">
              Meses em atraso: {debtInfo.unpaid_months.length}
            </p>
          </div>

          {/* Unpaid Months Section */}
          {debtInfo.unpaid_months.length > 0 && (
            <div className="bg-dark-surface border border-dark-border rounded-lg p-6 mb-6">
              <h2 className="text-xl font-bold text-dark-text-primary mb-4">Meses Não Pagos</h2>
              <table className="w-full">
                <thead>
                  <tr className="border-b border-dark-border">
                    <th className="text-left py-2 text-dark-text-secondary">Mês/Ano</th>
                    <th className="text-left py-2 text-dark-text-secondary">Valor</th>
                    <th className="text-left py-2 text-dark-text-secondary">Ação</th>
                  </tr>
                </thead>
                <tbody>
                  {debtInfo.unpaid_months.map((um, idx) => (
                    <tr key={idx} className="border-b border-dark-border">
                      <td className="py-2 text-dark-text-primary">{um.display}</td>
                      <td className="py-2 text-dark-text-primary">{formatCurrency(um.amount)}</td>
                      <td className="py-2">
                        <button className="text-dark-accent text-sm hover:underline">
                          + Adicionar Pagamento
                        </button>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </>
      ) : null}

      {/* Payment History Section */}
      <div className="bg-dark-surface border border-dark-border rounded-lg p-6">
        <h2 className="text-xl font-bold text-dark-text-primary mb-4">Histórico de Pagamentos</h2>
        {memberPayments.length === 0 ? (
          <p className="text-dark-text-secondary">Nenhum pagamento registrado</p>
        ) : (
          <>
            <table className="w-full">
              <thead>
                <tr className="border-b border-dark-border">
                  <th className="text-left py-2 text-dark-text-secondary">Data Pago</th>
                  <th className="text-left py-2 text-dark-text-secondary">Ref. Mês/Ano</th>
                  <th className="text-left py-2 text-dark-text-secondary">Valor</th>
                  <th className="text-left py-2 text-dark-text-secondary">Ação</th>
                </tr>
              </thead>
              <tbody>
                {paginatedMemberPayments.map((payment) => (
                  <tr key={payment.id} className="border-b border-dark-border">
                    <td className="py-2 text-dark-text-primary">{formatDate(payment.payment_date)}</td>
                    <td className="py-2 text-dark-text-primary">
                      {MONTH_NAMES_PT[payment.month - 1]} {payment.year}
                    </td>
                    <td className="py-2 text-dark-text-primary">{formatCurrency(payment.amount_brl)}</td>
                    <td className="py-2">
                      <button
                        onClick={() => handleDeletePayment(payment.id)}
                        className="text-dark-error text-sm hover:underline"
                      >
                        Excluir
                      </button>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>

            {/* Pagination */}
            {memberPaymentsTotalPages > 1 && (
              <div className="flex items-center justify-between mt-4">
                <div className="flex items-center gap-2">
                  <button
                    onClick={() => setMemberPaymentsPage(p => Math.max(1, p - 1))}
                    disabled={memberPaymentsPage === 1}
                    className="px-3 py-1 bg-dark-bg border border-dark-border rounded disabled:opacity-50"
                  >
                    Anterior
                  </button>
                  <span className="text-dark-text-secondary">
                    Página {memberPaymentsPage} de {memberPaymentsTotalPages}
                  </span>
                  <button
                    onClick={() => setMemberPaymentsPage(p => Math.min(memberPaymentsTotalPages, p + 1))}
                    disabled={memberPaymentsPage === memberPaymentsTotalPages}
                    className="px-3 py-1 bg-dark-bg border border-dark-border rounded disabled:opacity-50"
                  >
                    Próxima
                  </button>
                </div>
                <div className="flex items-center gap-2">
                  <span className="text-dark-text-secondary text-sm">Itens por página:</span>
                  <select
                    value={memberPaymentsPageSize}
                    onChange={(e) => {
                      setMemberPaymentsPageSize(Number(e.target.value));
                      setMemberPaymentsPage(1);
                    }}
                    className="bg-dark-bg border border-dark-border text-dark-text-primary rounded px-2 py-1 text-sm"
                  >
                    <option value="15">15</option>
                    <option value="30">30</option>
                    <option value="100">100</option>
                  </select>
                </div>
              </div>
            )}
          </>
        )}
      </div>
    </div>
  );
};
