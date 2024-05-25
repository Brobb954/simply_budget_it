use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::schema::budgets;
use crate::domain::users::User;

#[derive(Queryable, Identifiable, Associations, Selectable, Debug, PartialEq)]
#[diesel(belongs_to(User))]
#[diesel(table_name = budgets)]
pub struct Budget {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub user_id: i32,
}

#[derive(Insertable, Associations)]
#[diesel(belongs_to(User))]
#[diesel(table_name = budgets)]
pub struct NewBudget<'a> {
    pub name: &'a String,
    pub description: Option<&'a String>,
    pub created_at: Option<&'a NaiveDateTime>,
    pub user_id: &'a i32,
}


