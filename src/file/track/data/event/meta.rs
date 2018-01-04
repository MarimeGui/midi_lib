use super::super::super::super::super::VLVRead;
use ez_io::ReadE;
use std::io::Read;
use std::error::Error;
use std::result::Result;

#[derive(Clone)]
pub struct SequenceNumber {
    pub sequence_number: u16
}

impl SequenceNumber {
    pub fn read<R: Read>(reader: &mut R) -> Result<SequenceNumber, Box<Error>> {
        let sequence_number: u16 = reader.read_be_to_u16()?;
        Ok(SequenceNumber {
            sequence_number
        })
    }
}

#[derive(Clone)]
pub struct Text {
    pub text: String
}

impl Text {
    pub fn read<R: Read>(reader: &mut R, length: u32) -> Result<Text, Box<Error>> {
        let mut bytes = vec![0; length as usize];
        reader.read_exact(&mut bytes)?;
        let text: String = String::from_utf8_lossy(&bytes).into_owned();
        Ok(Text {
            text
        })
    }
}

#[derive(Clone)]
pub struct MIDIChannelPrefix {
    pub channel: u8
}

impl MIDIChannelPrefix {
    pub fn read<R: Read>(reader: &mut R) -> Result<MIDIChannelPrefix, Box<Error>> {
        let channel: u8 = reader.read_to_u8()?;
        Ok(MIDIChannelPrefix {
            channel
        })
    }
}

#[derive(Clone)]
pub struct MIDIPort {
    pub port: u8
}

impl MIDIPort {
    pub fn read<R: Read>(reader: &mut R) -> Result<MIDIPort, Box<Error>> {
        let port: u8 = reader.read_to_u8()?;
        Ok(MIDIPort {
            port
        })
    }
}

#[derive(Clone)]
pub struct EndOfTrack {}

#[derive(Clone)]
pub struct SetTempo {
    pub tempo: u32
}

impl SetTempo {
    pub fn read<R: Read>(reader: &mut R) -> Result<SetTempo, Box<Error>> {
        let tempo: u32 = u32::from(reader.read_be_to_u16()?) + u32::from(reader.read_to_u8()?);
        Ok(SetTempo {
            tempo
        })
    }
}

#[derive(Clone)]
pub struct SMTPEOffset {
    pub hour: u8,
    pub minute: u8,
    pub seconds: u8,
    pub frames: u8,
    pub hundred_of_frame: u8
}

impl SMTPEOffset {
    pub fn read<R: Read>(reader: &mut R) -> Result<SMTPEOffset, Box<Error>> {
        let hour: u8 = reader.read_to_u8()?;
        let minute: u8 = reader.read_to_u8()?;
        let seconds: u8 = reader.read_to_u8()?;
        let frames: u8 = reader.read_to_u8()?;
        let hundred_of_frame: u8 = reader.read_to_u8()?;
        Ok(SMTPEOffset {
            hour,
            minute,
            seconds,
            frames,
            hundred_of_frame
        })
    }
}

#[derive(Clone)]
pub struct TimeSignature {
    pub nominator: u8,
    pub denominator: u8,  // Expressed as a power of two
    pub midi_ticks_per_metronome_tick: u8,
    pub thing: u8
}

impl TimeSignature {
    pub fn read<R: Read>(reader: &mut R) -> Result<TimeSignature, Box<Error>> {
        let nominator: u8 = reader.read_to_u8()?;
        let denominator: u8 = reader.read_to_u8()?;
        let midi_ticks_per_metronome_tick: u8 = reader.read_to_u8()?;
        let thing: u8 = reader.read_to_u8()?;
        Ok(TimeSignature {
            nominator,
            denominator,
            midi_ticks_per_metronome_tick,
            thing
        })
    }
}

#[derive(Clone)]
pub struct KeySignature {
    pub number_of_sharp_flats: u8,
    pub major_key: bool
}

impl KeySignature {
    pub fn read<R: Read>(reader: &mut R) -> Result<KeySignature, Box<Error>> {
        let number_of_sharp_flats: u8 = reader.read_to_u8()?;
        let major_key: u8 = reader.read_to_u8()?;
        let major_key_bool: bool = major_key == 0;
        Ok(KeySignature {
            number_of_sharp_flats,
            major_key: major_key_bool
        })
    }
}

#[derive(Clone)]
pub struct SequencerSpecific {
    pub length: u32,
    pub id: u32,
    pub data: Vec<u8>
}

impl SequencerSpecific {
    pub fn read<R: Read>(reader: &mut R, total_length: u32) -> Result<SequencerSpecific, Box<Error>> {
        // Read the VLV containing the id
        let vlv_id = reader.read_vlv()?;
        // Get the id
        let id: u32 = vlv_id.data;
        // Get the number of bytes that have been read when reading the id vlv
        let id_length: u8 = vlv_id.real_length;
        // Create the data value
        let mut data: Vec<u8> = vec![0; (total_length-u32::from(id_length)) as usize];
        // Read the rest of the Event
        reader.read_exact(&mut data)?;
        Ok(SequencerSpecific {
            length: total_length,
            id,
            data
        })
    }
}

#[derive(Clone)]
pub struct Unknown {}