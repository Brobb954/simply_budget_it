# Simple_Budget - Edit selected branch

This branch is to work on implementing the edit feature of the budget program so that the user doesn't have to completely delete an entry to change it.

Currently the edit selection feature is working superficially, still requires more testing to make sure all parts work as required.

## Overall program progress in this branch.

### Bugs fixed is this commit:

* Format only showing one decimal place instead of two
* Upon reloading any 0s after decimal disappear


### Current bugs:

* Totals do not update in gui after update_totals is called
* Type change does not save


### Current Features:

* Keep data in local excel file to load on program start
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
