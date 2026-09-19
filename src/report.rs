use std::{
    fs, io,
    path::PathBuf,
};

use chrono::{DateTime, NaiveDate, Local};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum EventTag {
    Move,
    Class,
    Study,
    Work,
    Meeting,
    Life,
    Activity,
    Other(String),
}

#[derive(Debug, Deserialize)]
pub enum Status {
    Scheduled,
    Canceled,
    Ongoing,
    Done,
}


#[derive(Debug, Deserialize)]
pub struct Event {
    pub title: Option<String>,
    pub description: Option<String>,
    pub tag: Option<EventTag>,
    pub status: Option<Status>,
    pub at: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub start_at: Option<DateTime<Local>>,
    pub end_at: Option<DateTime<Local>>,
}

#[derive(Debug, Deserialize)]
pub struct Report {
    pub date: Option<NaiveDate>,
    pub updatedate: Option<DateTime<Local>>,
    pub start: Option<String>,
    pub finish: Option<String>,
    pub text: Option<String>,
    pub events: Vec<Event>
}

impl Report {
    pub fn view_model(self) -> ReportViewModel {
        ReportViewModel {
            date: self.date
                .map(|date| date.to_string())
                .unwrap_or_else(|| "(none)".to_string()),
            updatedate: self.updatedate
                .map(|datetime| datetime.to_string())
                .unwrap_or_else(|| "(none)".to_string()),
            start: self.start.unwrap_or_else(|| "(none)".to_string()),
            finish: self.finish.unwrap_or_else(|| "(none)".to_string()),
            text: self.text.unwrap_or_else(|| "(none)".to_string()),
        }
    }
}

pub struct ReportViewModel {
    pub date: String,
    pub updatedate: String,
    pub start: String,
    pub finish: String,
    pub text: String,
}

#[derive(Debug)]
pub struct ReportRepository {
    dir: PathBuf,
}

impl ReportRepository {
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self { dir: dir.into() }
    }

    pub fn load(&self, date: &str) -> io::Result<Option<Report>> {
        if !is_valid_date(date) {return Ok(None);}

        let path = self.dir.join(format!("{date}.toml"));

        if !path.exists() {
            return Ok(None);
        }

        let report_raw = fs::read_to_string(&path)?;
        let report: Report = toml::from_str(&report_raw)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;

        Ok(Some(report))
    }

    pub fn latest(&self) -> io::Result<Option<String>> {
        let mut latest_date: Option<String> = None;

        for i in fs::read_dir(&self.dir)? {
            let entry = i?;
            let path = entry.path();

            if !path.is_file() {
                continue;
            }
            if path.extension().and_then(|ext| ext.to_str()) != Some("toml") {
                continue;
            }

            let Some(stem) = path.file_stem() else {
                continue;
            };

            let date = stem.to_string_lossy().to_string();

            if latest_date.as_ref().is_none_or(|cur| date > *cur) {
                latest_date = Some(date);
            }
        }

        Ok(latest_date)
    }
}

pub fn is_valid_date(date: &str) -> bool {
    let date = match NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => return false,
    };

    date <= Local::now().date_naive()
}
