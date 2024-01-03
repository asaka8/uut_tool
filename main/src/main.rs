use std::env;
use core::communicator::read_data_buffer;

fn main() {
    cmd();
    // let mut input_string = String::new();
    // println!("Please Input packet type:");
    // io::stdin().read_line(&mut input_string)
    //     .expect("Error reading input");
    // let data_type = input_string.trim();
    // read_data_buffer(data_type);
}

pub fn cmd() {
    let args: Vec<String> = env::args().collect();
    println!("program name: {}", args[0]);
    if args.len() > 1 {
        println!("Other arguments: {:?}", &args[1..]);
        let port_name = args[1].as_str();
        let baudrate: u32;
        let data_type = args[3].as_str();

        match args[2].parse::<u32>() {
            Ok(baud) => {
                baudrate = baud;
            }
            Err(e) => {
                eprintln!("Get baudrate failed: {e}");
                return
            }
        }
        read_data_buffer(port_name, baudrate, data_type);
    }
    else {
        println!("No other arguments");
    }
}