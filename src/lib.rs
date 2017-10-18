extern crate ez_io;

use ez_io::ReadE;
use std::error::Error;
use std::io::Read;
use std::result::Result;
use std::fmt;

#[derive(Debug)]
pub struct VLVTooBigError;

impl Error for VLVTooBigError {
    fn description(&self) -> &str {
        "Trying to read a VLV bigger than 4 bytes"
    }
}

impl fmt::Display for VLVTooBigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Yea error lol")
    }
}

pub struct VLV {
    pub real_length: u8,
    pub data: u32
}

// Makes it easy to read VLVs
pub trait VLVRead: Read {
    fn read_vlv(&mut self) -> Result<VLV, Box<Error>> {
        let mut out: u32 = 0u32;
        let mut counter: u8 = 0;
        loop {
            let current = self.read_to_u8()?;
            out += current as u32 & 0b01111111u32;
            if current & 0b10000000u8 == 0 {
                break;
            }
            if counter >= 4 {
                return Err(Box::new(VLVTooBigError{}))
            }
            counter += 1;
        }
        Ok(VLV {
            real_length: counter + 1,
            data: out
        })
    }
}

// Implement the VLVRead trait to anything that has the Read trait
impl<R: Read + ?Sized> VLVRead for R {}

pub mod file;