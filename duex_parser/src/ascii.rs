#[derive(Copy, Clone)]
pub enum ASCII {
    // 特殊符号
    Newline = 10,
    Return = 13,
    Whitespace = 32,

    // 小括号
    LeftParenthes = 40,
    RightParenthes = 41,
    // 中括号
    LeftBracket = 91,
    RightBracket = 93,
    // 大括号
    LeftBrace = 123,
    RightBrace = 125,

    // symbol
    Plus = 43,
    Equal = 61,

    // dot
    Dot = 46,

    // semicolon
    Semicolon = 59,

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
}

pub fn from_u8(num: u8) -> Option<ASCII> {
    match num {
        10 => Some(ASCII::Newline),
        13 => Some(ASCII::Return),
        32 => Some(ASCII::Whitespace),

        40 => Some(ASCII::LeftParenthes),
        41 => Some(ASCII::RightParenthes),
        91 => Some(ASCII::LeftBracket),
        93 => Some(ASCII::RightBracket),
        123 => Some(ASCII::LeftBrace),
        125 => Some(ASCII::RightBrace),

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
