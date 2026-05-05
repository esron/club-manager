export interface Member {
  id: number;
  name: string;
  start_date: string;
  created_at: string;
  active: boolean;
}

export interface Payment {
  id: number;
  member_id: number;
  month: number;
  year: number;
  amount_brl: number;
  payment_date: string;
  created_at: string;
}

export interface AppSettings {
  minimumFee: string;
}

export const formatCurrency = (value: number): string => {
  return new Intl.NumberFormat('pt-BR', {
    style: 'currency',
    currency: 'BRL',
  }).format(value);
};

export const formatDate = (dateStr: string): string => {
  const [year, month, day] = dateStr.split('T')[0].split('-');
  return `${day.padStart(2, '0')}/${month.padStart(2, '0')}/${year}`;
};

export interface UnpaidMonth {
  month: number;
  year: number;
  amount: number;
  display: string;
}

export interface MemberDebtInfo {
  member_id: number;
  member_name: string;
  total_debt: number;
  unpaid_months: UnpaidMonth[];
}

export const MONTH_NAMES_PT = [
  'Janeiro', 'Fevereiro', 'Março', 'Abril', 'Maio', 'Junho',
  'Julho', 'Agosto', 'Setembro', 'Outubro', 'Novembro', 'Dezembro'
];

export const formatMonthYear = (month: number, year: number): string => {
  return `${MONTH_NAMES_PT[month - 1]} ${year}`;
};
