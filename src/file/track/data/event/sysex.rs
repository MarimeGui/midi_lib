#[derive(Clone)]
pub struct F0SysexEvent {
    pub length: u32,
    pub data: u32  // Might be bigger
}

#[derive(Clone)]
pub struct F7SysexEvent {
    pub length: u32,
    pub any_data: u32
}