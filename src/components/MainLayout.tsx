import { useEffect, useState } from 'react';
import { useApp } from '../contexts/AppContext';
import { formatCurrency, formatDate } from '../types';

export const MainLayout = () => {
  const { members, payments, refreshMembers, refreshPayments, addMember, addPayment } = useApp();
  const [activeTab, setActiveTab] = useState<'members' | 'payments'>('members');
  const [showAddMember, setShowAddMember] = useState(false);
  const [showAddPayment, setShowAddPayment] = useState(false);
  const [memberName, setMemberName] = useState('');
  const [memberStartDate, setMemberStartDate] = useState('');
  const [selectedMemberId, setSelectedMemberId] = useState<number>(0);
  const [paymentMonth, setPaymentMonth] = useState(new Date().getMonth() + 1);
  const [paymentYear, setPaymentYear] = useState(new Date().getFullYear());
  const [paymentAmount, setPaymentAmount] = useState('15.00');
  const [paymentDate, setPaymentDate] = useState(new Date().toISOString().split('T')[0]);

  useEffect(() => {
    refreshMembers();
    refreshPayments();
  }, []);

  const handleAddMember = async (e: React.FormEvent) => {
    e.preventDefault();
    await addMember(memberName, memberStartDate);
    setMemberName('');
    setMemberStartDate('');
    setShowAddMember(false);
  };

  const handleAddPayment = async (e: React.FormEvent) => {
    e.preventDefault();
    await addPayment(selectedMemberId, paymentMonth, paymentYear, parseFloat(paymentAmount), paymentDate);
    setShowAddPayment(false);
  };

  return (
    <div className="min-h-screen bg-dark-bg flex">
      <div className="w-64 bg-dark-surface border-r border-dark-border p-4">
        <h1 className="text-xl font-bold mb-8 text-dark-text-primary">Gestor do Clube</h1>
        <nav>
          <button
            onClick={() => setActiveTab('members')}
            className={`w-full text-left px-4 py-2 rounded mb-2 ${
              activeTab === 'members' ? 'bg-dark-accent text-white' : 'text-dark-text-primary hover:bg-dark-bg'
            }`}
          >
            Membros
          </button>
          <button
            onClick={() => setActiveTab('payments')}
            className={`w-full text-left px-4 py-2 rounded ${
              activeTab === 'payments' ? 'bg-dark-accent text-white' : 'text-dark-text-primary hover:bg-dark-bg'
            }`}
          >
            Pagamentos
          </button>
        </nav>
      </div>

      <div className="flex-1 p-8">
        {activeTab === 'members' && (
          <div>
            <div className="flex justify-between items-center mb-6">
              <h2 className="text-2xl font-bold text-dark-text-primary">Membros ({members.length})</h2>
              <button
                onClick={() => setShowAddMember(!showAddMember)}
                className="bg-dark-accent text-white px-4 py-2 rounded hover:opacity-90"
              >
                + Novo Membro
              </button>
            </div>

            {showAddMember && (
              <form onSubmit={handleAddMember} className="bg-dark-surface p-4 rounded border border-dark-border mb-6">
                <div className="grid grid-cols-2 gap-4 mb-4">
                  <div>
                    <label className="block mb-2 text-dark-text-secondary">Nome</label>
                    <input
                      type="text"
                      value={memberName}
                      onChange={(e) => setMemberName(e.target.value)}
                      className="w-full bg-dark-bg border border-dark-border text-dark-text-primary rounded px-3 py-2"
                      required
                    />
                  </div>
                  <div>
                    <label className="block mb-2 text-dark-text-secondary">Data de Início</label>
                    <input
                      type="date"
                      value={memberStartDate}
                      onChange={(e) => setMemberStartDate(e.target.value)}
                      className="w-full bg-dark-bg border border-dark-border text-dark-text-primary rounded px-3 py-2"
                      required
                    />
                  </div>
                </div>
                <button type="submit" className="bg-dark-accent text-white px-4 py-2 rounded hover:opacity-90">
                  Salvar
                </button>
              </form>
            )}

            <div className="bg-dark-surface rounded border border-dark-border">
              <table className="w-full">
                <thead className="border-b border-dark-border">
                  <tr>
                    <th className="text-left p-4 text-dark-text-secondary">Nome</th>
                    <th className="text-left p-4 text-dark-text-secondary">Data de Início</th>
                    <th className="text-left p-4 text-dark-text-secondary">Status</th>
                  </tr>
                </thead>
                <tbody>
                  {members.map((member) => (
                    <tr key={member.id} className="border-b border-dark-border last:border-0">
                      <td className="p-4 text-dark-text-primary">{member.name}</td>
                      <td className="p-4 text-dark-text-secondary">{formatDate(member.start_date)}</td>
                      <td className="p-4">
                        <span className="text-dark-success">Ativo</span>
                      </td>
                    </tr>
                  ))}
                  {members.length === 0 && (
                    <tr>
                      <td colSpan={3} className="p-8 text-center text-dark-text-secondary">
                        Nenhum membro cadastrado
                      </td>
                    </tr>
                  )}
                </tbody>
              </table>
            </div>
          </div>
        )}

        {activeTab === 'payments' && (
          <div>
            <div className="flex justify-between items-center mb-6">
              <h2 className="text-2xl font-bold text-dark-text-primary">Pagamentos ({payments.length})</h2>
              <button
                onClick={() => setShowAddPayment(!showAddPayment)}
                className="bg-dark-accent text-white px-4 py-2 rounded hover:opacity-90"
                disabled={members.length === 0}
              >
                + Novo Pagamento
              </button>
            </div>

            {showAddPayment && (
              <form onSubmit={handleAddPayment} className="bg-dark-surface p-4 rounded border border-dark-border mb-6">
                <div className="grid grid-cols-2 gap-4 mb-4">
                  <div>
                    <label className="block mb-2 text-dark-text-secondary">Membro</label>
                    <select
                      value={selectedMemberId}
                      onChange={(e) => setSelectedMemberId(Number(e.target.value))}
                      className="w-full bg-dark-bg border border-dark-border text-dark-text-primary rounded px-3 py-2"
                      required
                    >
                      <option value={0}>Selecione...</option>
                      {members.map((m) => (
                        <option key={m.id} value={m.id}>
                          {m.name}
                        </option>
                      ))}
                    </select>
                  </div>
                  <div>
                    <label className="block mb-2 text-dark-text-secondary">Mês/Ano</label>
                    <div className="flex gap-2">
                      <input
                        type="number"
                        min={1}
                        max={12}
                        value={paymentMonth}
                        onChange={(e) => setPaymentMonth(Number(e.target.value))}
                        className="w-24 bg-dark-bg border border-dark-border text-dark-text-primary rounded px-3 py-2"
                        placeholder="Mês"
                        required
                      />
                      <input
                        type="number"
                        value={paymentYear}
                        onChange={(e) => setPaymentYear(Number(e.target.value))}
                        className="w-32 bg-dark-bg border border-dark-border text-dark-text-primary rounded px-3 py-2"
                        placeholder="Ano"
                        required
                      />
                    </div>
                  </div>
                  <div>
                    <label className="block mb-2 text-dark-text-secondary">Valor (R$)</label>
                    <input
                      type="number"
                      step="0.01"
                      value={paymentAmount}
                      onChange={(e) => setPaymentAmount(e.target.value)}
                      className="w-full bg-dark-bg border border-dark-border text-dark-text-primary rounded px-3 py-2"
                      required
                    />
                  </div>
                  <div>
                    <label className="block mb-2 text-dark-text-secondary">Data de Pagamento</label>
                    <input
                      type="date"
                      value={paymentDate}
                      onChange={(e) => setPaymentDate(e.target.value)}
                      className="w-full bg-dark-bg border border-dark-border text-dark-text-primary rounded px-3 py-2"
                      required
                    />
                  </div>
                </div>
                <button type="submit" className="bg-dark-accent text-white px-4 py-2 rounded hover:opacity-90">
                  Salvar
                </button>
              </form>
            )}

            <div className="bg-dark-surface rounded border border-dark-border">
              <table className="w-full">
                <thead className="border-b border-dark-border">
                  <tr>
                    <th className="text-left p-4 text-dark-text-secondary">Membro</th>
                    <th className="text-left p-4 text-dark-text-secondary">Mês/Ano</th>
                    <th className="text-left p-4 text-dark-text-secondary">Valor</th>
                    <th className="text-left p-4 text-dark-text-secondary">Data</th>
                  </tr>
                </thead>
                <tbody>
                  {payments.map((payment) => {
                    const member = members.find((m) => m.id === payment.member_id);
                    return (
                      <tr key={payment.id} className="border-b border-dark-border last:border-0">
                        <td className="p-4 text-dark-text-primary">{member?.name || 'N/A'}</td>
                        <td className="p-4 text-dark-text-secondary">
                          {payment.month}/{payment.year}
                        </td>
                        <td className="p-4 text-dark-text-primary">{formatCurrency(payment.amount_brl)}</td>
                        <td className="p-4 text-dark-text-secondary">{formatDate(payment.payment_date)}</td>
                      </tr>
                    );
                  })}
                  {payments.length === 0 && (
                    <tr>
                      <td colSpan={4} className="p-8 text-center text-dark-text-secondary">
                        Nenhum pagamento registrado
                      </td>
                    </tr>
                  )}
                </tbody>
              </table>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};
