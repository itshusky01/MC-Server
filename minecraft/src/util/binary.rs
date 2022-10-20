use std::mem::size_of;
use crate::error::{Result, Error, ErrorKind};

pub trait ReadN {
    fn _read(&mut self, n: usize) -> Result<Vec<u8>>;
}

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait BytesToNumber {
    fn from_bytes(b: Vec<u8>) -> Self;
}

pub struct VarInt { }

pub struct BinaryWriter {
    buffer: Vec<u8>
}

pub struct BinaryReader {
    buffer: Vec<u8>,
    offset: usize
}

pub struct PacketWriter{
    writer: BinaryWriter
}

impl VarInt {
    pub fn as_varint(v: i32) -> Vec<u8> {
        let len = Self::varint_byte_count(v);
        let mut buf = vec![0u8; len];
        let mut value = v;

        for i in 0..=len {
            if (value & !0x7f) == 0 {
                buf[i] = (value & 0x7f) as u8;
                break;
            } else {
                buf[i] = ((value & 0x7f) | 0x80) as u8;
                value >>= 7;
            }
        }
        buf
    }

    pub fn varint_byte_count(v: i32) -> usize {
        for i in 1..=4 {
            if (v & (-1i32 << (i * 7))) == 0 {
                return i;
            }
        }

        5
    }
}

pub trait ReadVarInt : ReadN {
    fn read_varint(&mut self) -> Result<i32> {
        let mut value = 0_i32;
        let mut shift = 0u32;

        loop {
            match self._read(1) {
                Err(e) => return Err(e),
                Ok(byte) => {
                    value |= (byte[0] as i32 & 127).wrapping_shl(shift * 7);
                    shift += 1;

                    if shift > 5 {
                        return Err(Error::new(ErrorKind::OutOfRange, "VarInt too big"));
                    }
                    
                    if (byte[0] as u8 & 128) != 128 {
                        break;
                    }
                }
            }
        }

        Ok(value)
    }

    fn read_varlong(&mut self) -> Result<i64> {
        let mut value = 0i64;
        let mut shift = 0u32;

        loop {
            match self._read(1) {
                Err(e) => return Err(e),
                Ok(byte) => {
                    value |= (byte[0] & 127).wrapping_shl(shift * 7) as i64;
                    shift += 1;
            
                    if shift > 10 {
                        return Err(Error::new(ErrorKind::OutOfRange, "VarLong too big"));
                    }

                    if (byte[0] as u8 & 128) != 128 {
                        break;
                    }
                }
            }
        }
    
        Ok(value)
    }
}

impl BinaryWriter {
    pub fn new() -> Self {
        BinaryWriter { buffer: Vec::new() }
    }

    pub fn write<T: ToBytes>(&mut self, data: T) {
        self.write_bytes(&mut data.to_bytes());
    }

    pub fn write_bytes(&mut self, bytes:&mut Vec<u8>) {
        self.buffer.append(bytes)
    }

    pub fn write_varint(&mut self, v: i32) {
        let len = VarInt::varint_byte_count(v);
        let mut buf = vec![0u8; len];
        let mut value = v;

        for i in 0..=len {
            if (value & !0x7f) == 0 {
                buf[i] = (value & 0x7f) as u8;
                break;
            } else {
                buf[i] = ((value & 0x7f) | 0x80) as u8;
                value >>= 7;
            }
        }
        
        self.write_bytes(&mut buf)
    }

    pub fn write_varlong(&mut self, v: i64) {
        !todo!()
    }

    pub fn length(&self) -> usize {
        self.buffer.len()
    }

    pub fn buffer_cloned(&self) -> Vec<u8> {
        self.buffer.to_vec()
    }

    pub fn buffer(&self) -> Vec<u8> {
        self.buffer.to_owned()
    }
}

impl PacketWriter {
    pub fn new() -> Self {
        PacketWriter { writer: BinaryWriter::new() }
    }

    pub fn write<T: ToBytes>(&mut self, data: T) {
        self.writer.write_bytes(&mut data.to_bytes());
    }

    pub fn write_bytes(&mut self, bytes:&mut Vec<u8>) {
        self.writer.write_bytes(bytes)
    }

    pub fn write_varint(&mut self, v: i32) {
        self.writer.write_varint(v)
    }

