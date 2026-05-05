// src/components/AddPaymentModal.tsx
import { useState, useEffect } from 'react';
import { useApp } from '../contexts/AppContext';
import { MONTH_NAMES_PT } from '../types';
import { DateInput } from './DateInput';

interface PaymentPrefill {
  memberId?: number;
  memberName?: string;
  month?: number;
  year?: number;
}

interface AddPaymentModalProps {
  isOpen: boolean;
  onClose: () => void;
  prefill?: PaymentPrefill;
}

export const AddPaymentModal = ({ isOpen, onClose, prefill }: AddPaymentModalProps) => {
  const { members, settings, addPayment } = useApp();
  const [selectedMemberName, setSelectedMemberName] = useState('');
  const [selectedMemberId, setSelectedMemberId] = useState(0);
  const [paymentMonth, setPaymentMonth] = useState(new Date().getMonth() + 1);
  const [paymentYear, setPaymentYear] = useState(new Date().getFullYear());
  const [paymentAmount, setPaymentAmount] = useState('15.00');
  const [paymentDate, setPaymentDate] = useState(new Date().toISOString().split('T')[0]);
  const [error, setError] = useState('');
  const [submitting, setSubmitting] = useState(false);

  const activeMembers = members.filter(m => m.active);

  useEffect(() => {
    if (isOpen) {
      // Reset or pre-fill form
      if (prefill) {
        setSelectedMemberId(prefill.memberId || 0);
        setSelectedMemberName(prefill.memberName || '');
        setPaymentMonth(prefill.month || new Date().getMonth() + 1);
        setPaymentYear(prefill.year || new Date().getFullYear());
      } else {
        setSelectedMemberId(0);
        setSelectedMemberName('');
        setPaymentMonth(new Date().getMonth() + 1);
        setPaymentYear(new Date().getFullYear());
      }
      setPaymentAmount(settings.minimumFee);
      setPaymentDate(new Date().toISOString().split('T')[0]);
      setError('');
    }
  }, [isOpen, prefill, settings.minimumFee]);

  const handleMemberInputChange = (value: string) => {
    setSelectedMemberName(value);
    const member = activeMembers.find(m => m.name === value);
    setSelectedMemberId(member?.id || 0);
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    setSubmitting(true);

    try {
      await addPayment(selectedMemberId, paymentMonth, paymentYear, parseFloat(paymentAmount), paymentDate);
      onClose();
    } catch (err) {
      console.error('Error adding payment:', err);
      setError(String(err));
    } finally {
      setSubmitting(false);
    }
  };

  const handleClose = () => {
    if (!submitting) {
      onClose();
    }
  };

  const handleBackdropClick = (e: React.MouseEvent) => {
    if (e.target === e.currentTarget) {
      handleClose();
    }
  };

  useEffect(() => {
    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        handleClose();
      }
    };

    if (isOpen) {
      document.addEventListener('keydown', handleEscape);
    }

    return () => {
      document.removeEventListener('keydown', handleEscape);
    };
  }, [isOpen, submitting]);

  if (!isOpen) return null;

  const isPrefilled = !!prefill?.memberId;

  return (
    <div
      className="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
      onClick={handleBackdropClick}
    >
      <div className="bg-dark-surface border border-dark-border rounded-lg p-6 w-full max-w-md relative">
        <button
          onClick={handleClose}
          disabled={submitting}
          className="absolute top-4 right-4 text-dark-text-secondary hover:text-dark-text-primary disabled:opacity-50"
        >
          ✕
        </button>

        <h2 className="text-xl font-bold text-dark-text-primary mb-4">Adicionar Pagamento</h2>

        <form onSubmit={handleSubmit}>
          <div className="mb-4">
            <label className="block mb-2 text-dark-text-secondary">Membro</label>
            <input
              list="member-list-modal"
              value={selectedMemberName}
              onChange={(e) => handleMemberInputChange(e.target.value)}
              className="w-full bg-dark-bg border border-dark-border text-dark-text-primary rounded px-3 py-2"
              placeholder="Digite o nome do membro"
              required
              disabled={isPrefilled}
            />
            <datalist id="member-list-modal">
              {activeMembers.map((member) => (
                <option key={member.id} value={member.name} />
              ))}
            </datalist>
          </div>

          <div className="mb-4">
            <label className="block mb-2 text-dark-text-secondary">Mês</label>
            <select
              value={paymentMonth}
              onChange={(e) => setPaymentMonth(Number(e.target.value))}
              className="w-full bg-dark-bg border border-dark-border text-dark-text-primary rounded px-3 py-2"
              required
              disabled={isPrefilled}
            >
              {MONTH_NAMES_PT.map((name, idx) => (
                <option key={idx} value={idx + 1}>
                  {name}
                </option>
              ))}
            </select>
          </div>

          <div className="mb-4">
            <label className="block mb-2 text-dark-text-secondary">Ano</label>
            <input
              type="number"
              value={paymentYear}
              onChange={(e) => setPaymentYear(Number(e.target.value))}
              className="w-full bg-dark-bg border border-dark-border text-dark-text-primary rounded px-3 py-2"
              required
              disabled={isPrefilled}
            />
          </div>

          <div className="mb-4">
            <label className="block mb-2 text-dark-text-secondary">Valor (R$)</label>
            <input
              type="text"
              value={paymentAmount}
              onChange={(e) => setPaymentAmount(e.target.value)}
              className="w-full bg-dark-bg border border-dark-border text-dark-text-primary rounded px-3 py-2"
              placeholder="15.00"
              required
            />
          </div>

          <div className="mb-4">
            <label className="block mb-2 text-dark-text-secondary">Data do Pagamento</label>
            <DateInput
              value={paymentDate}
              onChange={setPaymentDate}
              className="w-full"
            />
          </div>

          {error && <p className="text-dark-error text-sm mb-4">{error}</p>}

          <div className="flex gap-2 justify-end">
            <button
              type="button"
              onClick={handleClose}
              disabled={submitting}
              className="px-4 py-2 border border-dark-border text-dark-text-secondary rounded hover:bg-dark-bg disabled:opacity-50"
            >
              Cancelar
            </button>
            <button
              type="submit"
              disabled={submitting}
              className="px-4 py-2 bg-dark-accent text-white rounded hover:opacity-90 disabled:opacity-50"
            >
              {submitting ? 'Salvando...' : 'Salvar'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};
