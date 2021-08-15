#[derive(Debug)]
pub enum Version {
    V1,
}

impl Version {
    pub fn from_raw(bytes: u8) -> Version {
        match bytes {
            0 => Version::V1,
            _ => todo!(),
        }
    }
    pub fn serialize(&self) -> u8 {
        use self::Version::*;

        match self {
            V1 => 0,
        }
    }
}

#[derive(Debug)]
pub struct Header {
    pub version: Version,
    pub offset: u8,
    pub symbol_size: u32,
    pub content_size: u32,
}

impl Header {
    pub fn new(offset: u8, symbol_size: u32, content_size: u32) -> Self {
        Self {
            version: Version::V1,
            offset,
            symbol_size,
            content_size,
        }
    }

    pub fn from_raw(raw: u64) -> Self {
        Self::from(Version::from_raw((raw >> 60) as u8), raw)
    }

    pub fn from(version: Version, content: u64) -> Self {
        match version {
            // Check if this is correct
            Version::V1 => Self {
                version,
                offset: (content << 4 >> 60) as u8,
                symbol_size: (content << 8 >> 40) as u32,
                content_size: content as u32,
            },
        }
    }

    // Header Layout:
    // Vers offB     Symbols-Size     Content size (max 4.294967296gb)
    // 0000 0000 00000000000000000000 00000000000000000000000000000000
    pub fn serialize(&self) -> [u8; 8] {
        ((((*self).version.serialize() as u64) << 60)
            | (((*self).offset as u64) << 60 >> 4)
            | (((*self).symbol_size as u64) << 40 >> 8)
            | ((*self).content_size as u64))
            .to_be_bytes()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn experiments() {
        let num: u64 = 0x1234567890abcdef;

        println!("{:#018x}", num);
        println!("{:#018x}", (num >> 32) as u16);

        println!("{:#018x}", (num >> (8 * 7 + 4)) as u8);

        let num: u64 = 0b0000111100000000000000000000000000000000000000000000000000000000;

        println!("{:#010b}", num << 4 >> 60);
    }
}
