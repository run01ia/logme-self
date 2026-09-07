use askama::Template;
use axum::{extract::{Path, State}, response::{Response}};
use crate::{report::Report, AppState};

#[derive(Template)]
#[template(path = "daily.html")]
struct DailyTemplate {
    report: Option<Report>
}

pub async fn daily_report(
    State(state): State<AppState>,
    Path(date): Path<String>
) -> Response {
    todo!()
}

pub async fn latest_report(
    State(state): State<AppState>
) -> Response {
    todo!()
}
