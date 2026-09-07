use std::{
    fs, io,
    path::{Path, PathBuf},
};

use serde::Deserialize;

use crate::report;

#[derive(Debug, Deserialize)]
pub struct Report {
    pub date: Option<String>,
    pub updatedate: Option<String>,
    pub summary: Option<Summary>,
    pub note: Option<Note>,
}

#[derive(Debug, Deserialize)]
pub struct Summary {
    pub start: Option<String>,
    pub finish: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Note {
    pub text: Option<String>,
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
