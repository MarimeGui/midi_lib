pub mod data;

use ez_io::ReadE;
use self::data::TrackEvent;
use self::data::event::Event;
use std::error::Error;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::result::Result;


// Standard Midi File Track Chunk

#[derive(Clone)]
pub struct SMFTrackChunk {
    pub length: u32,
    pub track_events: Vec<TrackEvent>
}

impl SMFTrackChunk {
    pub fn read<R: Read + Seek>(reader: &mut R) -> Result<SMFTrackChunk, Box<Error>> {
        assert_eq!(String::from("MTrk"), reader.read_to_string_n(4)?, "Magic Number did not match");
        let length: u32 = reader.read_be_to_u32()?;
        // Where does these track events start
        let track_event_start = reader.seek(SeekFrom::Current(0))?;
        // Make the vector that will be responsible for storing all the Track Events
        let mut track_events: Vec<TrackEvent> = Vec::new();
        // Read the First Track Event
        let current_track_event: TrackEvent = TrackEvent::new(reader, None)?;
        // Copy it to the Vector
        track_events.push(current_track_event.clone());
        // Save the Event inside the Track Event we just read for later
        let mut last_event: Event = current_track_event.event;
        // While until we reach the end of the data
        while reader.seek(SeekFrom::Current(0))? < (track_event_start + (length as u64)) {
            // Read a Track Event
            let current_track_event = TrackEvent::new(reader, Some(last_event))?;
            // Keep the Event inside the Track Event we just read for later
            last_event = current_track_event.event.clone();
            // Push the Track Event we just read to the Vector
            track_events.push(current_track_event);
        }
        Ok(SMFTrackChunk {
            length,
            track_events
        })
    }
}