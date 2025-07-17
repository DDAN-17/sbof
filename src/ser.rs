use std::{any::Any, io::Write};

use crate::{Error, Result};

use serde::{ser, Serialize};

pub fn to_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    let mut serializer = Serializer::default();

    value.serialize(&mut serializer)?;

    Ok(serializer.inner)
}

#[derive(Default)]
pub struct Serializer {
    inner: Vec<u8>,
    temp_bytes: Vec<u8>,
    temp_len: usize,
}

impl ser::Serializer for &mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        self.inner.push(if v { 1 } else { 0 });
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        self.inner.write_all(&v.to_le_bytes())?;
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        self.inner.write_all(&v.to_le_bytes())?;
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        let bytes = v.to_le_bytes();
        let mut new_bytes = Vec::with_capacity(bytes.len());
        for i in bytes {
            if i != 0x00 {
                new_bytes.push(i);
            } else {
                break;
            }
        }

        if new_bytes.len() == 1 && new_bytes[0] != 0 && new_bytes[0] as usize > bytes.len() {
            self.inner.write_all(&new_bytes)?;
        } else {
            self.inner.write_all(&[new_bytes.len() as u8])?;
            self.inner.write_all(&new_bytes)?;
        }

        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        self.inner.write_all(&smallest_le_bytes(v))?;
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        let bytes = v.to_le_bytes();
        let mut new_bytes = Vec::with_capacity(bytes.len());
        for i in bytes {
            if i != 0x00 {
                new_bytes.push(i);
            } else {
                break;
            }
        }

        if new_bytes.len() == 1 && new_bytes[0] != 0 && new_bytes[0] as usize > bytes.len() {
            self.inner.write_all(&new_bytes)?;
        } else {
            self.inner.write_all(&[new_bytes.len() as u8])?;
            self.inner.write_all(&new_bytes)?;
        }

        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        let bytes = v.to_le_bytes();
        let mut new_bytes = Vec::with_capacity(bytes.len());
        for i in bytes {
            if i != 0x00 {
                new_bytes.push(i);
            } else {
                break;
            }
        }

        if new_bytes.len() == 1 && new_bytes[0] != 0 && new_bytes[0] as usize > bytes.len() {
            self.inner.write_all(&new_bytes)?;
        } else {
            self.inner.write_all(&[new_bytes.len() as u8])?;
            self.inner.write_all(&new_bytes)?;
        }

        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        let bytes = v.to_le_bytes();
        let mut new_bytes = Vec::with_capacity(bytes.len());
        for i in bytes {
            if i != 0x00 {
                new_bytes.push(i);
            } else {
                break;
            }
        }

        if new_bytes.len() == 1 && new_bytes[0] != 0 && new_bytes[0] as usize > bytes.len() {
            self.inner.write_all(&new_bytes)?;
        } else {
            self.inner.write_all(&[new_bytes.len() as u8])?;
            self.inner.write_all(&new_bytes)?;
        }

        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        let bytes = v.to_le_bytes();
        let mut new_bytes = Vec::with_capacity(bytes.len());
        for i in bytes {
            if i != 0x00 {
                new_bytes.push(i);
            } else {
                break;
            }
        }

        if new_bytes.len() == 1 && new_bytes[0] != 0 && new_bytes[0] as usize > bytes.len() {
            self.inner.write_all(&new_bytes)?;
        } else {
            self.inner.write_all(&[new_bytes.len() as u8])?;
            self.inner.write_all(&new_bytes)?;
        }

        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        let bytes = v.to_le_bytes();
        let mut new_bytes = Vec::with_capacity(bytes.len());
        for i in bytes {
            if i != 0x00 {
                new_bytes.push(i);
            } else {
                break;
            }
        }

        if new_bytes.len() == 1 && new_bytes[0] != 0 && new_bytes[0] as usize > bytes.len() {
            self.inner.write_all(&new_bytes)?;
        } else {
            self.inner.write_all(&[new_bytes.len() as u8])?;
            self.inner.write_all(&new_bytes)?;
        }

        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        let bytes = v.to_le_bytes();
        let mut new_bytes = Vec::with_capacity(bytes.len());
        for i in bytes {
            if i != 0x00 {
                new_bytes.push(i);
            } else {
                break;
            }
        }

        if new_bytes.len() == 1 && new_bytes[0] != 0 && new_bytes[0] as usize > bytes.len() {
            self.inner.write_all(&new_bytes)?;
        } else {
            self.inner.write_all(&[new_bytes.len() as u8])?;
            self.inner.write_all(&new_bytes)?;
        }

        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        let len = v.len_utf8() as u8;
        if len == 1 {
            let byte = v as u8;
            if (1..=4).contains(&byte) {
                self.inner.write_all(&[1, byte])?; // 01 XX
            } else {
                self.inner.write_all(&[byte])?; // XX
            }
        } else {
            let mut buf = vec![len; len as usize + 1];
            v.encode_utf8(&mut buf[1..]);
            self.inner.write_all(&buf[..])?; // ll XX XX XX XX
        }
        Ok(())
    }
    
    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        v.as_bytes().serialize(self)
    }
    
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        v.len().serialize(self);
        self.inner.write_all(v);
        Ok(())
    }
    
    fn serialize_none(self) -> Result<Self::Ok> {
        false.serialize(self)
    }
    
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + serde::Serialize {
        let bytes = to_bytes(value);
        if bytes.starts_with(&[0x00]) || bytes.starts_with(&[0x01]) {
            true.serialize(self);
        }
        value.serialize(self)
    }
    
    fn serialize_unit(self) -> Result<Self::Ok> {
        Ok(())
    }
    
    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> {
        Ok(())
    }
    
    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        variant_index.serialize(self)
    }
    
    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + serde::Serialize {
        value.serialize(self)
    }
    
    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + serde::Serialize {
        variant_index.serialize(self)?;
        value.serialize(self)
    }
    
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.temp_bytes.clear();
        self.temp_len = 0;
        Ok(self)
    }
    
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.temp_bytes.clear();
        self.temp_len = 0;
        Ok(self)
    }
    
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.temp_bytes.clear();
        self.temp_len = 0;
        Ok(self)
    }
    
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        variant_index.serialize(self)?;
        self.temp_bytes.clear();
        self.temp_len = 0;
        Ok(self)
    }
    
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        self.temp_bytes.clear();
        self.temp_len = 0;
        Ok(self)
    }
    
    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct> {
        self.temp_bytes.clear();
        self.temp_len = 0;
        Ok(self)
    }
    
    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        variant_index.serialize(self);
        self.temp_bytes.clear();
        self.temp_len = 0;
        Ok(self)
    }
}

