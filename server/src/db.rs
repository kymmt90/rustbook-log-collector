use chrono::{DateTime, Utc};
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SqliteConnection};

use crate::model::Log;

pub fn logs(
    conn: &mut SqliteConnection,
    from: Option<DateTime<Utc>>,
    until: Option<DateTime<Utc>>,
) -> QueryResult<Vec<Log>> {
    use crate::schema::logs::dsl::*;

    let mut query = logs.into_boxed();
    if let Some(from) = from {
        query = query.filter(timestamp.ge(from.naive_utc()));
    }
    if let Some(until) = until {
        query = query.filter(timestamp.lt(until.naive_utc()));
    }

    query.order(timestamp.asc()).load(conn)
}
