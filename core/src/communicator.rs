use std::io::Write;

use chrono::prelude::*;
use utils::uut::{create_uut, create_log_file};
use crate::data_parse;

pub fn read_data_buffer(port_name: &str, baudrate: u32, data_type: &str) {
    let mut data_buffer = data_buffer_generator(data_type);
    let port_name: String = port_name.to_string();
    let baudrate: u32 = baudrate;
    let timestamp: DateTime<Utc> = Utc::now();
    let mut port = create_uut(port_name, baudrate)
    .open().expect("Port open failed");
    match create_log_file(timestamp) {
        Ok(file) => {
            let mut log_file = file;
            loop {
                match port.read(&mut data_buffer.as_mut_slice()) {
                    Ok(bytes_read) => {
                        if bytes_read > 0 {
                            let data = &data_buffer[..bytes_read];
                            log_file.write_all(&data).unwrap();
                            let latest = data_parse::PacketS3::parse(&data);
                            println!("Received data: {:?}", latest);
                        }
                    }
                    Err(e) => {
                        eprintln!("read_error: {}", e);
                        break;
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

 

pub fn data_buffer_generator(data_type: &str) -> Vec<u8> {
    let data_buffer: Vec<u8>;
    match data_type {
        "S1" => {
            data_buffer = vec![0; 31];
            data_buffer
        },
        "S3" => {
            data_buffer = vec![0; 40];
            data_buffer
        }
        _ => {
            data_buffer = vec![0; 0];
            data_buffer
        }
    }
}