use std::{u16, i16};

pub struct _PacketS1 {
    pub x_accel: i16,
    pub y_accel: i16,
    pub z_accel: i16,
    pub x_rate: i16,
    pub y_rate: i16,
    pub z_rate: i16,
    pub x_rate_temp: i16,
    pub y_rate_temp: i16,
    pub z_rate_temp: i16,
    pub board_temp: i16,
    pub timer: u16,
    pub bit_status: u16,
} // Packet structure: header(2 bytes) + packet type(2 bytes) + length(1 bytes) + payload({length}bytes) + crc(2 bytes)

pub struct _PacketS3 {
    pub num: u32,
    pub x_rate_counts: i32,
    pub y_rate_counts: i32,
    pub z_rate_counts: i32,
    pub x_accel_counts: i32,
    pub y_accel_counts: i32,
    pub z_accel_counts: i32,
    pub board_temp_counts: i16,
    pub supplierid: u16,
    pub productid: u32,
} // Packet structure: header(0 bytes) + packet type(3 bytes) + length(0 bytes) + payload({length}bytes) + crc(1 bytes)

#[derive(Debug)]
pub struct PacketS1 {
    pub x_accel: f64,
    pub y_accel: f64,
    pub z_accel: f64,
    pub x_rate: f64,
    pub y_rate: f64,
    pub z_rate: f64,
    pub x_rate_temp: f64,
    pub y_rate_temp: f64,
    pub z_rate_temp: f64,
    pub board_temp: f64,
    pub timer: f64,
    pub bit_status: u16,
} // Packet Length: 31

#[derive(Debug)]
pub struct PacketS3 {
    pub num: u32,
    pub x_rate_counts: f64,
    pub y_rate_counts: f64,
    pub z_rate_counts: f64,
    pub x_accel_counts: f64,
    pub y_accel_counts: f64,
    pub z_accel_counts: f64,
    pub board_temp_counts: f64,
    pub supplierid: u16,
    pub productid: u32,
} // Packet Length: 40


impl PacketS1 {
    // MSB
    pub fn parse(_data_buffer: &[u8]) -> Option<PacketS1> {
        if _data_buffer.len() >= 31 {
            let x_accel  = (i16::from_be_bytes([_data_buffer[5], _data_buffer[6]]) as f64) * 20.0 / (2u32.pow(16) as f64);
            let y_accel  = (i16::from_be_bytes([_data_buffer[7], _data_buffer[8]]) as f64) * 20.0 / (2u32.pow(16) as f64);
            let z_accel  = (i16::from_be_bytes([_data_buffer[9], _data_buffer[10]]) as f64) * 20.0 / (2u32.pow(16) as f64);
            let x_rate = (i16::from_be_bytes([_data_buffer[11], _data_buffer[12]]) as f64) * 1260.0 / (2u32.pow(16) as f64);
            let y_rate = (i16::from_be_bytes([_data_buffer[13], _data_buffer[14]]) as f64) * 1260.0 / (2u32.pow(16) as f64);
            let z_rate = (i16::from_be_bytes([_data_buffer[15], _data_buffer[16]]) as f64) * 1260.0 / (2u32.pow(16) as f64);
            let x_rate_temp = (i16::from_be_bytes([_data_buffer[17], _data_buffer[18]]) as f64) * 200.0 / (2u32.pow(16) as f64);
            let y_rate_temp = (i16::from_be_bytes([_data_buffer[19], _data_buffer[20]]) as f64) * 200.0 / (2u32.pow(16) as f64);
            let z_rate_temp = (i16::from_be_bytes([_data_buffer[21], _data_buffer[22]]) as f64) * 200.0 / (2u32.pow(16) as f64);
            let board_temp = (i16::from_be_bytes([_data_buffer[23], _data_buffer[24]]) as f64) * 200.0 / (2u32.pow(16) as f64);
            let timer = (u16::from_be_bytes([_data_buffer[25], _data_buffer[26]]) as f64) * 15.259022;
            let bit_status: u16 = u16::from_be_bytes([_data_buffer[27], _data_buffer[28]]);

  
            Some(PacketS1 {
                x_accel,
                y_accel,
                z_accel,
                x_rate,
                y_rate,
                z_rate,
                x_rate_temp,
                y_rate_temp,
                z_rate_temp,
                board_temp,
                timer,
                bit_status
            })
        }
        else {
            None
        }
    }
}

impl PacketS3 {
    // LSB
    pub fn parse(_data_buffer: &[u8]) -> Option<PacketS3> {
        if _data_buffer.len() >= 40 {
            let num = u32::from_be_bytes([_data_buffer[6], _data_buffer[5], _data_buffer[4], _data_buffer[3]]);
            let x_rate_counts = (i32::from_be_bytes([_data_buffer[10], _data_buffer[9], _data_buffer[8], _data_buffer[7]]) as f64) * 0.0000001;
            let y_rate_counts = (i32::from_be_bytes([_data_buffer[14], _data_buffer[13], _data_buffer[12], _data_buffer[11]]) as f64) * 0.0000001;
            let z_rate_counts = (i32::from_be_bytes([_data_buffer[18], _data_buffer[17], _data_buffer[16], _data_buffer[15]]) as f64) * 0.0000001;
            let x_accel_counts = (i32::from_be_bytes([_data_buffer[22], _data_buffer[21], _data_buffer[20], _data_buffer[19]]) as f64) * 0.0000001;
            let y_accel_counts = (i32::from_be_bytes([_data_buffer[26], _data_buffer[25], _data_buffer[24], _data_buffer[23]]) as f64) * 0.0000001;
            let z_accel_counts = (i32::from_be_bytes([_data_buffer[30], _data_buffer[29], _data_buffer[28], _data_buffer[27]]) as f64) * 0.0000001;
            let board_temp_counts = (i16::from_be_bytes([_data_buffer[32], _data_buffer[31]]) as f64) * 0.008;
            let supplierid = u16::from_be_bytes([_data_buffer[34], _data_buffer[33]]);
            let productid = u32::from_be_bytes([_data_buffer[38], _data_buffer[37], _data_buffer[36], _data_buffer[35]]);
            Some(PacketS3 {
                num,
                x_rate_counts,
                y_rate_counts,
                z_rate_counts,
                x_accel_counts,
                y_accel_counts,
                z_accel_counts,
                board_temp_counts,
                supplierid,
                productid
            })
        }
        else {
            None
        }
    } 
}