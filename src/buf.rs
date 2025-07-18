use std::io::{self, Read};

use crate::{Error, Result};

pub struct Buf<'src> {
    src: &'src [u8],
    cursor: usize,
}

impl<'src> Buf<'src> {
    pub fn new(src: &'src [u8]) -> Self {
        Buf { src, cursor: 0 }
    }

    fn handle_error<T>(res: io::Result<T>) -> Result<T> {
        if let Err(e) = res {
            match e.kind() {
                std::io::ErrorKind::UnexpectedEof => Err(Error::EOF),
                _ => Err(e)?,
            }
        } else if let Ok(v) = res {
            Ok(v)
        } else {
            unreachable!()
        }
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        let res = self.peek_u8();
        self.cursor += 1;
        res
    }

    pub fn peek_u8(&mut self) -> Result<u8> {
        let mut buf = [0; 1];
        Self::handle_error(self.src[self.cursor..].as_ref().read_exact(&mut buf))?;
        Ok(u8::from_le_bytes(buf))
    }

    pub fn read_i8(&mut self) -> Result<i8> {
        let res = self.peek_i8();
        self.cursor += 1;
        res
    }

    pub fn peek_i8(&mut self) -> Result<i8> {
        let mut buf = [0; 1];
        Self::handle_error(self.src[self.cursor..].as_ref().read_exact(&mut buf))?;
        Ok(i8::from_le_bytes(buf))
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        let res = self.peek_u16();
        self.cursor += 1;
        res
    }

    pub fn peek_u16(&mut self) -> Result<u16> {
        let mut buf = [0; 2];
        Self::handle_error(self.src[self.cursor..].as_ref().read_exact(&mut buf))?;
        Ok(u16::from_le_bytes(buf))
    }

    pub fn read_i16(&mut self) -> Result<i16> {
        let res = self.peek_i16();
        self.cursor += 1;
        res
    }

    pub fn peek_i16(&mut self) -> Result<i16> {
        let mut buf = [0; 2];
        Self::handle_error(self.src[self.cursor..].as_ref().read_exact(&mut buf))?;
        Ok(i16::from_le_bytes(buf))
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        let res = self.peek_u32();
        self.cursor += 1;
        res
    }

    pub fn peek_u32(&mut self) -> Result<u32> {
        let mut buf = [0; 4];
        Self::handle_error(self.src[self.cursor..].as_ref().read_exact(&mut buf))?;
        Ok(u32::from_le_bytes(buf))
    }

    pub fn read_i32(&mut self) -> Result<i32> {
        let res = self.peek_i32();
        self.cursor += 1;
        res
    }

    pub fn peek_i32(&mut self) -> Result<i32> {
        let mut buf = [0; 4];
        Self::handle_error(self.src[self.cursor..].as_ref().read_exact(&mut buf))?;
        Ok(i32::from_le_bytes(buf))
    }

    pub fn read_u64(&mut self) -> Result<u64> {
        let res = self.peek_u64();
        self.cursor += 1;
        res
    }

    pub fn peek_u64(&mut self) -> Result<u64> {
        let mut buf = [0; 8];
        Self::handle_error(self.src[self.cursor..].as_ref().read_exact(&mut buf))?;
        Ok(u64::from_le_bytes(buf))
    }

    pub fn read_i64(&mut self) -> Result<i64> {
        let res = self.peek_i64();
        self.cursor += 1;
        res
    }

    pub fn peek_i64(&mut self) -> Result<i64> {
        let mut buf = [0; 8];
        Self::handle_error(self.src[self.cursor..].as_ref().read_exact(&mut buf))?;
        Ok(i64::from_le_bytes(buf))
    }

    pub fn read_u128(&mut self) -> Result<u128> {
        let res = self.peek_u128();
        self.cursor += 1;
        res
    }

    pub fn peek_u128(&mut self) -> Result<u128> {
        let mut buf = [0; 16];
        Self::handle_error(self.src[self.cursor..].as_ref().read_exact(&mut buf))?;
        Ok(u128::from_le_bytes(buf))
    }

    pub fn read_i128(&mut self) -> Result<i128> {
        let res = self.peek_i128();
        self.cursor += 1;
        res
    }

    pub fn peek_i128(&mut self) -> Result<i128> {
        let mut buf = [0; 16];
        Self::handle_error(self.src[self.cursor..].as_ref().read_exact(&mut buf))?;
        Ok(i128::from_le_bytes(buf))
    }
}

impl<'src> Read for Buf<'src> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        
    }
}