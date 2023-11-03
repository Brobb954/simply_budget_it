import re
from openpyxl import Workbook, load_workbook
import os

class DataManager:
    def __init__(self, filename="budget_data.xlsx"):
        self.filename = filename
        self.workbook = None
        self.sheet = None
        self.load_or_create_workbook()

    def load_or_create_workbook(self):
        if os.path.exists(self.filename):
            self.workbook = load_workbook(self.filename)
        else:
            self.workbook = Workbook()
            self.sheet = self.workbook.active
            self.sheet["A1"] = "Type"
            self.sheet["B1"] = "Description"
            self.sheet["C1"] = "Amount"
            self.workbook.save(self.filename)
        self.sheet = self.workbook.active
        # Once workbook is set up, load data into app
        self.load_data_from_excel()
    
    def add_data_to_excel(self, entry):
        self.sheet.append(entry)
        self.workbook.save(self.filename)

    def load_data_from_excel(self):
        data = []
        if self.workbook:
            for row in self.sheet.iter_rows(values_only=True, min_row=2):
                data.append(row)
        return data


    def delete_from_excel(self, treeview_index):

        # Open workbook and select active sheet
        workbook = load_workbook(self.filename)
        sheet = workbook.active

        # Delete row
        sheet.delete_rows(treeview_index + 2)

        # Save the workbook
        workbook.save(self.filename)

    def clear_excel(self):
        workbook = load_workbook(self.filename)
        sheet = workbook.active

        num_rows = sheet.max_row

        if num_rows > 1:
            sheet.delete_rows(2, num_rows-1)

        workbook.save(self.filename)

    

    def calculate_totals(self):
        # Initialize Totals
        total_income = 0.0
        total_expense = 0.0

        # Calculate totals from treeview data
        for entry in self.load_data_from_excel():
            if entry[0] == "Income":
                total_income+= float(entry[2])
            else:
                total_expense += float(entry[2])

        balance = total_income - total_expense
        return total_expense, total_income, balance
    
    def is_valid_amount(self, amount_str):
        return re.fullmatch(r'\d+(\.\d{1,2})?$', amount_str) is not None
        