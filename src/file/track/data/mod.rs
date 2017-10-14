pub mod event;

use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::error::Error;
use super::super::super::VLVRead;
use self::event::Event;

// Represents the combination of a delta_time and an SMFEvent

#[derive(Clone)]
pub struct TrackEvent {
    pub delta_time: u32,
    pub event: Event
}

impl TrackEvent {
    pub fn new<R: Read + Seek>(reader: &mut R, last_event: Option<Event>) -> Result<TrackEvent, Box<Error>> {
        println!("New Track Event @ {}", reader.seek(SeekFrom::Current(0))?);
        let delta_time: u32 = reader.read_vlv()?;
        println!("After Delta Time @ {}", reader.seek(SeekFrom::Current(0))?);
        let event: Event = Event::new(reader, last_event)?;
        Ok(TrackEvent {
            delta_time,
            event
        })
    }
}
