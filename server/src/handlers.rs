use actix_web::{
    error,
    web::{Data, Json, Query},
    HttpResponse, Responder, Result,
};
use chrono::{DateTime, Utc};

use crate::Server;
use crate::{db, model::NewLog};

pub async fn handle_get_logs(
    data: Data<Server>,
    range: Query<api::logs::get::Query>,
) -> Result<impl Responder> {
    let mut conn = data.establish_database_connection();
    let logs = db::logs(&mut conn, range.from, range.until);

    if logs.is_err() {
        return Err(error::ErrorInternalServerError("internal server error"));
    }

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

pub async fn handle_post_logs(
    data: Data<Server>,
    log: Json<api::logs::post::Request>,
) -> Result<impl Responder> {
    let mut conn = data.establish_database_connection();

    let log = NewLog {
        user_agent: log.user_agent.clone(),
        response_time: log.response_time,
        timestamp: log.timestamp.unwrap_or_else(Utc::now).naive_utc(),
    };
    let log = db::insert_log(&mut conn, &log);

    if log.is_err() {
        return Err(error::ErrorInternalServerError("internal server error"));
    }

    Ok(HttpResponse::Accepted().finish())
}
