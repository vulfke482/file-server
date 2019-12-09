extern crate data_encoding;
extern crate ring;

use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::hash::{Hash, Hasher};

pub trait SavedData {
    fn process(&mut self);
}

#[derive(Hash)]
pub struct Header {
    pub name_length : u32,
    pub size: u32,
    pub name:Vec<u8>,
}

pub struct Data {
    pub header:Header,
    pub data:Vec<u8>,
}

impl Data {
    pub fn from_vecu8(packet: Vec<u8>) -> Option<Self> {
        let num_length = vec_to_i32(&packet[0..4].to_vec());
        let size = vec_to_i32(&packet[4..8].to_vec());

        let name = &packet[8..((num_length as usize) + 8)].to_vec();
        let data = &packet[((num_length as usize) + 8)..((num_length as usize) + (size as usize) + 8)].to_vec();

        Some(Data{
            header: Header {
                name_length:num_length,
                size:size,
                name:name.to_vec()
            },
            data:data.to_vec()
        })
    }

    pub fn from_vecu8_ref(packet: &Vec<u8>) -> Option<Self> {
        let num_length = vec_to_i32(&packet[0..4].to_vec());
        let size = vec_to_i32(&packet[4..8].to_vec());

        let name = &packet[8..((num_length as usize) + 8)].to_vec();
        let data = &packet[((num_length as usize) + 8)..((num_length as usize) + (size as usize) + 8)].to_vec();

        Some(Data{
            header: Header {
                name_length:num_length,
                size:size,
                name:name.to_vec()
            },
            data:data.to_vec()
        })
    }
}

impl SavedData for Data {
    fn process(&mut self) {
        println!("I am been processing");
        
    }
}

impl From<Vec<u8>> for Data {
    fn from(packet: Vec<u8>) -> Self {
        Self::from_vecu8(packet).expect("Packet is not valid.")
    }
}

impl From<&Vec<u8>> for Data {
    fn from(packet: &Vec<u8>) -> Self {
        Self::from_vecu8_ref(packet).expect("Packet is not valid.")
    }
}

impl Hash for Data{
    fn hash<H:Hasher>(&self, state:&mut H) {
        self.header.hash(state);
        self.data.hash(state);
    }
}

pub fn vec_to_i32(data : &Vec<u8>) -> u32 {
    ((data[0] as u32) << 24) + ((data[1] as u32) << 16) + ((data[2] as u32) << 8) + (data[3] as u32)
}

pub fn sha256_digest(packet: &Vec<u8>) -> Result<Digest, ()> {
    assert!(packet.len() == 1024);
    let mut context = Context::new(&SHA256);
    context.update(packet);
    Ok(context.finish())
}