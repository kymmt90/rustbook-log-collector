use std::io::BufReader;

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{
    error,
    web::{Data, Json, Query},
    HttpResponse, Responder, Result,
};
use chrono::{DateTime, Utc};
use itertools::Itertools;

use crate::{db, model::NewLog};
use crate::{db::insert_logs, Server};

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

#[derive(MultipartForm)]
pub struct UploadedCsv {
    file: TempFile,
}

pub async fn handle_post_csv(
    data: Data<Server>,
    form: MultipartForm<UploadedCsv>,
) -> Result<impl Responder> {
    const MAX_FILE_SIZE: u64 = 1024 * 1024 * 10; // 10 MB

    match form.file.size {
        0 => return Ok(HttpResponse::BadRequest().finish()),
        length if length > MAX_FILE_SIZE.try_into().unwrap() => {
            return Ok(HttpResponse::BadRequest().finish())
        }
        _ => {}
    }

    let mut conn = data.establish_database_connection();
    let buf = BufReader::new(form.file.file.as_file());
    let in_logs = csv::Reader::from_reader(buf).into_deserialize::<::api::Log>();

    for logs in &in_logs.chunks(1000) {
        let logs = logs
            .filter_map(Result::ok)
            .map(|log| NewLog {
                user_agent: log.user_agent,
                response_time: log.response_time,
                timestamp: log.timestamp.naive_utc(),
            })
            .collect_vec();

        let result = insert_logs(&mut conn, &logs);

        if result.is_err() {
            return Err(error::ErrorInternalServerError("internal server error"));
        }
    }

    Ok(HttpResponse::Ok().finish())
}
