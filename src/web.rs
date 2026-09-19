use crate::{AppState, report::{ReportViewModel, is_valid_date}};
use askama::Template;
use axum::{
    extract::{Path, State}, http::StatusCode, response::{Html, IntoResponse, Redirect, Response},
};

#[derive(Template)]
#[template(path = "daily.html")]
struct DailyTemplate {
    report: Option<ReportViewModel>,
}

pub async fn daily_report(State(state): State<AppState>, Path(date): Path<String>) -> Response {
    if !is_valid_date(&date) {
        return (StatusCode::BAD_REQUEST, "Invalid date").into_response();
    }

    match state.reports.load(&date) {
        Ok(Some(report)) => Html(
            DailyTemplate {
                report: Some(report.view_model()),
            }
            .render()
            .unwrap(),
        )
            .into_response(),

        Ok(None) => (
            StatusCode::NOT_FOUND,
            Html(
                DailyTemplate {report: None}
                    .render()
                    .unwrap(),
            ),
        )
            .into_response(),

        Err(e) => {
            tracing::error!("Failed to load {date}.toml file: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error: faile to load files",
            )
                .into_response()
        }
    }
}

pub async fn latest_report(State(state): State<AppState>) -> Response {
    match state.reports.latest() {
        Ok(Some(date)) => {
            Redirect::temporary(&format!("/reports/daily/{date}")).into_response()
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Html(
                DailyTemplate {report: None}
                    .render()
                    .unwrap(),
            ),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("failed to find latest report: {e}");

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error: faild to find a file",
            )
                .into_response()
        }
    }
}
