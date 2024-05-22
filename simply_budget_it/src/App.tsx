import React, { useEffect, useState } from "react";
import Header from "./components/Header";
import TableView from "./components/TableView";
import ActionButton from "./components/ActionButton";
import SummarySection from "./components/SummarySection";
import { Entry, Totals, TransactionResponse } from "./Interfaces";
import { invoke } from "@tauri-apps/api/tauri";

const App: React.FC = () => {
  const [entries, setEntries] = useState<Entry[]>([]);
  const [totals, setTotals] = useState<>;
  useEffect(() => {
    const fetchData = async () => {
      try {
        const response: TransactionResponse = await invoke(
          "fetch_transactions_and_totals"
        );
        const { transactions, totals } = response;
        setEntries(transactions);
        setTotals(totals);
      } catch (error) {
        console.error("Failed to fetch entries:", error);
      }
    };
    fetchData();
  }, []);
};

export default App;
