import tkinter as tk
from data_manager import DataManager
from gui import BudgetApp

def main():
    root = tk.Tk()
    data_manager = DataManager()
    app = BudgetApp(root, data_manager)
    root.mainloop()


if __name__ == "__main__":
    main()