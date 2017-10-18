use ez_io::ReadE;
use std::io::Read;
use std::error::Error;
use std::result::Result;
use std::mem::transmute;

#[derive(Clone)]
pub struct NoteChange {
    pub key: u8,
    pub velocity: u8
}

impl NoteChange {
    pub fn read<R: Read>(reader: &mut R, running_status_byte: Option<u8>) -> Result<NoteChange, Box<Error>> {
        let key: u8;
        match running_status_byte {
            Some(x) => key = x,
            None    => key = reader.read_to_u8()?,
        }
        let velocity: u8 = reader.read_to_u8()?;
        Ok(NoteChange{
            key,
            velocity
        })
    }
}

#[derive(Clone)]
pub struct PolyphonicKeyPressure {
    pub key: u8,
    pub pressure: u8
}

impl PolyphonicKeyPressure {
    pub fn read<R: Read>(reader: &mut R, running_status_byte: Option<u8>) -> Result<PolyphonicKeyPressure, Box<Error>> {
        let key: u8;
        match running_status_byte {
            Some(x) => key = x,
            None    => key = reader.read_to_u8()?,
        }
        let pressure: u8 = reader.read_to_u8()?;
        Ok(PolyphonicKeyPressure {
            key,
            pressure
        })
    }
}

#[derive(Clone)]
pub struct ControllerChange {
    pub controller_number: u8,
    pub controller_value: u8
}

impl ControllerChange {
    pub fn read<R: Read>(reader: &mut R, running_status_byte: Option<u8>) -> Result<ControllerChange, Box<Error>> {
        let controller_number: u8;
        match running_status_byte {
            Some(x) => controller_number = x,
            None    => controller_number = reader.read_to_u8()?,
        }
        let controller_value: u8 = reader.read_to_u8()?;
        Ok(ControllerChange {
            controller_number,
            controller_value
        })
    }
}

#[derive(Clone)]
pub struct ProgramChange {
    pub new_program_number: u8,
}

impl ProgramChange {
    pub fn read<R: Read>(reader: &mut R, running_status_byte: Option<u8>) -> Result<ProgramChange, Box<Error>> {
        let new_program_number: u8;
        match running_status_byte {
            Some(x) => new_program_number = x,
            None    => new_program_number = reader.read_to_u8()?,
        }
        Ok(ProgramChange {
            new_program_number
        })
    }
}

#[derive(Clone)]
pub struct ChannelKeyPressure {
    pub value: u8
}

impl ChannelKeyPressure {
    pub fn read<R: Read>(reader: &mut R, running_status_byte: Option<u8>) -> Result<ChannelKeyPressure, Box<Error>> {
        let value: u8;
        match running_status_byte {
            Some(x) => value = x,
            None    => value = reader.read_to_u8()?,
        }
        Ok(ChannelKeyPressure {
            value
        })
    }
}

#[derive(Clone)]
pub struct PitchBend {
    pub value: u16
}

impl PitchBend {
    pub fn read<R: Read>(reader: &mut R, running_status_byte: Option<u8>) -> Result<PitchBend, Box<Error>> {
        let value: u16;
        match running_status_byte {
            Some(x) => value = unsafe {transmute::<[u8; 2], u16>([x, reader.read_to_u8()?])},
            None    => value = reader.read_be_to_u16()?,
        }
        Ok(PitchBend {
            value
        })
    }
}