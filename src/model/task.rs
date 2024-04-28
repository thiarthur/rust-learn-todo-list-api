use crate::schema::tasks;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable, Selectable, Insertable, Identifiable, Debug, Clone, Deserialize, Serialize, PartialEq,
)]
#[diesel(table_name = tasks)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask {
    pub title: String,
    pub description: Option<String>,
}
