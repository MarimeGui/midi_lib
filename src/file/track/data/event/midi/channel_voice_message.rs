use ez_io::ReadE;
use std::io::Read;
use std::error::Error;
use std::result::Result;

#[derive(Clone)]
pub struct NoteChange {
    pub channel: u8,
    pub key: u8,
    pub velocity: u8
}

impl NoteChange {
    pub fn read<R: Read>(reader: &mut R, channel: u8) -> Result<NoteChange, Box<Error>> {
        let key: u8 = reader.read_to_u8()?;
        let velocity: u8 = reader.read_to_u8()?;
        Ok(NoteChange{
            channel,
            key,
            velocity
        })
    }
}

#[derive(Clone)]
pub struct PolyphonicKeyPressure {
    pub channel: u8,
    pub key: u8,
    pub pressure: u8
}

impl PolyphonicKeyPressure {
    pub fn read<R: Read>(reader: &mut R, channel: u8) -> Result<PolyphonicKeyPressure, Box<Error>> {
        let key: u8 = reader.read_to_u8()?;
        let pressure: u8 = reader.read_to_u8()?;
        Ok(PolyphonicKeyPressure {
            channel,
            key,
            pressure
        })
    }
}

#[derive(Clone)]
pub struct ControllerChange {
    pub channel: u8,
    pub controller_number: u8,
    pub controller_value: u8
}

impl ControllerChange {
    pub fn read<R: Read>(reader: &mut R, channel: u8) -> Result<ControllerChange, Box<Error>> {
        let controller_number: u8 = reader.read_to_u8()?;
        let controller_value: u8 = reader.read_to_u8()?;
        Ok(ControllerChange {
            channel,
            controller_number,
            controller_value
        })
    }
}

#[derive(Clone)]
pub struct ProgramChange {
    pub channel: u8,
    pub new_program_number: u8,
}

impl ProgramChange {
    pub fn read<R: Read>(reader: &mut R, channel: u8) -> Result<ProgramChange, Box<Error>> {
        let new_program_number: u8 = reader.read_to_u8()?;
        Ok(ProgramChange {
            channel,
            new_program_number
        })
    }
}

#[derive(Clone)]
pub struct ChannelKeyPressure {
    pub channel: u8,
    pub value: u8
}

impl ChannelKeyPressure {
    pub fn read<R: Read>(reader: &mut R, channel: u8) -> Result<ChannelKeyPressure, Box<Error>> {
        let value: u8 = reader.read_to_u8()?;
        Ok(ChannelKeyPressure {
            channel,
            value
        })
    }
}

#[derive(Clone)]
pub struct PitchBend {
    pub channel: u8,
    pub value: u16
}

impl PitchBend {
    pub fn read<R: Read>(reader: &mut R, channel: u8) -> Result<PitchBend, Box<Error>> {
        let value: u16 = reader.read_be_to_u16()?;
        Ok(PitchBend {
            channel,
            value
        })
    }
}