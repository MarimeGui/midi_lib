// Standard Midi File Types
#[derive(Clone)]
pub enum MidiFormat {
    SingleTrack,
    SimultaneousTracks,
    IndependentTracks
}

// Standard Midi File Division System
#[derive(Clone)]
pub enum MidiDivisionsType {
    TicksPerQuarterNote(MidiTPQNDivisions),
    SMTPEFrames(MidiSMTPEDivisions)
}

// Ticks per Quartet Note System
#[derive(Clone)]
pub struct MidiTPQNDivisions {
    pub ticks_per_quarter_note: u16
}

// SMTPE System
#[derive(Clone)]
pub struct MidiSMTPEDivisions {
    pub ticks_per_smtpe_frame: u16,
    pub smtpe_frames_per_second: u16
}