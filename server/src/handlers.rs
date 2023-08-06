use actix_web::{
    web::{Json, Query},
    Responder, Result,
};
use api::Log;
use log::debug;

pub async fn handle_get_logs(range: Query<api::logs::get::Query>) -> Result<impl Responder> {
    debug!("{:?}", range);

    let logs: Vec<Log> = Default::default();

    Ok(Json(api::logs::get::Response(logs)))
}
