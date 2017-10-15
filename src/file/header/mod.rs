pub mod data;

use ez_io::ReadE;
use self::data::MidiFormat;
use self::data::MidiDivisionsType;
use self::data::MidiTPQNDivisions;
use self::data::MidiSMTPEDivisions;
use std::error::Error;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::result::Result;
use std::fmt;


#[derive(Debug)]
struct InvalidMidiFormatError;

impl Error for InvalidMidiFormatError {
    fn description(&self) -> &str {
        "Unrecognized Midi Type"
    }
}

impl fmt::Display for InvalidMidiFormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Yea error lol")
    }
}


// Standard Midi File Header Chunk

#[derive(Clone)]
pub struct SMFHeaderChunk {
    pub length: u32,
    pub format: MidiFormat,
    pub nb_tracks: u16,
    pub division_system: MidiDivisionsType
}

impl SMFHeaderChunk {
    pub fn read<R: Read + Seek>(reader: &mut R) -> Result<SMFHeaderChunk, Box<Error>> {
        assert_eq!(String::from("MThd"), reader.read_to_string_n(4)?, "Magic Number did not match");
        let length: u32 = reader.read_be_to_u32()?;
        let format_num: u16 = reader.read_be_to_u16()?;
        let format: MidiFormat;
        match format_num {
            0 => format = MidiFormat::SingleTrack,
            1 => format = MidiFormat::SimultaneousTracks,
            2 => format = MidiFormat::IndependentTracks,
            _ => return Err(Box::new(InvalidMidiFormatError{}))
        }
        let nb_tracks: u16 = reader.read_be_to_u16()?;
        let division_info: u16 = reader.read_be_to_u16()?;
        let division_system;
        if (division_info & 0b1000000000000000u16) == 0 {
            division_system = MidiDivisionsType::TicksPerQuarterNote(
                MidiTPQNDivisions{ ticks_per_quarter_note: division_info });
        } else {
            let ticks_per_smtpe_frame: u16 = division_info & 0b0000000011111111u16;
            let smtpe_frames_per_second: u16 = division_info & 0b0111111100000000u16;
            division_system = MidiDivisionsType::SMTPEFrames(
                MidiSMTPEDivisions{ ticks_per_smtpe_frame, smtpe_frames_per_second }
            )
        }
        // For non-standard headers
        reader.seek(SeekFrom::Current((length as i64) - (6 as i64)))?;
        Ok(SMFHeaderChunk {
            length,
            format,
            nb_tracks,
            division_system
        })
    }
}