impl ser::SerializeMap for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<Self::Ok>
    where
        T: ?Sized + ser::Serialize {
        let bytes = to_bytes(key)?;
        self.temp_len += 1;
        self.temp_bytes.write_all(&bytes)?;
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + ser::Serialize {
        let bytes = to_bytes(value)?;
        self.temp_bytes.write_all(&bytes)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        self.temp_len.serialize(self)?;
        self.inner.write_all(&self.temp_bytes)?;
        Ok(())
    }
}

impl ser::SerializeSeq for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + ser::Serialize {
        let bytes = to_bytes(value)?;
        self.temp_len += 1;
        self.temp_bytes.write_all(&bytes);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        self.temp_len.serialize(self)?;
        self.inner.write_all(&self.temp_bytes)?;
        Ok(())
    }
}

impl ser::SerializeStruct for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + ser::Serialize {
        let bytes = to_bytes(value)?;
        self.temp_bytes.write_all(&bytes);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        self.inner.write_all(&self.temp_bytes)?;
        Ok(())
    }
}

impl ser::SerializeStructVariant for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + ser::Serialize {
        let bytes = to_bytes(value)?;
        self.temp_bytes.write_all(&bytes);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        self.inner.write_all(&self.temp_bytes)?;
        Ok(())
    }
}

impl ser::SerializeTuple for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + ser::Serialize {
        let bytes = to_bytes(value)?;
        self.temp_bytes.write_all(&bytes);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        self.inner.write_all(&self.temp_bytes)?;
        Ok(())
    }
}

impl ser::SerializeTupleStruct for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + ser::Serialize {
        let bytes = to_bytes(value)?;
        self.temp_bytes.write_all(&bytes);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        self.inner.write_all(&self.temp_bytes)?;
        Ok(())
    }
}

impl ser::SerializeTupleVariant for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + ser::Serialize {
        let bytes = to_bytes(value)?;
        self.temp_bytes.write_all(&bytes);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        self.inner.write_all(&self.temp_bytes)?;
        Ok(())
    }
}

#[test]
fn char_test() -> Result<()> {
    let c = to_bytes(&'c')?;
    let null = to_bytes(&'\0')?;
    let one = to_bytes(&'\x01')?;
    let two_byte = to_bytes(&'ß')?;
    let three_byte = to_bytes(&'ℝ')?;
    let four_byte = to_bytes(&'💣')?;
    assert_eq!(c, b"c");
    assert_eq!(null, b"\0");
    assert_eq!(one, b"\x01\x01");
    assert_eq!(two_byte, "\x02ß".as_bytes());
    assert_eq!(three_byte, "\x03ℝ".as_bytes());
    assert_eq!(four_byte, "\x04💣".as_bytes());
    Ok(())
}

pub fn smallest_le_bytes<T: Into<i128> + Copy>(value: T) -> Vec<u8> {
    let v: i128 = value.into();
    let bytes = v.to_le_bytes();
    let mut end = bytes.len();
    // Remove trailing zeros for positive, trailing 0xFF for negative
    if v >= 0 {
        while end > 1 && bytes[end - 1] == 0 {
            end -= 1;
        }
    } else {
        while end > 1 && bytes[end - 1] == 0xFF {
            end -= 1;
        }
    }
    bytes[..end].to_vec()
}

#[test]
fn smallest_test() {
    assert_eq!(smallest_le_bytes(5u32), (5u8).to_le_bytes());
    assert_eq!(smallest_le_bytes(-1i128), (-1i8).to_le_bytes());
    assert_eq!(smallest_le_bytes(-256i32), (-256i16).to_le_bytes());
}
