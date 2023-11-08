# Simple_Budget - Data Memory
This branch is to work on changing the program over from storing every change into the excel immediately into working with memory data and then only storing upon program exit.


## Overall program progress in this branch.

Instituted a context manager to properly handle the opening and closing of the excel document as well as a method to save the data in memory to the excel document automatically upon program exit.

### Bugs fixed is this commit:

### Current bugs:

### Current Features:

    * Keep data in local excel file to load on program start
    * Work within memory until program is closed then save to excel
    * Delete a selected item
    * Delete all items in budget
    * Edit selected item
    * Running totals of Income, Expenses and Balance
    * Treeview front end view
    * Error box messages

### Plans for future:

    * Multiple budget management
    * SQL integration
    * Graphs to show money spent by category
