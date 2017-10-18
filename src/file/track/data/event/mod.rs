pub mod midi;
pub mod sysex;
pub mod meta;

use super::super::super::super::VLVRead;
use ez_io::ReadE;
use self::midi::*;
use self::sysex::*;
use self::meta::*;
use std::error::Error;
use std::fmt;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::result::Result;

#[derive(Debug)]
pub struct NoPreviousEvent;

impl Error for NoPreviousEvent {
    fn description(&self) -> &str {
        "Tried to do a Running Status Event without any previous event"
    }
}

impl fmt::Display for NoPreviousEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Yea error lol")
    }
}

#[derive(Debug)]
pub struct UnknownEventError;

impl Error for UnknownEventError {
    fn description(&self) -> &str {
        "Unknown Event"
    }
}

impl fmt::Display for UnknownEventError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Yea error lol")
    }
}

#[derive(Clone)]
pub enum MidiEventType {
    NoteOff(NoteChange),
    NoteOn(NoteChange),
    PolyphonicKeyPressure(PolyphonicKeyPressure),
    ControllerChange(ControllerChange),
    ProgramChange(ProgramChange),
    ChannelKeyPressure(ChannelKeyPressure),
    PitchBend(PitchBend)
}

#[derive(Clone)]
pub struct MidiEvent {
    pub code_byte: u8,
    pub channel: u8,
    pub event: MidiEventType
}

impl MidiEvent {
    pub fn read<R: Read>(reader: &mut R, code_byte: u8, running_status_byte: Option<u8>) -> Result<MidiEvent, Box<Error>> {
        let code_byte: u8 = code_byte & 0xF0u8;
        let channel: u8 = code_byte & 0x0Fu8;
        if code_byte == 0x80u8 {
            // Note Off
            Ok(MidiEvent {
                code_byte,
                channel,
                event: MidiEventType::NoteOff(NoteChange::read(reader, running_status_byte)?)
            })
        } else if code_byte == 0x90u8 {
            // Note On
            Ok(MidiEvent {
                code_byte,
                channel,
                event: MidiEventType::NoteOn(NoteChange::read(reader, running_status_byte)?)
            })
        } else if code_byte == 0xA0u8 {
            // Polyphonic Key Pressure
            Ok(MidiEvent {
                code_byte,
                channel,
                event: MidiEventType::NoteOn(NoteChange::read(reader, running_status_byte)?)
            })
        } else if code_byte == 0xB0u8 {
            // Controller Change
            Ok(MidiEvent {
                code_byte,
                channel,
                event: MidiEventType::ControllerChange(ControllerChange::read(reader, running_status_byte)?)
            })
        } else if code_byte == 0xC0u8 {
            // Program Change
            Ok(MidiEvent {
                code_byte,
                channel,
                event: MidiEventType::ProgramChange(ProgramChange::read(reader, running_status_byte)?)
            })
        } else if code_byte == 0xD0u8 {
            // Channel Key Pressure
            Ok(MidiEvent {
                code_byte,
                channel,
                event: MidiEventType::ChannelKeyPressure(ChannelKeyPressure::read(reader, running_status_byte)?)
            })
        } else if code_byte == 0xE0u8 {
            // Pitch Bend
            Ok(MidiEvent {
                code_byte,
                channel,
                event: MidiEventType::PitchBend(PitchBend::read(reader, running_status_byte)?)
            })
        } else {
            Err(Box::new(UnknownEventError))
        }
    }
}


#[derive(Clone)]
pub enum SysexEventType {
    F0SysexEvent(Sysex),
    F7SysexEvent(Sysex)
}

#[derive(Clone)]
pub struct SysexEvent {
    pub event: SysexEventType
}

impl SysexEvent {
    pub fn read<R: Read>(reader: &mut R, code_byte: u8) -> Result<SysexEvent, Box<Error>> {
        let event: SysexEventType;
        if code_byte == 0xF0u8 {
            event = SysexEventType::F0SysexEvent(Sysex::read(reader)?);
            Ok(SysexEvent {
                event
            })
        } else if code_byte == 0xF7u8 {
            event = SysexEventType::F7SysexEvent(Sysex::read(reader)?);
            Ok(SysexEvent {
                event
            })
        } else {
            Err(Box::new(UnknownEventError))
        }
    }
}


#[derive(Clone)]
pub enum MetaEventType {
    SequenceNumber(SequenceNumber),
    TextEvent(Text),
    CopyrightNotice(Text),
    SequenceTrackName(Text),
    InstrumentName(Text),
    Lyric(Text),
    Marker(Text),
    CuePoint(Text),
    ProgramName(Text),
    DeviceName(Text),
    MIDIChannelPrefix(MIDIChannelPrefix),
    MIDIPort(MIDIPort),
    EndOfTrack(EndOfTrack),
    SetTempo(SetTempo),
    SMTPEOffset(SMTPEOffset),
    TimeSignature(TimeSignature),
    KeySignature(KeySignature),
    SequencerSpecific(SequencerSpecific),
    Unknown(Unknown)
}

#[derive(Clone)]
pub struct MetaEvent {
    pub sub_code_byte: u8,
    pub length: u32,
    pub event: MetaEventType
}

