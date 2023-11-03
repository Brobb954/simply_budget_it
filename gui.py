import tkinter as tk
from tkinter import ttk
from tkinter import messagebox

from data_manager import DataManager

class BudgetApp:
    def __init__(self, root, data_manager):
        self.root = root
        self.data_manager = data_manager
        self.root.title("Budget Tracker")

        # Initialize variables
        self.is_income = tk.BooleanVar(value=True)
        self.total_income_var = tk.StringVar(value="Total Income: $0.00")
        self.total_expense_var = tk.StringVar(value="Total Expenses: $0.00")
        self.balance_var = tk.StringVar(value="Balance: $0.00")
        self.description_var = tk.StringVar()
        self.amount_var = tk.StringVar()
        

        #Set up GUI components
        self.setup_widgets()


    def setup_widgets(self):
        #Treeview for displaying data
        self.treeview = ttk.Treeview(self.root, columns=('Type', 'Description', 'Amount'), show='headings')
        self.treeview.grid(row=0, column=0, columnspan=5, sticky='nsew')

        #Define the column headings
        for col in self.treeview['columns']:
            self.treeview.heading(col, text=col)

        # Populate Treeview with data from Excel
        self.populate_treeview()

        # Set up bindings
        self.treeview.bind('<<TreeviewSelect>>', self.on_select)

        # User input boxes
        self.description_entry = tk.Entry(self.root, text="Description", textvariable=self.description_var)
        self.description_entry.grid(row=1, column=0, columnspan=3, sticky='ew')

        self.amount_entry = tk.Entry(self.root, text="Amount", textvariable=self.amount_var)
        self.amount_entry.grid(row=1, column=2, sticky='ew')

        self.add_button = tk.Button(self.root, text="Add Entry", command=self.add_entry)
        self.add_button.grid(row=1, column=3, sticky='ew')

        self.switch_button = tk.Button(self.root, text="Switch to Expense", command=self.switch_type, width=10)
        self.switch_button.grid(row=2, column=3, sticky='ew')

        self.delete_button = tk.Button(self.root, text="Delete Selected", command=self.delete_selected)
        self.delete_button.grid(row=2, column=0, sticky='ew')

        self.delete_all_button = tk.Button(self.root, text="Delete All", command=self.delete_all)
        self.delete_all_button.grid(row=2, column=1, sticky='ew')

        self.edit_button = tk.Button(self.root, text="Edit Selected", command=self.open_edit_window)
        self.edit_button.grid(row=2, column=2, sticky='ew')

        self.total_income_label = tk.Label(self.root, textvariable=self.total_income_var)
        self.total_income_label.grid(row=3, column=0, columnspan=2, sticky='nsew')

        self.total_expense_label = tk.Label(self.root, textvariable=self.total_expense_var)
        self.total_expense_label.grid(row=3, column=2, columnspan=2, sticky='nsew')

        self.balance_label = tk.Label(self.root, textvariable=self.balance_var)
        self.balance_label.grid(row=4, column=0, columnspan=5, sticky='ew')

        self.root.grid_rowconfigure(0, weight=1)
        self.root.grid_columnconfigure(0, weight=1)
        self.root.grid_columnconfigure(1, weight=1)
        self.root.grid_columnconfigure(2, weight=1)
        self.root.grid_columnconfigure(3, weight=1)

    def on_select(self, event):
        selected_items = self.treeview.selection()

        if selected_items:
            self.delete_button.config(state='normal')
        else:
            self.delete_button.config(state='disabled')
    

    def add_entry(self):
        description = self.description_var.get().strip()
        amount_str = self.amount_var.get()

        print(f"Description: {description}, Amount: {amount_str}")
        if not description:
            self.show_error("Description cannot be empty")
            return
        
        if not self.data_manager.is_valid_amount(amount_str):
            self.show_error("Invalid amount. Please enter a number with up to two decimal places")
            return
        
        amount = float(amount_str)

        type_str = "Income" if self.is_income.get() else "Expense"
        entry = (type_str, description, amount)
        self.treeview.insert('', 'end', values=entry)

        self.data_manager.add_data_to_excel(entry)

        self.description_var.set('')
        self.amount_var.set('')
        self.update_totals()

    def switch_type(self):

        # Changes boolean value to update button
        self.is_income.set(not self.is_income.get())

        if self.is_income.get():
            self.switch_button.config(text="Switch to Expense")
        else:
            self.switch_button.config(text="Switch to Income")


    def populate_treeview(self):
        for item in self.treeview.get_children():
            self.treeview.delete(item)

        data = self.data_manager.load_data_from_excel()

        for entry in data:
            self.treeview.insert('', 'end', values=entry)

        self.update_totals()

    def open_edit_window(self):

        # Initiate Variables
        selected_item = self.treeview.selection()
       
        
        # Checks for selection
        if selected_item:

            selected_values = self.treeview.item(selected_item, 'values')

            self.edit_window = tk.Toplevel(self.root)
            self.edit_window.title("Edit Entry")
            self.edit_window.geometry("500x200")

            self.is_income.set(selected_values[0] == "Income")
        
            self.edit_description_var = tk.StringVar(value=selected_values[1])
            self.edit_amount_var = tk.StringVar(value=selected_values[2])

            self.description_entry = tk.Entry(self.edit_window, text="Description", textvariable=self.edit_description_var)
            self.description_entry.grid(row=4, column=0, columnspan=3, sticky='ew')

            self.amount_entry = tk.Entry(self.edit_window, text="Amount", textvariable=self.edit_amount_var)
            self.amount_entry.grid(row=4, column=4, columnspan=2, sticky='ew')

            toggle_button_text = "Switch to Expense" if self.is_income.get() else "Switch to Income"
            self.toggle_type_button = tk.Button(self.edit_window, text=toggle_button_text, command=self.edit_switch_type, width=10)
            self.toggle_type_button.grid(row=3, column=3, columnspan=2, sticky='ew')

            self.save_edit_button = tk.Button(self.edit_window, text="Save Changes", command=self.save_edited_entry)
            self.save_edit_button.grid(row=5, column=3, sticky='ew')


        else:
            self.show_error("No item selected for editing")

    def edit_switch_type(self):
        current_type = self.is_income.get()
        self.is_income.set(not current_type)

        self.toggle_type_button.config(text="Switch to Expense" if self.is_income.get() else "Switch to Income")

    def delete_selected(self):
        # Get selected item in treeview
        selected_item = self.treeview.selection()

        confirm = messagebox.askyesno("Confirm", "Do you want to delete selection")
        
        if confirm:

            selected_index = self.treeview.index(selected_item)

            self.data_manager.delete_from_excel(selected_index)

            self.treeview.delete(selected_item)

        self.update_totals()


    def delete_all(self):
        
        # Confirm user wants to delete all
        confirm = messagebox.askyesno("Confirm", "Are you sure you want to start over?")

        if confirm:

            # Clear all data
            self.clear_treeview()
            self.data_manager.clear_excel()

        # Reset totals and balances
        self.update_totals()
            


    def clear_treeview(self):
        for item in self.treeview.get_children():
            self.treeview.delete(item)


    def edit_selected(self):
        pass

    def update_totals(self):

        # Calculate totals and update StringVars
        total_income, total_expenses, balance = self.data_manager.calculate_totals()

        self.total_income_var.set(f"Total Income: ${total_income:,.2f}")
        self.total_expense_var.set(f"Total Expenses: ${total_expenses:,.2f}")
        self.balance_var.set(f"Balance: ${balance:,.2f}")
        

    def show_error(self, message):
        error_box = tk.Toplevel(self.root)
        error_box.title("Error")
        error_box.geometry("500x100")
        tk.Label(error_box, text=message).pack(expand=True)

        error_box.after(1000, error_box.destroy)

    def run(self):
        self.root.mainloop()


if __name__ == "__main__":
    root = tk.Tk()
    app = BudgetApp(root)
    app.run()

        