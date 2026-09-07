use std::{io, path::{Path, PathBuf}};

use serde::Deserialize;


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
        todo!()
    }

    pub fn latest(&self) -> io::Result<Option<String>> {
        todo!();
    }
}
