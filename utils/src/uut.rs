use std::fs::{OpenOptions, File};
use std::io::Result;
use std::time::Duration;
use chrono::prelude::*;
use serialport::{new, SerialPortBuilder};

pub fn create_uut(com: String, baud: u32) -> SerialPortBuilder {
    let port: SerialPortBuilder = new(com, baud)
    .timeout(Duration::from_millis(1000));
    port
}

pub fn create_log_file(timestamp: DateTime<Utc>) -> Result<File>{
    let file_path = format!("./imu_{}_data.bin", timestamp.format("%Y%m%d%H%M%S"));
    let file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(file_path)?;
    // file.write_all(&data)?;
    Ok(file)
}