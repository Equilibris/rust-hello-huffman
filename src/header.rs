pub enum Version {
    V1,
}

impl Version {
    pub fn new(bytes: u8) -> Version {
        match bytes {
            0 => Version::V1,
            _ => todo!(),
        }
    }
    pub fn serialize(&self) -> u8 {
        use self::Version::*;

        match self {
            v1 => 0,
        }
    }
}

pub struct Header {
    version: Version,
    offset: u8,
    symbol_size: u32,
    content_size: u32,
}

impl Header {
    pub fn new_from_raw(raw: u64) -> Self {
        Self::new(Version::new((raw >> 60) as u8), raw)
    }

    pub fn new(version: Version, content: u64) -> Self {
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
    pub fn serialize(&self) -> u64 {
        (((*self).version.serialize() as u64) << 60)
            | (((*self).offset as u64) << 60 >> 4)
            | (((*self).symbol_size as u64) << 40 >> 8)
            | ((*self).content_size as u64)
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
