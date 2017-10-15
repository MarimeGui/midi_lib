pub mod midi;
pub mod sysex;
pub mod meta;

use ez_io::ReadE;
use self::midi::*;
use self::midi::channel_voice_message::*;
// use self::midi::channel_mode_message::*;
use std::io::Seek;
use std::io::SeekFrom;
use self::sysex::*;
use self::meta::*;
use std::error::Error;
use std::fmt;
use std::io::Read;
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
pub enum MidiEventTypes {
    ChannelVoiceMessage(VoiceEventType),
    ChannelModeMessage(ModeEventType)
}

#[derive(Clone)]
pub enum SysexEventTypes {
    F0SysexEvent(F0SysexEvent),
    F7SysexEvent(F7SysexEvent)
}

#[derive(Clone)]
pub enum MetaEventTypes {
    SequenceNumber(SequenceNumber),
    TextEvent(TextEvent),
    CopyrightNotice(TextEvent),
    SequenceTrackName(TextEvent),
    InstrumentName(TextEvent),
    Lyric(TextEvent),
    Marker(TextEvent),
    CuePoint(TextEvent),
    MIDIChannelPrefix(MIDIChannelPrefix),
    EndOfTrack(EndOfTrack),
    SetTempo(SetTempo),
    SMTPEOffset(SMTPEOffset),
    TimeSignature(TimeSignature),
    KeySignature(KeySignature),
    SequencerSpecificMetaEvent(SequencerSpecificMetaEvent)
}

#[derive(Clone)]
pub enum Event {
    MidiEvent(MidiEventTypes),
    SysExEvent(SysexEventTypes),
    MetaEvent(MetaEventTypes)
}

