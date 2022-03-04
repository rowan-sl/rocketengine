use std::{path::PathBuf, fs::File};

use csv::{Writer, WriterBuilder};

#[derive(Debug, serde::Serialize)]
pub struct CSVRow {
    pub time: f64,
    pub pos_x: f64,
    pub pos_y: f64,
    pub pos_z: f64,
    pub accel_x: f64,
    pub accel_y: f64,
    pub accel_z: f64,
    pub accel_ng_x: f64,
    pub accel_ng_y: f64,
    pub accel_ng_z: f64,
    pub vel_x: f64,
    pub vel_y: f64,
    pub vel_z: f64,
    pub mass: f64,
}

#[derive(Debug)]
pub struct Logger {
    writer: Writer<File>
}

impl Logger {
    pub fn open_file(file: PathBuf) -> Result<Self, std::io::Error> {
        Ok(Self {
            writer: WriterBuilder::new().has_headers(true).from_path(file)?
        })
    }

    pub fn write_record(&mut self, record: CSVRow) -> Result<(), csv::Error> {
        self.writer.serialize(record)?;
        Ok(())
    }
}