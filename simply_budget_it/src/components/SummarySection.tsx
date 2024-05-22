interface SummarySectionProps {
    totalIncome: number;
    totalExpense: number;
    balance: number;
}

const SummarySection: React.FC<SummarySectionProps> = ({totalIncome,totalExpenses, balance}) => {
    return (
        <div className="container mx-auto my-4 p-4 bg-gray-200 rounded shadow">
            <div className="flex justify between">
                <span>Total Income: ${totalIncome.toFixed(2)}</span>
                <span> Total Expense: ${totalExpenses.toFixed(2)}</span>
                <span>Balance: ${balance.toFixed(2)}</span>
            </div>
        </div>
    );
};

export default SummarySection;