impl Event {
    pub fn new<R: Read + Seek>(reader: &mut R, last_event: Option<Event>) -> Result<Event, Box<Error>> {
        let event: u8 = reader.read_to_u8().unwrap();
        if (event & 0b10000000u8 == 0u8) & (!last_event.is_some()) {  // Running Status
            Err(Box::new(NoPreviousEvent))
        } else if (event & 0b10000000u8 == 0u8) & (last_event.is_some()) {
            unimplemented!();
        } else if event & 0xF0u8 == 0x80u8 {  // Note Off
            Ok(Event::MidiEvent(MidiEventTypes::ChannelVoiceMessage(VoiceEventType::NoteOff(NoteChange::read(reader, event & 0x0Fu8)?))))
        } else if event & 0xF0u8 == 0x90u8 {  // Note On
            Ok(Event::MidiEvent(MidiEventTypes::ChannelVoiceMessage(VoiceEventType::NoteOn(NoteChange::read(reader, event & 0x0Fu8)?))))
        } else if event & 0xF0u8 == 0xA0u8 {  // Polyphonic Key Pressure
            Ok(Event::MidiEvent(MidiEventTypes::ChannelVoiceMessage(VoiceEventType::PolyphonicKeyPressure(PolyphonicKeyPressure::read(reader, event & 0x0Fu8)?))))
        } else if event & 0xF0u8 == 0xB0u8 {  // Controller Change
            Ok(Event::MidiEvent(MidiEventTypes::ChannelVoiceMessage(VoiceEventType::ControllerChange(ControllerChange::read(reader, event & 0x0Fu8)?))))
        } else if event & 0xF0u8 == 0xC0u8 {  // Program Change
            Ok(Event::MidiEvent(MidiEventTypes::ChannelVoiceMessage(VoiceEventType::ProgramChange(ProgramChange::read(reader, event & 0x0Fu8)?))))
        } else if event & 0xF0u8 == 0xD0u8 {  // Channel Key Pressure
            Ok(Event::MidiEvent(MidiEventTypes::ChannelVoiceMessage(VoiceEventType::ChannelKeyPressure(ChannelKeyPressure::read(reader, event & 0x0Fu8)?))))
        } else if event & 0xF0u8 == 0xE0u8 {  // Pitch Bend
            Ok(Event::MidiEvent(MidiEventTypes::ChannelVoiceMessage(VoiceEventType::PitchBend(PitchBend::read(reader, event & 0x0Fu8)?))))
        } else if event == 0xFF {  // Meta Event
            let sub_event: u8 = reader.read_to_u8().unwrap();
            if sub_event == 0x00u8 {  // Sequence Number
                if reader.read_to_u8()? == 0x02u8 {
                    Ok(Event::MetaEvent(MetaEventTypes::SequenceNumber(meta::SequenceNumber::read(reader)?)))
                } else {
                    Err(Box::new(UnknownEventError))
                }
            } else if sub_event == 0x01u8 {  // TextEvent
                Ok(Event::MetaEvent(MetaEventTypes::TextEvent(meta::TextEvent::read(reader)?)))
            } else if sub_event == 0x02u8 {  // Copyright Notice
                Ok(Event::MetaEvent(MetaEventTypes::CopyrightNotice(meta::TextEvent::read(reader)?)))
            } else if sub_event == 0x03u8 {  // Sequence/Track Name
                Ok(Event::MetaEvent(MetaEventTypes::SequenceTrackName(meta::TextEvent::read(reader)?)))
            } else if sub_event == 0x04u8 {  // Instrument Name
                Ok(Event::MetaEvent(MetaEventTypes::InstrumentName(meta::TextEvent::read(reader)?)))
            } else if sub_event == 0x05u8 {  // Lyric
                Ok(Event::MetaEvent(MetaEventTypes::Lyric(meta::TextEvent::read(reader)?)))
            } else if sub_event == 0x06u8 {  // Marker
                Ok(Event::MetaEvent(MetaEventTypes::Marker(meta::TextEvent::read(reader)?)))
            } else if sub_event == 0x07u8 {  // Cue Point
                Ok(Event::MetaEvent(MetaEventTypes::CuePoint(meta::TextEvent::read(reader)?)))
            } else if sub_event == 0x20u8 {  // MIDI Channel Prefix
                if reader.read_to_u8()? == 0x01u8 {
                    Ok(Event::MetaEvent(MetaEventTypes::MIDIChannelPrefix(meta::MIDIChannelPrefix::read(reader)?)))
                } else {
                    Err(Box::new(UnknownEventError))
                }
            } else if sub_event == 0x2Fu8 {  // End of Track
                if reader.read_to_u8()? == 0x00 {
                    Ok(Event::MetaEvent(MetaEventTypes::EndOfTrack(meta::EndOfTrack {})))
                } else {
                    Err(Box::new(UnknownEventError))
                }
            } else if sub_event == 0x51u8 {  // Set Tempo
                if reader.read_to_u8()? == 0x03u8 {
                    Ok(Event::MetaEvent(MetaEventTypes::SetTempo(meta::SetTempo::read(reader)?)))
                } else {
                    Err(Box::new(UnknownEventError))
                }
            } else if sub_event == 0x54u8 {  // SMTPE Offset
                if reader.read_to_u8()? == 0x05u8 {
                    Ok(Event::MetaEvent(MetaEventTypes::SMTPEOffset(meta::SMTPEOffset::read(reader)?)))
                } else {
                    Err(Box::new(UnknownEventError))
                }
            } else if sub_event == 0x58u8 {  // Time Signature
                if reader.read_to_u8()? == 0x04u8 {
                    Ok(Event::MetaEvent(MetaEventTypes::TimeSignature(meta::TimeSignature::read(reader)?)))
                } else {
                    Err(Box::new(UnknownEventError))
                }
            } else if sub_event == 0x59u8 {  // Key Signature
                if reader.read_to_u8()? == 0x02u8 {
                    Ok(Event::MetaEvent(MetaEventTypes::KeySignature(meta::KeySignature::read(reader)?)))
                } else {
                    Err(Box::new(UnknownEventError))
                }
            } else if sub_event == 0x7Fu8 {  // Sequencer-Specific Meta-event
                Ok(Event::MetaEvent(MetaEventTypes::SequencerSpecificMetaEvent(meta::SequencerSpecificMetaEvent::read(reader)?)))
            } else {
                Err(Box::new(UnknownEventError))
            }
        } else {
            Err(Box::new(UnknownEventError))
        }
    }
}
