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

export const formatCurrency = (value: number): string => {
  return new Intl.NumberFormat('pt-BR', {
    style: 'currency',
    currency: 'BRL',
  }).format(value);
};

export const formatDate = (dateStr: string): string => {
  const date = new Date(dateStr);
  return new Intl.DateFormat('pt-BR').format(date);
};
