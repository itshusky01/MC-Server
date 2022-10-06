use std::io::{Result, Error, ErrorKind};

pub struct BinaryReader {
    buffer: Vec<u8>,
    offset: usize
}

impl BinaryReader {
    pub fn new(source: Vec<u8>) -> Self {
        BinaryReader { buffer: source, offset: 0 }
    }

    pub fn read_8(&mut self) -> Result<i8> {
        match self.read_bytes(1) {
            Err(e) => Err(e),
            Ok(v) => Ok(v[0] as i8)
        }
    }

    pub fn read_16(&mut self) -> Result<i16> {
        match self.read_bytes(2) {
            Err(e) => Err(e),
            Ok(v) => {
                Ok(i16::from_be_bytes(<[u8; 2]>::try_from(v).unwrap()))
            }
        }
    }

    pub fn read_32(&mut self) -> Result<i32> {
        match self.read_bytes(4) {
            Err(e) => Err(e),
            Ok(v) => {
                Ok(i32::from_be_bytes(<[u8; 4]>::try_from(v).unwrap()))
            }
        }
    }

    pub fn read_64(&mut self) -> Result<i64> {
        match self.read_bytes(8) {
            Err(e) => Err(e),
            Ok(v) => {
                Ok(i64::from_be_bytes(<[u8; 8]>::try_from(v).unwrap()))
            }
        }
    }

    pub fn read_bytes(&mut self, count: usize) -> Result<Vec<u8>> {
        if self.buffer.len() >= self.offset + count {
            let data = &self.buffer[self.offset..self.offset+count-1];
            self.offset += count;
            Ok(data.to_vec())
        } else {
            Err(Error::new(ErrorKind::Other, "Out of index"))
        }
    }

    pub fn read_varint(&mut self) -> Result<i32> {
        let mut value = 0i32;
        let mut shift = 0u32;

        loop {
            match self.read_8() {
                Err(e) => return Err(e),
                Ok(byte) => {
                    value |= (byte & 127).wrapping_shl(shift * 7) as i32;
                    shift += 1;
            
                    if shift > 5 {
                        return  Err(Error::new(ErrorKind::Other, "VarInt too big"));
                    }
                            
                    if (byte as u8 & 128) != 128 {
                        break;
                    }
                }
            }
        }
    
        Ok(value)
    }

    pub fn read_varlong(&mut self) -> Result<i64> {
        let mut value = 0i64;
        let mut shift = 0u32;

        loop {
            match self.read_8() {
                Err(e) => return Err(e),
                Ok(byte) => {
                    value |= (byte & 127).wrapping_shl(shift * 7) as i64;
                    shift += 1;
            
                    if shift > 10 {
                        return  Err(Error::new(ErrorKind::Other, "VarInt too big"));
                    }

                    if (byte as u8 & 128) != 128 {
                        break;
                    }
                }
            }
        }
    
        Ok(value)
    }
}