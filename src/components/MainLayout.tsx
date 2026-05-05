import { useEffect, useState, useRef } from 'react';
import { useApp } from '../contexts/AppContext';
import { formatCurrency, formatDate } from '../types';
import { DateInput } from './DateInput';
import { SettingsScreen } from './SettingsScreen';
import { DashboardScreen } from './DashboardScreen';
import { MemberDetailView } from './MemberDetailView';

export const MainLayout = () => {
  const { members, payments, settings, refreshMembers, refreshPayments, refreshSettings, getAllDebts, addMember, addPayment, updateMemberActive, updateMemberName, deletePayment } = useApp();
  const [activeTab, setActiveTab] = useState<'dashboard' | 'members' | 'payments' | 'settings'>('dashboard');
  const [showAddMember, setShowAddMember] = useState(false);
  const [showAddPayment, setShowAddPayment] = useState(false);
  const [memberName, setMemberName] = useState('');
  const [memberStartDate, setMemberStartDate] = useState('');
  const [paymentMemberId, setPaymentMemberId] = useState<number>(0);
  const [selectedMemberName, setSelectedMemberName] = useState('');
  const [paymentMonth, setPaymentMonth] = useState(new Date().getMonth() + 1);
  const [paymentYear, setPaymentYear] = useState(new Date().getFullYear());
  const [paymentAmount, setPaymentAmount] = useState('15.00');
  const [paymentDate, setPaymentDate] = useState(new Date().toISOString().split('T')[0]);
  const [paymentError, setPaymentError] = useState('');
  const [memberError, setMemberError] = useState('');
  const [editingMemberId, setEditingMemberId] = useState<number | null>(null);
  const [editingMemberName, setEditingMemberName] = useState('');
  const [selectedMemberId, setSelectedMemberId] = useState<number | null>(null);
  const [viewingMemberDetail, setViewingMemberDetail] = useState(false);
  const [membersPage, setMembersPage] = useState(1);
  const [membersPageSize, setMembersPageSize] = useState(15);
  const [inactiveMembersPage, setInactiveMembersPage] = useState(1);
  const [inactiveMembersPageSize, setInactiveMembersPageSize] = useState(15);
  const [paymentsPage, setPaymentsPage] = useState(1);
  const [paymentsPageSize, setPaymentsPageSize] = useState(15);

  useEffect(() => {
    refreshMembers();
    refreshPayments();
    refreshSettings();
  }, []);

  const activeMembers = members.filter((m) => {
    return m.active === true || m.active === 1 || (m.active as any) === "true" || (m.active as any) === "1";
  });

  const inactiveMembers = members.filter((m) => {
    return m.active === false || m.active === 0 || (m.active as any) === "false" || (m.active as any) === "0";
  });

  const paginatedActiveMembers = activeMembers.slice(
    (membersPage - 1) * membersPageSize,
    membersPage * membersPageSize
  );
  const activeMembersTotalPages = Math.ceil(activeMembers.length / membersPageSize);

  const paginatedInactiveMembers = inactiveMembers.slice(
    (inactiveMembersPage - 1) * inactiveMembersPageSize,
    inactiveMembersPage * inactiveMembersPageSize
  );
  const inactiveMembersTotalPages = Math.ceil(inactiveMembers.length / inactiveMembersPageSize);

  const paginatedPayments = payments.slice(
    (paymentsPage - 1) * paymentsPageSize,
    paymentsPage * paymentsPageSize
  );
  const paymentsTotalPages = Math.ceil(payments.length / paymentsPageSize);

  const handleAddMember = async (e: React.FormEvent) => {
    e.preventDefault();
    setMemberError('');
    try {
      await addMember(memberName, memberStartDate);
      setMemberName('');
      setMemberStartDate('');
      setShowAddMember(false);
    } catch (err) {
      console.error('Error adding member:', err);
      setMemberError(String(err));
    }
  };

  const handleAddPayment = async (e: React.FormEvent) => {
    e.preventDefault();
    setPaymentError('');
    try {
      await addPayment(paymentMemberId, paymentMonth, paymentYear, parseFloat(paymentAmount), paymentDate);
      setSelectedMemberName('');
      setPaymentMemberId(0);
      setShowAddPayment(false);
    } catch (err) {
      console.error('Error adding payment:', err);
      setPaymentError(String(err));
    }
  };

  const handleMemberInputChange = (value: string) => {
    setSelectedMemberName(value);
    const member = activeMembers.find((m) => m.name === value);
    setPaymentMemberId(member?.id || 0);
  };

  const handleDeactivateMember = async (id: number) => {
    if (confirm('Tem certeza que deseja desativar este membro?')) {
      await updateMemberActive(id, false);
    }
  };

  const startEditingMember = (id: number, currentName: string) => {
    setEditingMemberId(id);
    setEditingMemberName(currentName);
    setMemberError('');
  };

  const cancelEditingMember = () => {
    setEditingMemberId(null);
    setEditingMemberName('');
    setMemberError('');
  };

  const saveEditingMember = async () => {
    if (!editingMemberId) return;
    setMemberError('');
    try {
      await updateMemberName(editingMemberId, editingMemberName);
      setEditingMemberId(null);
      setEditingMemberName('');
    } catch (err) {
      console.error('Error updating member:', err);
      setMemberError(String(err));
    }
  };

  const handleDeletePayment = async (id: number) => {
    if (confirm('Tem certeza que deseja excluir este pagamento?')) {
      await deletePayment(id);
    }
  };

  return (
    <div className="min-h-screen bg-dark-bg flex">
      <div className="w-64 bg-dark-surface border-r border-dark-border p-4">
        <h1 className="text-xl font-bold mb-8 text-dark-text-primary">Gestor do Clube</h1>
        <nav>
          <button
            onClick={() => setActiveTab('dashboard')}
            className={`w-full text-left px-4 py-2 rounded mb-2 ${
              activeTab === 'dashboard' ? 'bg-dark-accent text-white' : 'text-dark-text-primary hover:bg-dark-bg'
            }`}
          >
            Dashboard
          </button>
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
            className={`w-full text-left px-4 py-2 rounded mb-2 ${
              activeTab === 'payments' ? 'bg-dark-accent text-white' : 'text-dark-text-primary hover:bg-dark-bg'
            }`}
          >
            Pagamentos
          </button>
          <button
            onClick={() => setActiveTab('settings')}
            className={`w-full text-left px-4 py-2 rounded ${
              activeTab === 'settings' ? 'bg-dark-accent text-white' : 'text-dark-text-primary hover:bg-dark-bg'
            }`}
          >
            Configurações
          </button>
        </nav>
      </div>

      <div className="flex-1">
        {activeTab === 'dashboard' && (
          <DashboardScreen />
        )}

        {activeTab === 'members' && !viewingMemberDetail && (
          <div className="p-8">
            <div className="flex justify-between items-center mb-6">
              <h2 className="text-2xl font-bold text-dark-text-primary">Membros Ativos ({activeMembers.length})</h2>
              <button
                onClick={() => {
                  setShowAddMember(!showAddMember);
                  setMemberError('');
                }}
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
                    <DateInput
                      value={memberStartDate}
                      onChange={setMemberStartDate}
                      className="w-full bg-dark-bg border border-dark-border text-dark-text-primary rounded px-3 py-2"
                      required
                    />
                  </div>
                </div>
                {memberError && <p className="text-dark-error mb-4 text-sm">{memberError}</p>}
                <button type="submit" className="bg-dark-accent text-white px-4 py-2 rounded hover:opacity-90">
                  Salvar
                </button>
              </form>
            )}

            <div className="flex justify-between items-center mb-4">
              <div className="flex items-center gap-2">
                <span className="text-dark-text-secondary text-sm">Itens por página:</span>
                <select
                  value={membersPageSize}
                  onChange={(e) => {
                    setMembersPageSize(Number(e.target.value));
                    setMembersPage(1);
                  }}
                  className="text-sm"
                >
                  <option value={15}>15</option>
                  <option value={30}>30</option>
                  <option value={100}>100</option>
                </select>
              </div>
            </div>

            <div className="bg-dark-surface rounded border border-dark-border">
              <table className="w-full">
                <thead className="border-b border-dark-border">
                  <tr>
                    <th className="text-left p-4 text-dark-text-secondary">Nome</th>
                    <th className="text-left p-4 text-dark-text-secondary">Data de Início</th>
                    <th className="text-left p-4 text-dark-text-secondary">Status</th>
                    <th className="text-left p-4 text-dark-text-secondary">Ações</th>
                  </tr>
                </thead>
                <tbody>
                  {paginatedActiveMembers.map((member) => (
                    <tr key={member.id} className="border-b border-dark-border last:border-0">
                      <td className="p-4 text-dark-text-primary">
                        {editingMemberId === member.id ? (
                          <div className="flex gap-2 items-center">
                            <input
                              type="text"
                              value={editingMemberName}
                              onChange={(e) => setEditingMemberName(e.target.value)}
                              className="bg-dark-bg border border-dark-border text-dark-text-primary rounded px-2 py-1"
                              autoFocus
                            />
                            <button
                              onClick={saveEditingMember}
                              className="bg-dark-success text-white px-2 py-1 rounded text-sm hover:opacity-90"
                            >
                              Salvar
                            </button>
                            <button
                              onClick={cancelEditingMember}
                              className="bg-dark-border text-dark-text-primary px-2 py-1 rounded text-sm hover:opacity-90"
                            >
                              Cancelar
                            </button>
                          </div>
                        ) : (
                          <button
                            onClick={() => {
                              setSelectedMemberId(member.id);
                              setViewingMemberDetail(true);
                            }}
                            className="text-dark-accent hover:underline text-left"
                          >
                            {member.name}
                          </button>
                        )}
                      </td>
                      <td className="p-4 text-dark-text-secondary">{formatDate(member.start_date)}</td>
                      <td className="p-4">
                        <span className="text-dark-success">Ativo</span>
                      </td>
                      <td className="p-4">
                        <div className="flex gap-2">
                          <button
                            onClick={() => startEditingMember(member.id, member.name)}
                            className="bg-dark-accent text-white px-2 py-1 rounded text-sm hover:opacity-90"
                            disabled={editingMemberId !== null}
                          >
                            Editar
                          </button>
                          <button
                            onClick={() => handleDeactivateMember(member.id)}
                            className="bg-dark-warning text-white px-2 py-1 rounded text-sm hover:opacity-90"
                            disabled={editingMemberId !== null}
                          >
                            Desativar
                          </button>
                        </div>
                      </td>
                    </tr>
                  ))}
                  {activeMembers.length === 0 && (
                    <tr>
                      <td colSpan={4} className="p-8 text-center text-dark-text-secondary">
                        Nenhum membro ativo
                      </td>
                    </tr>
                  )}
                </tbody>
              </table>
            </div>
            {memberError && editingMemberId !== null && (
              <p className="text-dark-error mt-2 text-sm">{memberError}</p>
            )}

            {activeMembersTotalPages > 1 && (
              <div className="flex justify-center items-center gap-2 mt-4">
                <button
                  onClick={() => setMembersPage(Math.max(1, membersPage - 1))}
                  disabled={membersPage === 1}
                  className="bg-dark-surface border border-dark-border text-dark-text-primary px-3 py-1 rounded disabled:opacity-50"
                >
                  Anterior
                </button>
                <span className="text-dark-text-secondary text-sm">
                  Página {membersPage} de {activeMembersTotalPages}
                </span>
                <button
                  onClick={() => setMembersPage(Math.min(activeMembersTotalPages, membersPage + 1))}
                  disabled={membersPage === activeMembersTotalPages}
                  className="bg-dark-surface border border-dark-border text-dark-text-primary px-3 py-1 rounded disabled:opacity-50"
                >
                  Próxima
                </button>
              </div>
            )}

            {inactiveMembers.length > 0 && (
              <>
                <h3 className="text-xl font-bold text-dark-text-primary mt-8 mb-4">Membros Inativos ({inactiveMembers.length})</h3>

                <div className="flex justify-between items-center mb-4">
                  <div className="flex items-center gap-2">
                    <span className="text-dark-text-secondary text-sm">Itens por página:</span>
                    <select
                      value={inactiveMembersPageSize}
                      onChange={(e) => {
                        setInactiveMembersPageSize(Number(e.target.value));
                        setInactiveMembersPage(1);
                      }}
                      className="text-sm"
                    >
                      <option value={15}>15</option>
                      <option value={30}>30</option>
                      <option value={100}>100</option>
                    </select>
                  </div>
                </div>

                <div className="bg-dark-surface rounded border border-dark-border">
                  <table className="w-full">
                    <thead className="border-b border-dark-border">
                      <tr>
                        <th className="text-left p-4 text-dark-text-secondary">Nome</th>
                        <th className="text-left p-4 text-dark-text-secondary">Data de Início</th>
                        <th className="text-left p-4 text-dark-text-secondary">Status</th>
                        <th className="text-left p-4 text-dark-text-secondary">Ações</th>
                      </tr>
                    </thead>
                    <tbody>
                      {paginatedInactiveMembers.map((member) => (
                        <tr key={member.id} className="border-b border-dark-border last:border-0">
                          <td className="p-4 text-dark-text-secondary">{member.name}</td>
                          <td className="p-4 text-dark-text-secondary">{formatDate(member.start_date)}</td>
                          <td className="p-4">
                            <span className="text-dark-text-secondary">Inativo</span>
                          </td>
                          <td className="p-4">
                            <button
                              onClick={() => updateMemberActive(member.id, true)}
                              className="bg-dark-success text-white px-2 py-1 rounded text-sm hover:opacity-90"
                            >
                              Reativar
                            </button>
                          </td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>

                {inactiveMembersTotalPages > 1 && (
                  <div className="flex justify-center items-center gap-2 mt-4">
                    <button
                      onClick={() => setInactiveMembersPage(Math.max(1, inactiveMembersPage - 1))}
                      disabled={inactiveMembersPage === 1}
                      className="bg-dark-surface border border-dark-border text-dark-text-primary px-3 py-1 rounded disabled:opacity-50"
                    >
                      Anterior
                    </button>
                    <span className="text-dark-text-secondary text-sm">
                      Página {inactiveMembersPage} de {inactiveMembersTotalPages}
                    </span>
                    <button
                      onClick={() => setInactiveMembersPage(Math.min(inactiveMembersTotalPages, inactiveMembersPage + 1))}
                      disabled={inactiveMembersPage === inactiveMembersTotalPages}
                      className="bg-dark-surface border border-dark-border text-dark-text-primary px-3 py-1 rounded disabled:opacity-50"
                    >
                      Próxima
                    </button>
                  </div>
                )}
              </>
            )}
          </div>
        )}

        {activeTab === 'members' && viewingMemberDetail && selectedMemberId && (
          <MemberDetailView
            memberId={selectedMemberId}
            onBack={() => {
              setViewingMemberDetail(false);
              setSelectedMemberId(null);
              refreshMembers();
              refreshPayments();
            }}
          />
        )}

        {activeTab === 'payments' && (
          <div className="p-8">
            <div className="flex justify-between items-center mb-6">
              <h2 className="text-2xl font-bold text-dark-text-primary">Pagamentos ({payments.length})</h2>
              <button
                onClick={() => {
                  setShowAddPayment(!showAddPayment);
                  if (!showAddPayment) {
                    setSelectedMemberName('');
                    setPaymentMemberId(0);
                    setPaymentError('');
                  }
                }}
                className="bg-dark-accent text-white px-4 py-2 rounded hover:opacity-90"
                disabled={activeMembers.length === 0}
              >
                + Novo Pagamento
              </button>
            </div>

            {showAddPayment && (
              <form onSubmit={handleAddPayment} className="bg-dark-surface p-4 rounded border border-dark-border mb-6">
                <div className="grid grid-cols-2 gap-4 mb-4">
                  <div>
                    <label className="block mb-2 text-dark-text-secondary">Membro</label>
                    <input
                      type="text"
                      list="members-list"
                      value={selectedMemberName}
                      onChange={(e) => handleMemberInputChange(e.target.value)}
                      className="w-full bg-dark-bg border border-dark-border text-dark-text-primary rounded px-3 py-2"
                      placeholder="Digite para buscar..."
                      required
                    />
                    <datalist id="members-list">
                      {activeMembers.map((m) => (
                        <option key={m.id} value={m.name} />
                      ))}
                    </datalist>
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
                    <DateInput
                      value={paymentDate}
                      onChange={setPaymentDate}
                      className="w-full bg-dark-bg border border-dark-border text-dark-text-primary rounded px-3 py-2"
                      required
                    />
                  </div>
                </div>
                {paymentError && <p className="text-dark-error mb-4 text-sm">{paymentError}</p>}
                <button type="submit" className="bg-dark-accent text-white px-4 py-2 rounded hover:opacity-90">
                  Salvar
                </button>
              </form>
            )}

            <div className="flex justify-between items-center mb-4">
              <div className="flex items-center gap-2">
                <span className="text-dark-text-secondary text-sm">Itens por página:</span>
                <select
                  value={paymentsPageSize}
                  onChange={(e) => {
                    setPaymentsPageSize(Number(e.target.value));
                    setPaymentsPage(1);
                  }}
                  className="text-sm"
                >
                  <option value={15}>15</option>
                  <option value={30}>30</option>
                  <option value={100}>100</option>
                </select>
              </div>
            </div>

            <div className="bg-dark-surface rounded border border-dark-border">
              <table className="w-full">
                <thead className="border-b border-dark-border">
                  <tr>
                    <th className="text-left p-4 text-dark-text-secondary">Membro</th>
                    <th className="text-left p-4 text-dark-text-secondary">Mês/Ano</th>
                    <th className="text-left p-4 text-dark-text-secondary">Valor</th>
                    <th className="text-left p-4 text-dark-text-secondary">Data</th>
                    <th className="text-left p-4 text-dark-text-secondary">Ações</th>
                  </tr>
                </thead>
                <tbody>
                  {paginatedPayments.map((payment) => {
                    const member = members.find((m) => m.id === payment.member_id);
                    return (
                      <tr key={payment.id} className="border-b border-dark-border last:border-0">
                        <td className="p-4 text-dark-text-primary">{member?.name || 'N/A'}</td>
                        <td className="p-4 text-dark-text-secondary">
                          {payment.month}/{payment.year}
                        </td>
                        <td className="p-4 text-dark-text-primary">{formatCurrency(payment.amount_brl)}</td>
                        <td className="p-4 text-dark-text-secondary">{formatDate(payment.payment_date)}</td>
                        <td className="p-4">
                          <button
                            onClick={() => handleDeletePayment(payment.id)}
                            className="bg-dark-error text-white px-2 py-1 rounded text-sm hover:opacity-90"
                          >
                            Excluir
                          </button>
                        </td>
                      </tr>
                    );
                  })}
                  {payments.length === 0 && (
                    <tr>
                      <td colSpan={5} className="p-8 text-center text-dark-text-secondary">
                        Nenhum pagamento registrado
                      </td>
                    </tr>
                  )}
                </tbody>
              </table>
            </div>

            {paymentsTotalPages > 1 && (
              <div className="flex justify-center items-center gap-2 mt-4">
                <button
                  onClick={() => setPaymentsPage(Math.max(1, paymentsPage - 1))}
                  disabled={paymentsPage === 1}
                  className="bg-dark-surface border border-dark-border text-dark-text-primary px-3 py-1 rounded disabled:opacity-50"
                >
                  Anterior
                </button>
                <span className="text-dark-text-secondary text-sm">
                  Página {paymentsPage} de {paymentsTotalPages}
                </span>
                <button
                  onClick={() => setPaymentsPage(Math.min(paymentsTotalPages, paymentsPage + 1))}
                  disabled={paymentsPage === paymentsTotalPages}
                  className="bg-dark-surface border border-dark-border text-dark-text-primary px-3 py-1 rounded disabled:opacity-50"
                >
                  Próxima
                </button>
              </div>
            )}
          </div>
        )}

        {activeTab === 'settings' && (
          <SettingsScreen />
        )}
      </div>
    </div>
  );
};
