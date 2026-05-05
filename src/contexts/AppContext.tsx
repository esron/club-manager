import { createContext, useContext, useState, ReactNode } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Member, Payment, AppSettings } from '../types';
import { useAuth } from './AuthContext';

interface AppContextType {
  members: Member[];
  payments: Payment[];
  settings: AppSettings;
  refreshMembers: () => Promise<void>;
  refreshPayments: () => Promise<void>;
  refreshSettings: () => Promise<void>;
  updateSetting: (key: string, value: string) => Promise<void>;
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

  return (
    <AppContext.Provider value={{ members, payments, settings, refreshMembers, refreshPayments, refreshSettings, updateSetting, addMember, addPayment, updateMemberActive, updateMemberName, deletePayment }}>
      {children}
    </AppContext.Provider>
  );
};

export const useApp = () => {
  const context = useContext(AppContext);
  if (!context) throw new Error('useApp must be used within AppProvider');
  return context;
};
