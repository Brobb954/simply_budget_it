import tkinter as tk
from data_manager import DataManager
from gui import BudgetApp
import atexit


def main():
    root = tk.Tk()
    data_manager = DataManager('budget_data.xlsx')
    data_manager.load_or_create_workbook()
    app = BudgetApp(root, data_manager)
    atexit.register(data_manager.close)
    root.mainloop()



if __name__ == "__main__":
    main()