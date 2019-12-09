#[macro_use]
extern crate clap;

use clap::{Arg, App, SubCommand, ArgMatches};
use data_encoding::HEXUPPER;
use data_encoding;
use std::io::prelude::*;
use std::net::TcpStream;
use std::fs;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let host = "127.0.0.1:7878";

    if let Some(matches) = matches.subcommand_matches("save") {
        save_file(matches, host);
    }

    if let Some(matches) = matches.subcommand_matches("delete") {
        delete_file(matches, host);
    }

    if let Some(matches) = matches.subcommand_matches("get") {
        get_file(matches, host);
    }
}

fn save_file(matches: &ArgMatches, host: &str) {
    if matches.is_present("file") {
        let filename = matches.value_of("file").unwrap();
        let file = fs::read(filename).expect(&format!("Cannot read file {}", filename));
        let name_size = filename.len() as u32;
        let size = file.len() as u32;
        let command = 1;
        let mut stream = TcpStream::connect(host).expect(&format!("Cannot connect to host {}", host));
        
        let name_size_vec = u32_to_vec(name_size);
        let size_vec = u32_to_vec(size);
        let filename_vec = filename.as_bytes().to_vec();

        let data = [vec![0], name_size_vec, size_vec, filename_vec, file].concat();

        stream.write(data.as_ref()).unwrap();
        stream.flush().unwrap();

        let mut res = [0; 1024];
        stream.read(&mut res).unwrap();
        println!("id: {}", HEXUPPER.encode(&res));
    }
}

fn delete_file(matches: &ArgMatches, host: &str) {
    if matches.is_present("hash") {
        let hash = get_bytes_from_hex(matches.value_of("hash").unwrap()).unwrap();
        let request = [vec![2], hash].concat();
        let mut stream = TcpStream::connect(host).expect(&format!("Cannot connect to host {}", host));
        stream.write(&request).unwrap();
        stream.flush().unwrap();

    }
}

fn get_file(matches: &ArgMatches, host: &str) {
    if matches.is_present("hash") {
        let hash = get_bytes_from_hex(matches.value_of("hash").unwrap()).unwrap();
        let request = [vec![2], hash].concat();
        let mut stream = TcpStream::connect(host).expect(&format!("Cannot connect to host {}", host));
        stream.write(&request).unwrap();
        stream.flush().unwrap();

        let mut response =[0; 1024];
        stream.read(&mut response).unwrap();
        println!("{}", std::str::from_utf8(&response).unwrap());
    }
}


fn get_bytes_from_hex(hash: &str) -> Result<Vec<u8>, data_encoding::DecodeError> {
    HEXUPPER.decode(hash.as_bytes())
}

fn u32_to_vec(num: u32) -> Vec<u8> {
    let mask = (1 << 8) - 1;
    vec![
        (num & (mask << 24)) as u8,
        (num & (mask << 16)) as u8,
        (num & (mask << 8)) as u8,
        (num & mask) as u8,
    ]
}