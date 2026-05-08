import { createContext, useContext, useState, ReactNode } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Member, Payment, AppSettings, MemberDebtInfo, PaymentPrefill } from '../types';
import { useAuth } from './AuthContext';

interface AppContextType {
  members: Member[];
  payments: Payment[];
  settings: AppSettings;
  paymentModalOpen: boolean;
  paymentModalPrefill?: PaymentPrefill;
  showReAuthModal: boolean;
  reAuthCallback: ((password: string) => void) | null;
  refreshMembers: () => Promise<void>;
  refreshPayments: () => Promise<void>;
  refreshSettings: () => Promise<void>;
  updateSetting: (key: string, value: string) => Promise<void>;
  getMemberDebt: (memberId: number) => Promise<MemberDebtInfo>;
  getAllDebts: () => Promise<MemberDebtInfo[]>;
  openPaymentModal: (prefill?: PaymentPrefill) => void;
  closePaymentModal: () => void;
  requestReAuth: (callback: (password: string) => void) => void;
  closeReAuthModal: () => void;
  addMember: (name: string, startDate: string) => Promise<void>;
  addPayment: (memberId: number, month: number, year: number, amount: number, paymentDate: string) => Promise<void>;
  updateMemberActive: (id: number, active: boolean) => Promise<void>;
  updateMemberName: (id: number, name: string) => Promise<void>;
  deletePayment: (id: number) => Promise<void>;
}

const AppContext = createContext<AppContextType | undefined>(undefined);

export const AppProvider = ({ children }: { children: ReactNode }) => {
  const { password } = useAuth();
  const [members, setMembers] = useState<Member[]>([]);
  const [payments, setPayments] = useState<Payment[]>([]);
  const [settings, setSettings] = useState<AppSettings>({ minimumFee: '15.00' });
  const [paymentModalOpen, setPaymentModalOpen] = useState(false);
  const [paymentModalPrefill, setPaymentModalPrefill] = useState<PaymentPrefill>();
  const [showReAuthModal, setShowReAuthModal] = useState(false);
  const [reAuthCallback, setReAuthCallback] = useState<((password: string) => void) | null>(null);

  const refreshMembers = async () => {
    if (!password) return;
    try {
      const data = await invoke<Member[]>('get_all_members_cmd', { password });
      setMembers(data);
    } catch (err) {
      console.error('Error refreshing members:', err);
    }
  };

  const refreshPayments = async () => {
    if (!password) return;
    const data = await invoke<Payment[]>('get_payments_cmd', { password });
    setPayments(data);
  };

  const refreshSettings = async () => {
    if (!password) return;
    try {
      const minimumFee = await invoke<string>('get_setting_cmd', {
        password,
        key: 'minimum_fee_brl'
      });
      setSettings({ minimumFee });
    } catch (err) {
      console.error('Error refreshing settings:', err);
    }
  };

  const updateSetting = async (key: string, value: string) => {
    if (!password) throw new Error('Not authenticated');
    await invoke('update_setting_cmd', { password, key, value });
    await refreshSettings();
  };

  const getMemberDebt = async (memberId: number): Promise<MemberDebtInfo> => {
    if (!password) throw new Error('Not authenticated');
    const data = await invoke<MemberDebtInfo>('get_member_debt_cmd', { password, memberId });
    return data;
  };

  const getAllDebts = async (): Promise<MemberDebtInfo[]> => {
    if (!password) throw new Error('Not authenticated');
    const data = await invoke<MemberDebtInfo[]>('get_all_debts_cmd', { password });
    return data;
  };

  const addMember = async (name: string, startDate: string) => {
    if (!password) throw new Error('Not authenticated');
    await invoke('add_member_cmd', { password, name, startDate });
    await refreshMembers();
  };

  const addPayment = async (memberId: number, month: number, year: number, amount: number, paymentDate: string) => {
    if (!password) throw new Error('Not authenticated');
    await invoke('add_payment_cmd', { password, memberId, month, year, amountBrl: amount, paymentDate });
    await refreshPayments();
  };

  const updateMemberActive = async (id: number, active: boolean) => {
    if (!password) throw new Error('Not authenticated');
    await invoke('update_member_active_cmd', { password, id, active });
    await refreshMembers();
  };

  const updateMemberName = async (id: number, name: string) => {
    if (!password) throw new Error('Not authenticated');
    await invoke('update_member_name_cmd', { password, id, name });
    await refreshMembers();
  };

  const deletePayment = async (id: number) => {
    if (!password) throw new Error('Not authenticated');
    await invoke('delete_payment_cmd', { password, id });
    await refreshPayments();
  };

  const openPaymentModal = (prefill?: PaymentPrefill) => {
    setPaymentModalPrefill(prefill);
    setPaymentModalOpen(true);
  };

  const closePaymentModal = () => {
    setPaymentModalOpen(false);
    setPaymentModalPrefill(undefined);
  };

  const requestReAuth = (callback: (password: string) => void) => {
    setReAuthCallback(() => callback);
    setShowReAuthModal(true);
  };

  const closeReAuthModal = () => {
    setShowReAuthModal(false);
    setReAuthCallback(null);
  };

  return (
    <AppContext.Provider value={{
      members,
      payments,
      settings,
      paymentModalOpen,
      paymentModalPrefill,
      showReAuthModal,
      reAuthCallback,
      refreshMembers,
      refreshPayments,
      refreshSettings,
      updateSetting,
      getMemberDebt,
      getAllDebts,
      openPaymentModal,
      closePaymentModal,
      requestReAuth,
      closeReAuthModal,
      addMember,
      addPayment,
      updateMemberActive,
      updateMemberName,
      deletePayment
    }}>
      {children}
    </AppContext.Provider>
  );
};

export const useApp = () => {
  const context = useContext(AppContext);
  if (!context) throw new Error('useApp must be used within AppProvider');
  return context;
};
