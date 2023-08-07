use actix_web::{
    web::{Data, Json, Query},
    Responder, Result,
};
use chrono::{DateTime, Utc};
use log::debug;

use crate::db;
use crate::Server;

pub async fn handle_get_logs(
    data: Data<Server>,
    range: Query<api::logs::get::Query>,
) -> Result<impl Responder> {
    let mut conn = data.establish_database_connection();
    let logs = db::logs(&mut conn, range.from, range.until).or_else(|err| return Err(err));
    let logs = logs
        .unwrap()
        .into_iter()
        .map(|log| api::Log {
            user_agent: log.user_agent,
            response_time: log.response_time,
            timestamp: DateTime::from_utc(log.timestamp, Utc),
        })
        .collect();

    Ok(Json(api::logs::get::Response(logs)))
}
