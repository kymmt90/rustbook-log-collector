use chrono::NaiveDateTime;

use crate::schema::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Insertable)]
#[diesel(table_name = logs)]
pub struct NewLog {
    pub user_agent: String,
    pub response_time: i32,
    pub timestamp: NaiveDateTime,
}
