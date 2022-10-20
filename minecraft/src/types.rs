pub struct PlayerProfile {
    pub name: String
}

pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl Position {
    pub fn parse(v: u64) -> Self {
        let mut x = (v >> 38) as i32;
        let mut y = (v & 0xFFF) as i32;
        let mut z = (v >> 12 & 0x3FFFFFF) as i32;

        if x >= 1 << 25 { x -= 1 << 26 }
        if y >= 1 << 11 { y -= 1 << 12 }
        if z >= 1 << 25 { z -= 1 << 26 }

        Self { x, y, z}
    }

    pub fn to_long(&self) -> u64 {
        (self.x & 0x3FFFFFF).wrapping_shl(38) as u64 |
            (self.z & 0x3FFFFFF).wrapping_shl(12) as u64 | (self.y & 0xFFF) as u64
    }
}