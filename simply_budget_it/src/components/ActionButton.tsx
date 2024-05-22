// src/components/ActionButton.tsx
interface ActionButtonProps {
    children: React.ReactNode;
    onClick: () => void;
    variant: 'primary' | 'secondary';
  }
  
  const ActionButton: React.FC<ActionButtonProps> = ({ children, onClick, variant }) => {
    const buttonStyles = {
      primary: 'bg-blue-500 hover:bg-blue-700 text-white',
      secondary: 'bg-gray-300 hover:bg-gray-400 text-black',
    };
  
    return (
      <button
        className={`px-4 py-2 rounded shadow ${buttonStyles[variant]}`}
        onClick={onClick}
      >
        {children}
      </button>
    );
  };
  
  export default ActionButton;