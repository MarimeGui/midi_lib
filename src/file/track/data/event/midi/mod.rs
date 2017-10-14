pub mod channel_voice_message;
pub mod channel_mode_message;

use self::channel_voice_message::*;
use self::channel_mode_message::*;

#[derive(Clone)]
pub enum VoiceEventType {
    NoteOff(NoteChange),
    NoteOn(NoteChange),
    PolyphonicKeyPressure(PolyphonicKeyPressure),
    ControllerChange(ControllerChange),
    ProgramChange(ProgramChange),
    ChannelKeyPressure(ChannelKeyPressure),
    PitchBend(PitchBend)
}

#[derive(Clone)]
pub enum ModeEventType {
    AllSoundOff(AllSoundOff),
    ResetAllControllers(ResetAllControllers),
    LocalControl(LocalControl),
    AllNotesOff(AllNotesOff),
    OmniModeOff(OmniModeOff),
    OmniModeOn(OmniModeOn),
    MonoModeOn(MonoModeOn),
    PolyModeOn(PolyModeOn)
}