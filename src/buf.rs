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

    pub fn read_slice(&mut self, len: usize) -> Result<&[u8]> {
        if self.src.len() - self.cursor < len {
            Err(Error::EOF)
        } else {
            let old = self.cursor;
            self.cursor += len;
            Ok(&self.src[old..self.cursor])
        }
    }
}

impl<'src> Read for Buf<'src> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let res = self.src[self.cursor..].as_ref().read(buf);
        if let Ok(n) = res {
            self.cursor += n;
        }
        res
    }
}
