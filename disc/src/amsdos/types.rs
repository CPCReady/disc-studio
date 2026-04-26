#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FileType {
    Basic = 0,
    BasicProtected = 1,
    Binary = 2,
    BinaryProtected = 3,
    Ascii = 255,
}

impl FileType {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => FileType::Basic,
            1 => FileType::BasicProtected,
            2 => FileType::Binary,
            3 => FileType::BinaryProtected,
            _ => FileType::Ascii,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            FileType::Basic => "BASIC",
            FileType::BasicProtected => "BASIC(P)",
            FileType::Binary => "BINARY",
            FileType::BinaryProtected => "BINARY(P)",
            FileType::Ascii => "ASCII",
        }
    }
}