impl MetaEvent {
    pub fn read<R: Read + Seek>(reader: &mut R) -> Result<MetaEvent, Box<Error>> {
        let sub_code_byte: u8 = reader.read_to_u8()?;
        let length: u32 = reader.read_vlv()?.data;
        let mut to_skip: u32 = 0;
        let event: MetaEventType;
        if sub_code_byte == 0x00u8 {
            // Sequence Number
            event = MetaEventType::SequenceNumber(meta::SequenceNumber::read(reader)?);
            to_skip = length - 2u32;
        } else if sub_code_byte == 0x01u8 {
            // TextEvent
            event = MetaEventType::TextEvent(meta::Text::read(reader, length)?);
        } else if sub_code_byte == 0x02u8 {
            // Copyright Notice
            event = MetaEventType::CopyrightNotice(meta::Text::read(reader, length)?);
        } else if sub_code_byte == 0x03u8 {
            // Sequence/Track Name
            event = MetaEventType::SequenceTrackName(meta::Text::read(reader, length)?);
        } else if sub_code_byte == 0x04u8 {
            // Instrument Name
            event = MetaEventType::InstrumentName(meta::Text::read(reader, length)?);
        } else if sub_code_byte == 0x05u8 {
            // Lyric
            event = MetaEventType::Lyric(meta::Text::read(reader, length)?);
        } else if sub_code_byte == 0x06u8 {
            // Marker
            event = MetaEventType::Marker(meta::Text::read(reader, length)?);
        } else if sub_code_byte == 0x07u8 {
            // Cue Point
            event = MetaEventType::CuePoint(meta::Text::read(reader, length)?);
        } else if sub_code_byte == 0x08u8 {
            // Program Name
            event = MetaEventType::ProgramName(meta::Text::read(reader, length)?);
        } else if sub_code_byte == 0x09u8 {
            // Device Name
            event = MetaEventType::DeviceName(meta::Text::read(reader, length)?);
        } else if sub_code_byte == 0x20u8 {
            // MIDI Channel Prefix
            event = MetaEventType::MIDIChannelPrefix(meta::MIDIChannelPrefix::read(reader)?);
            to_skip = length - 1u32;
        } else if sub_code_byte == 0x21u8 {
            // Midi Port
            event = MetaEventType::MIDIPort(meta::MIDIPort::read(reader)?);
            to_skip = length - 1u32;
        } else if sub_code_byte == 0x2Fu8 {
            // End of Track
            event = MetaEventType::EndOfTrack(meta::EndOfTrack {});
            to_skip = length;
        } else if sub_code_byte == 0x51u8 {
            // Set Tempo
            event = MetaEventType::SetTempo(meta::SetTempo::read(reader)?);
            to_skip = length - 3u32;
        } else if sub_code_byte == 0x54u8 {
            // SMTPE Offset
            event = MetaEventType::SMTPEOffset(meta::SMTPEOffset::read(reader)?);
            to_skip = length - 5u32;
        } else if sub_code_byte == 0x58u8 {
            // Time Signature
            event = MetaEventType::TimeSignature(meta::TimeSignature::read(reader)?);
            to_skip = length - 4u32;
        } else if sub_code_byte == 0x59u8 {
            // Key Signature
            event = MetaEventType::KeySignature(meta::KeySignature::read(reader)?);
            to_skip = length - 2u32;
        } else if sub_code_byte == 0x7Fu8 {
            // Sequencer-Specific
            event = MetaEventType::SequencerSpecific(meta::SequencerSpecific::read(reader, length)?);
        } else {
            // Unknown, skip this stuff
            event = MetaEventType::Unknown(meta::Unknown {});
            to_skip = length;
        }
        reader.seek(SeekFrom::Current(to_skip as i64))?;
        Ok(MetaEvent {
            sub_code_byte,
            length,
            event
        })
    }
}


#[derive(Clone)]
pub enum EventType {
    MidiEvent(MidiEvent),
    SysExEvent(SysexEvent),
    MetaEvent(MetaEvent)
}

#[derive(Clone)]
pub struct Event {
    pub code_byte: u8,
    pub event: EventType
}

impl Event {
    pub fn new<R: Read + Seek>(reader: &mut R, last_event: Option<Event>) -> Result<Event, Box<Error>> {
        let event;
        let mut code_byte: u8 = reader.read_to_u8()?;
        let mut running_status_byte: Option<u8> = None;
        if (code_byte & 0b10000000u8 == 0u8) & (!last_event.is_some()) {  // Running Status
            return Err(Box::new(NoPreviousEvent))
        } else if (code_byte & 0b10000000u8 == 0u8) & (last_event.is_some()) {
            running_status_byte = Some(code_byte);
            code_byte = last_event.unwrap().code_byte;
        }
        if (code_byte & 0xF0u8 >= 0x80u8) & (code_byte & 0xF0u8 <= 0xE0u8) {
            event = EventType::MidiEvent(MidiEvent::read(reader, code_byte, running_status_byte)?);
            return Ok(Event {
                code_byte,
                event
            })
        } else if (code_byte == 0xF0u8) | (code_byte == 0xF7u8) {
            event = EventType::SysExEvent(SysexEvent::read(reader, code_byte)?);
            return Ok(Event {
                code_byte,
                event
            })
        } else if code_byte == 0xFFu8 {
            event = EventType::MetaEvent(MetaEvent::read(reader)?);
            return Ok(Event {
                code_byte,
                event
            })
        } else {
            return Err(Box::new(UnknownEventError))
        }
    }
}