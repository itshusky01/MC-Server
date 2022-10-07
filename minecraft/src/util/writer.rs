use std::{io::{Result, Error, ErrorKind}};

pub struct BinaryWriter {
    buffer: Vec<u8>
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
        let len = Self::varint_len(v);
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
        
        self.write(value)
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

    pub fn varint_len(v: i32) -> usize {
        for i in 1..=4 {
            if (v & (-1i32 << (i * 7))) == 0 {
                return i;
            }
        }

        5
    }
}

pub struct PacketWriter{
    writer: BinaryWriter
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

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

macro_rules! to_bytes_impl {
    ($t:ty) => {
        impl ToBytes for $t {
            fn to_bytes(&self) -> Vec<u8> {
                self.to_le_bytes().to_vec()
            }
        }
    };
}

to_bytes_impl!(u8);
to_bytes_impl!(i8);
to_bytes_impl!(u16);
to_bytes_impl!(i16);
to_bytes_impl!(u32);
to_bytes_impl!(i32);
to_bytes_impl!(u64);
to_bytes_impl!(i64);
to_bytes_impl!(u128);
to_bytes_impl!(i128);