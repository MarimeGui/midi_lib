use std::io::Read;
use std::error::Error;
use super::super::super::super::super::VLVRead;

#[derive(Clone)]
pub struct Sysex {
    pub length: u32,
    pub data: Vec<u8>
}

impl Sysex {
    pub fn read<R: Read>(reader: &mut R) -> Result<Sysex, Box<Error>> {
        let length: u32 = reader.read_vlv()?.data;
        let mut data: Vec<u8> = vec![0; length as usize];
        reader.read_exact(&mut data)?;
        Ok(Sysex {
            length,
            data
        })
    }
}