#[derive(Copy, Clone)]
pub enum ASCII {
    Newline = 10,
    Return = 13,
    Whitespace = 32,
    Semicolon = 59,
}

pub fn from_u8(num: u8) -> Option<ASCII> {
    match num {
        10 => Some(ASCII::Newline),
        13 => Some(ASCII::Return),
        32 => Some(ASCII::Whitespace),
        59 => Some(ASCII::Semicolon),
        _ => None,
    }
}
