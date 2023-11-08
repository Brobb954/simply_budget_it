import re
from openpyxl import Workbook, load_workbook
import os

class DataManager:
    def __init__(self, filename="budget_data.xlsx"):
        self.filename = filename
        self.workbook = None
        self.sheet = None
        self.in_memory_data = [] # In-memory storage for data
        self.load_or_create_workbook()

    def load_or_create_workbook(self):
        if os.path.exists(self.filename):
            self.workbook = load_workbook(self.filename)
            self.sheet = self.workbook.active
            self.load_data()
        else:
            self.workbook = Workbook()
            self.sheet = self.workbook.active
            self.sheet["A1"] = "Type"
            self.sheet["B1"] = "Description"
            self.sheet["C1"] = "Amount"
            self.workbook.save(self.filename)

    def load_data(self):
        self.in_memory_data = []
        for row in self.sheet.iter_rows(min_row=2, values_only=True):
            if row[0] is not None:
                self.in_memory_data.append(row)


    def save_data_to_excel(self):

        # Clear existing data if data exists
        if self.sheet.max_rows > 1:
            self.sheet.delete_rows(2, self.sheet.max_row - 1)

        # Write new data from memory
        for entry in self.in_memory_data:
            self.sheet.append(entry)

        # Save workbook
        self.workbook.save(self.filename)

    def add_and_process_data(self, entry_type, description, amount_str):
        formatted_entry = self.format_entry(entry_type, description, amount_str)

        if formatted_entry:
            self.in_memory_data(formatted_entry)

            return formatted_entry
        else:
            return None
        
    

    def delete_data(self, index):
        try:
            if index < len(self.in_memory_data):
                del self.in_memory_data[index]
            else:
                return "Error: No entry exists at the specified index"
            
        except Exception as e:
            return str(e)

    def clear_data(self):
        if len(self.in_memory_data) > 0:
           self.in_memory_data.clear()
           return "All entries have been cleared"
        else:
           return "There are no entries to delete"


    def update_data_entry(self, index, new_values):

        if 0 <= index < len(self.in_memory_data):
            self.in_memory_data[index] = new_values
            return "Entry updated successfully"
        else:
            return "Invalid entry index. Entry not found"
        
    def get_select_entry(self, index):
        if 0 <= index < len(self.in_memory_data):
            return self.in_memory_data[index]
        else:
            return None
        

    def calculate_totals(self):
        # Initialize Totals
        total_income = 0.00
        total_expense = 0.00

        # Calculate totals from treeview data
        for entry in self.in_memory_data:
            if entry[0] == "Income":
                total_income+= float(entry[2].replace('$',''))
            else:
                total_expense += float(entry[2].replace('$',''))

        balance = total_income - total_expense
        return total_expense, total_income, balance
    
    def is_valid_amount(self, amount_str):
        pattern = r'\d+(\.\d{1,2})?$'
        return re.fullmatch(pattern, amount_str) is not None
    
    def format_entry(self, entry_type, description, amount_str):
        if self.is_valid_amount(amount_str):
            amount_float = float(amount_str)
            formatted_amount = f"${amount_float:,.2f}"
            return(entry_type, description, formatted_amount)
        else:
            return None

        