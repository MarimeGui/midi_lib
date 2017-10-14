pub mod header;
pub mod track;

use self::header::SMFHeaderChunk;
use self::track::SMFTrackChunk;
use std::error::Error;
use std::io::Read;
use std::io::Seek;
use std::result::Result;

// Represents the Standard Midi File

#[derive(Clone)]
pub struct SMF {
    pub header: SMFHeaderChunk,
    pub tracks: Vec<SMFTrackChunk>
}

impl SMF {
    // Function for creating an SMF structure
    pub fn read<R: Read + Seek>(reader: &mut R) -> Result<SMF, Box<Error>> {
        println!("New file");
        let header: SMFHeaderChunk = SMFHeaderChunk::read(reader)?;
        let mut tracks: Vec<SMFTrackChunk> = Vec::with_capacity(header.nb_tracks as usize);
        for _ in 0..header.nb_tracks {
            tracks.push(SMFTrackChunk::read(reader)?);
        }
        Ok(SMF {
            header,
            tracks
        })
    }
}