    pub fn write_varlong(&mut self, v: i64) {
        self.writer.write_varlong(v)
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut writer = BinaryWriter::new();
        writer.write_varint(self.writer.length() as i32);
        writer.write_bytes(&mut self.writer.buffer());

        writer.buffer()
    }
}

impl BinaryReader {
    pub fn new(source: Vec<u8>) -> Self {
        BinaryReader { buffer: source, offset: 0 }
    }

    pub fn read<T>(&mut self) -> Result<T> where T: BytesToNumber {
        let len = size_of::<T>();
        match self.read_bytes(len) {
            Err(e) => Err(e),
            Ok(v) =>{
                Ok(T::from_bytes(v))
            }
        }
    }

    pub fn read_bytes(&mut self, count: usize) -> Result<Vec<u8>> {
        if count == 0 {
            return Ok(vec![0u8;0]);
        }

        if self.buffer.len() >= self.offset + count {
            let data = &self.buffer[self.offset..self.offset+count];
            self.offset += count;
            Ok(data.to_vec())
        } else {
            Err(Error::new(ErrorKind::OutOfRange, "Out of range"))
        }
    }

    pub fn read_string(&mut self) -> Result<String> {
        match self.read_varint() {
            Err(e) => return Err(e),
            Ok(len) => {
                match self.read_bytes(len as usize) {
                    Err(e) => return Err(e),
                    Ok(s) => {
                        return match std::str::from_utf8(&s) {
                            Err(e) => Err(Error::new(ErrorKind::OutOfRange, &e.to_string())),
                            Ok(s) => Ok(s.to_string())
                        }
                    }
                }
            }
        }
    }

    pub fn length(&self) -> usize {
        self.buffer.len()
    }

    pub fn current(&self) -> usize {
        self.offset
    }

    pub fn at(&self, i: isize) -> Result<u8> {
        if i >= 0 {
            if self.buffer.len() > i as usize {
                Ok(self.buffer[i as usize])
            } else {
                Err(Error::new(ErrorKind::OutOfRange, "Out of range"))
            }
        } else {
            let pos = self.length() as isize - i.abs();
            self.at(pos)
        }
    }

    pub fn set_pos(&mut self, pos: usize) {
        self.offset = pos
    }

    pub fn buffer_cloned(&self) -> Vec<u8> {
        self.buffer.clone()
    }
}

impl ReadN for BinaryReader {
    fn _read(&mut self, n: usize) -> Result<Vec<u8>> {
        self.read_bytes(n)
    }
}
impl ReadVarInt for BinaryReader {}

macro_rules! num_bytes_convert_impl {
    ($t:ty) => {
        impl ToBytes for $t {
            fn to_bytes(&self) -> Vec<u8> {
                self.to_le_bytes().to_vec()
            }
        }

        impl BytesToNumber for $t {
            fn from_bytes(bytes: Vec<u8>) -> Self {
                Self::from_be_bytes(<[u8; size_of::<Self>()]>::try_from(bytes).unwrap())
            }
        }
    };
}

num_bytes_convert_impl!(u8);
num_bytes_convert_impl!(i8);
num_bytes_convert_impl!(u16);
num_bytes_convert_impl!(i16);
num_bytes_convert_impl!(u32);
num_bytes_convert_impl!(i32);
num_bytes_convert_impl!(u64);
num_bytes_convert_impl!(i64);
num_bytes_convert_impl!(u128);
num_bytes_convert_impl!(i128);

impl ToBytes for bool {
    fn to_bytes(&self) -> Vec<u8> {
        [if *self { 0x01 } else { 0x00 }].to_vec()
    }
}

impl BytesToNumber for bool {
    fn from_bytes(bytes: Vec<u8>) -> Self {
        if bytes[0] == 0x00 { false } else { true }
    }
}

impl ToBytes for &str {
    fn to_bytes(&self) -> Vec<u8> {
        let mut w = BinaryWriter::new();
        w.write_varint(self.len() as i32);
        w.write_bytes(&mut self.as_bytes().to_vec());
        w.buffer()
    }
}

impl ToBytes for String {
    fn to_bytes(&self) -> Vec<u8> {
        let s: &str = self;
        return s.to_bytes();
    }
}