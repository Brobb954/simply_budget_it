export interface Entry {
  type: string;
  description: string;
  amount: number;
}

export interface Totals {
  total_income: number;
  total_expense: number;
  balance: number;
}

export interface TransactionResponse {
  transactions: Entry[];
  totals: Totals;
}
