type Offset = usize;

#[derive(Debug)]
pub enum RomLevel {
    W1_1,
    W1_2,
    W1_3,
    W1_4,
    W2_1,
    W2_2,
    W2_3,
    W2_4,
    W3_1,
    W3_2,
    W3_3,
    W3_4,
    W4_1,
    W4_2,
    W4_3,
    W4_4,
    W5_1,
    W5_2,
    W5_3,
    W5_4,
    W6_1,
    W6_2,
    W6_3,
    W6_4,
    W7_1,
    W7_2,
    W7_3,
    W7_4,
    W8_1,
    W8_2,
    W8_3,
    W8_4,
}

impl RomLevel {
    pub fn from_name(name: &str) -> Self {
        match name {
            "1-1" => Self::W1_1,
            "1-2" => Self::W1_2,
            "1-3" => Self::W1_3,
            "1-4" => Self::W1_4,
            "2-1" => Self::W2_1,
            "2-2" => Self::W2_2,
            "2-3" => Self::W2_3,
            "2-4" => Self::W2_4,
            "3-1" => Self::W3_1,
            "3-2" => Self::W3_2,
            "3-3" => Self::W3_3,
            "3-4" => Self::W3_4,
            "4-1" => Self::W4_1,
            "4-2" => Self::W4_2,
            "4-3" => Self::W4_3,
            "4-4" => Self::W4_4,
            "5-1" => Self::W5_1,
            "5-2" => Self::W5_2,
            "5-3" => Self::W5_3,
            "5-4" => Self::W5_4,
            "6-1" => Self::W6_1,
            "6-2" => Self::W6_2,
            "6-3" => Self::W6_3,
            "6-4" => Self::W6_4,
            "7-1" => Self::W7_1,
            "7-2" => Self::W7_2,
            "7-3" => Self::W7_3,
            "7-4" => Self::W7_4,
            "8-1" => Self::W8_1,
            "8-2" => Self::W8_2,
            "8-3" => Self::W8_3,
            "8-4" => Self::W8_4,
            _ => panic!("unknown level name: {}", name),
        }
    }

    /// Get the header, object, and enemy offsets for a given level.
    pub fn get_offsets(&self) -> (Offset, Offset, Offset) {
        match self {
            Self::W1_1 => (0x269e, 0x26a0, 0x1f11),
            Self::W1_2 => (0x2c45, 0x2c47, 0x20e8),
            Self::W1_3 => (0x2703, 0x2705, 0x1f2f),
            Self::W1_4 => (0x21bf, 0x21c1, 0x1d80),
            Self::W2_1 => (0x27dd, 0x27df, 0x1f61),
            Self::W2_2 => (0x2e55, 0x2e57, 0x2181),
            Self::W2_3 => (0x2758, 0x275a, 0x1f4c),
            Self::W2_4 => (0x229f, 0x22a1, 0x1dc0),
            Self::W3_1 => (0x2629, 0x262b, 0x1ee0),
            Self::W3_2 => (0x2c12, 0x2c14, 0x20c3),
            Self::W3_3 => (0x247b, 0x247d, 0x1e69),
            Self::W3_4 => (0x2312, 0x2314, 0x1def),
            Self::W4_1 => (0x2547, 0x2549, 0x1eab),
            Self::W4_2 => (0x2ce8, 0x2cea, 0x2115),
            Self::W4_3 => (0x289f, 0x28a1, 0x1fb9),
            Self::W4_4 => (0x2220, 0x2222, 0x1da7),
            Self::W5_1 => (0x284b, 0x284d, 0x1f8c),
            Self::W5_2 => (0x2aa2, 0x2aa4, 0x2045),
            Self::W5_3 => (0x2703, 0x2705, 0x1f2f),
            Self::W5_4 => (0x229f, 0x22a1, 0x1dc0),
            Self::W6_1 => (0x296b, 0x296d, 0x2001),
            Self::W6_2 => (0x259a, 0x259c, 0x1eb9),
            Self::W6_3 => (0x2906, 0x2908, 0x1fde),
            Self::W6_4 => (0x21bf, 0x21c1, 0x1d80),
            Self::W7_1 => (0x2b8e, 0x2b90, 0x209e),
            Self::W7_2 => (0x2e55, 0x2e57, 0x2181),
            Self::W7_3 => (0x2758, 0x275a, 0x1f4c),
            Self::W7_4 => (0x237f, 0x2381, 0x1e1a),
            Self::W8_1 => (0x2a0f, 0x2a11, 0x200b),
            Self::W8_2 => (0x2b15, 0x2b17, 0x2070),
            Self::W8_3 => (0x24de, 0x24e0, 0x1e8e),
            Self::W8_4 => (0x240a, 0x240c, 0x1e2f),
        }
    }
          }