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
pub struct TextEvent {
    pub length: u32,
    pub text: String
}

impl TextEvent {
    pub fn read<R: Read>(reader: &mut R) -> Result<TextEvent, Box<Error>> {
        let length: u32 = reader.read_vlv()?.0;
        let mut bytes = vec![0; length as usize];
        reader.read_exact(&mut bytes)?;
        let text: String = String::from_utf8_lossy(&bytes).into_owned();
        Ok(TextEvent {
            length,
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
pub struct EndOfTrack {}

#[derive(Clone)]
pub struct SetTempo {
    pub tempo: u32
}

impl SetTempo {
    pub fn read<R: Read>(reader: &mut R) -> Result<SetTempo, Box<Error>> {
        let tempo: u32 = reader.read_be_to_u16()? as u32 + reader.read_to_u8()? as u32;
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
        let major_key_bool: bool;
        if major_key == 0 {
            major_key_bool = true;
        } else {
            major_key_bool = false;
        }
        Ok(KeySignature {
            number_of_sharp_flats,
            major_key: major_key_bool
        })
    }
}

#[derive(Clone)]
pub struct SequencerSpecificMetaEvent {
    pub length: u32,
    pub id: u32,
    pub data: u32
}

impl SequencerSpecificMetaEvent {
    pub fn read<R: Read>(reader: &mut R) -> Result<SequencerSpecificMetaEvent, Box<Error>> {
        // Read the total length of the rest of the Event
        let total_length: u32 = reader.read_vlv()?.0;
        // Read the VLV containing the id
        let vlv_id = reader.read_vlv()?;
        // Get the id
        let id: u32 = vlv_id.0;
        // Get the number of bytes that have been read when reading the id vlv
        let id_length: u8 = vlv_id.1;
        // Create the data value
        let mut data: u32 = 0;
        // Read the rest of the Event
        for _ in 0..(total_length-(id_length as u32)) {
            data += reader.read_to_u8()? as u32;
        }
        Ok(SequencerSpecificMetaEvent {
            length: total_length,
            id,
            data
        })
    }
}