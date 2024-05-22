import {Entry} from '/Users/brandonrobb/simple_budget/simply_budget_it/src/interfaces/Entry.ts';

interface TableViewProps {
    entries: Entry[];
}

const TableView: React.FC<TableViewProps> = ({entries}) => {
    return (
        <div className='container mx-auto my-4'>
            <table className='min-w-full table-auto'>
                <thead className='bg-gray-200'>
                    <tr>
                        <th className='px-4 py-2'>Type</th>
                        <th className='px-4 py-2'>Description</th>
                        <th className='px-4 py-2'>Amount</th>
                    </tr>
                </thead>
                <tbody>
                    {entries.map((entry) => (
                        <tr key={entry.id} className='hover:bg-gray-100'>
                            <td className='border px-4 py-2'>{entry.type}</td>
                            <td className='border px-4 py-2'>{entry.description}</td>
                            <td className='border px-4 py-2'>{entry.amount.toFixed(2)}</td>
                       </tr>
                    ))}
                </tbody>
            </table>
        </div>
    )
}

export default TableView;