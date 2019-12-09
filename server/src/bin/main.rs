use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

use std::thread;
use std::time::Duration;

use rocksdb;
use tolyaproject::Data;
use tolyaproject::sha256_digest;

use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use std::sync::Arc;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let path = "storage/";
    let db = Arc::new(rocksdb::DB::open_default(path).unwrap());

    let pool = ThreadPool::new(6);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let db = Arc::clone(&db);
        pool.execute( move || {
            handle_connection(db, stream);  
        });
    }   
}

use std::fs;

fn handle_connection(db : Arc<rocksdb::DB>, mut stream : TcpStream) {
    let mut buffer = [0;1025];
    stream.read(&mut buffer).expect("cannot read buffer");

    match buffer[0] {
        0 => handle_save_request(db, stream, &buffer[1..].to_vec()),
        1 => handle_delete_request(db, stream, &buffer[1..33].to_vec()),
        2 => handle_get_request(db, stream, &buffer[1..33].to_vec()),
        _ => println!("Unknown command.")
    }

}

fn handle_save_request(db : Arc<rocksdb::DB>, mut stream : TcpStream, packet : &Vec<u8>) {
    println!("I am handling save request");
    if let Some(_data) = Data::from_vecu8_ref(packet) {
        let hash = sha256_digest(packet).ok().unwrap();
        if let Ok(_) = db.put(hash.as_ref(), packet) {
            stream.write(hash.as_ref()).expect("Cannot write to stream.");
            stream.flush().unwrap();
        }
    }
}

fn handle_delete_request(db : Arc<rocksdb::DB>, mut _stream : TcpStream, packet : &Vec<u8>) {
    println!("I am handling delete request");
    match db.delete(packet) {
        Ok(_) => println!("File was deleted successfully."),
        Err(error) => println!("Cannot delete file, reasoning: {}", error)
    }
}

fn handle_get_request(db : Arc<rocksdb::DB>, mut stream : TcpStream, packet : &Vec<u8>) {
    println!("I am handling get request");
    match db.get(packet) {
        Ok(data) => {
            if let Some(data) = data {
                stream.write(&data.to_vec()).expect("Cannot write to stream.");
                stream.flush().unwrap();
            }
        },
        Err(error) => println!("Cannot get file, reasoning: {}", error)
    }
}