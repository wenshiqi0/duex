#[derive(Copy, Clone)]
pub enum ASCII {
    Newline = 10,
    Return = 13,
    Whitespace = 32,

    Plus = 43,
    Dot = 46,

    // number
    Zero = 48,
    One = 49,
    Two = 50,
    Three = 51,
    Four = 52,
    Five = 53,
    Six = 54,
    Seven = 55,
    Eight = 56,
    Nine = 57,

    Semicolon = 59,

    Equal = 61,
}

pub fn from_u8(num: u8) -> Option<ASCII> {
    match num {
        10 => Some(ASCII::Newline),
        13 => Some(ASCII::Return),
        32 => Some(ASCII::Whitespace),

        43 => Some(ASCII::Plus),
        46 => Some(ASCII::Dot),

        48 => Some(ASCII::Zero),
        49 => Some(ASCII::One),
        50 => Some(ASCII::Two),
        51 => Some(ASCII::Three),
        52 => Some(ASCII::Four),
        53 => Some(ASCII::Five),
        54 => Some(ASCII::Six),
        55 => Some(ASCII::Seven),
        56 => Some(ASCII::Eight),
        57 => Some(ASCII::Nine),

        59 => Some(ASCII::Semicolon),
        61 => Some(ASCII::Equal),
        _ => None,
    }
}
