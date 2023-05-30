use axum::http::StatusCode;

pub(crate) async fn root() -> String {
    "
GET  /                  overview
GET  /actuator/live     liveness probe
GET  /actuator/ready    readiness probe
".to_string()
}

pub(crate) async fn actuator_live() -> StatusCode {
    StatusCode::OK
}

pub(crate) async fn actuator_ready() -> StatusCode {
    StatusCode::OK
}
