#[derive(Clone)]
pub struct AllSoundOff {
    pub channel: u8
}

#[derive(Clone)]
pub struct ResetAllControllers {
    pub channel: u8
}

#[derive(Clone)]
pub struct LocalControl {
    pub channel: u8,
    pub reconnect: bool
}

#[derive(Clone)]
pub struct AllNotesOff {
    pub channel: u8
}

#[derive(Clone)]
pub struct OmniModeOff {
    pub channel: u8
}

#[derive(Clone)]
pub struct OmniModeOn {
    pub channel: u8
}

#[derive(Clone)]
pub struct MonoModeOn {
    pub channel: u8,
    pub number: u8
}

#[derive(Clone)]
pub struct PolyModeOn {
    pub channel: u8
}