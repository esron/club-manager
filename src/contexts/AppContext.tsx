import { createContext, useContext, useState, ReactNode } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Member, Payment } from '../types';
import { useAuth } from './AuthContext';

interface AppContextType {
  members: Member[];
  payments: Payment[];
  refreshMembers: () => Promise<void>;
  refreshPayments: () => Promise<void>;
  addMember: (name: string, startDate: string) => Promise<void>;
  addPayment: (memberId: number, month: number, year: number, amount: number, paymentDate: string) => Promise<void>;
}

const AppContext = createContext<AppContextType | undefined>(undefined);

export const AppProvider = ({ children }: { children: ReactNode }) => {
  const { password } = useAuth();
  const [members, setMembers] = useState<Member[]>([]);
  const [payments, setPayments] = useState<Payment[]>([]);

  const refreshMembers = async () => {
    if (!password) return;
    const data = await invoke<Member[]>('get_members_cmd', { password });
    setMembers(data);
  };

  const refreshPayments = async () => {
    if (!password) return;
    const data = await invoke<Payment[]>('get_payments_cmd', { password });
    setPayments(data);
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

  return (
    <AppContext.Provider value={{ members, payments, refreshMembers, refreshPayments, addMember, addPayment }}>
      {children}
    </AppContext.Provider>
  );
};

export const useApp = () => {
  const context = useContext(AppContext);
  if (!context) throw new Error('useApp must be used within AppProvider');
  return context;
};
