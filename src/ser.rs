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
        let bytes = v.to_be_bytes();
        let mut new: Vec<u8> = bytes.into_iter().skip_while(|x| *x == 0).collect();
        if new.len() != 1 || (1..bytes.len() as u8 / 8).contains(&new[0]) {
            self.inner.write_all(&[new.len() as u8])?;
        }
        new.reverse();
        self.inner.write_all(&new)?;

        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        let bytes = v.to_le_bytes();
        let mut len = bytes.len();

        // Find minimal byte length that preserves sign and value
        while len > 1 && sign_extend_le(&bytes[..len - 1]) == v as i64 {
            len -= 1;
        }

        let new = &bytes[..len];

        if new.len() != 1 || (1..bytes.len() as u8 / 8).contains(&new[0]) {
            self.inner.write_all(&[new.len() as u8])?;
        }
        self.inner.write_all(new)?;

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
        let length = 0usize;
        let length_length = ((length as f32 + 1.0).log2().ceil() / 8.0).ceil() as usize;
        let length_bytes = &length.to_le_bytes()[..length_length];
        panic!("bytes: {length_bytes:02X?}. length_length: {length_length}. length: {length}");
        Ok(())
    }
    
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        todo!()
    }
    
    fn serialize_none(self) -> Result<Self::Ok> {
        todo!()
    }
    
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + serde::Serialize {
        todo!()
    }
    
    fn serialize_unit(self) -> Result<Self::Ok> {
        todo!()
    }
    
    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> {
        todo!()
    }
    
    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        todo!()
    }
    
    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + serde::Serialize {
        todo!()
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
        todo!()
    }
    
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        todo!()
    }
    
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        todo!()
    }
    
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        todo!()
    }
    
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        todo!()
    }
    
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        todo!()
    }
    
    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct> {
        todo!()
    }
    
    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        todo!()
    }
}

impl ser::SerializeMap for &mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<Self::Ok>
    where
        T: ?Sized + ser::Serialize {
        todo!()
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + ser::Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl ser::SerializeSeq for &mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + ser::Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl ser::SerializeStruct for &mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + ser::Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl ser::SerializeStructVariant for &mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + ser::Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl ser::SerializeTuple for &mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + ser::Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl ser::SerializeTupleStruct for &mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + ser::Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl ser::SerializeTupleVariant for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + ser::Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

#[test]
fn char_test() -> Result<()> {
    assert_eq!(to_bytes(&'c')?, b"c");
    assert_eq!(to_bytes(&'\0')?, b"\0");
    assert_eq!(to_bytes(&'\x01')?, b"\x01\x01");
    assert_eq!(to_bytes(&'ß')?, "\x02ß".as_bytes());
    assert_eq!(to_bytes(&'ℝ')?, "\x03ℝ".as_bytes());
    assert_eq!(to_bytes(&'💣')?, "\x04💣".as_bytes());
    Ok(())
}

#[test]
fn integer_test() -> Result<()> {
    assert_eq!(to_bytes(&5u16)?, [0x05]);
    assert_eq!(to_bytes(&16u16)?, [0x10]);
    assert_eq!(to_bytes(&256u16)?, [0x02, 0x00, 0x01]);
    assert_eq!(to_bytes(&-5i16)?, [0xfb]);
    assert_eq!(to_bytes(&-16i16)?, [0xf0]);
    assert_eq!(to_bytes(&256i16)?, [0x02, 0x00, 0x01]);
    assert_eq!(to_bytes(&-256i16)?, [0x02, 0x00, 0xff]);

    Ok(())
}

fn sign_extend_le(bytes: &[u8]) -> i64 {
    if bytes.len() > 8 || bytes.is_empty() {
        panic!("invalid bytes length {}", bytes.len());
    }
    if bytes.len() == 8 {
        return i64::from_le_bytes(bytes.try_into().unwrap());
    }

    let sign = bytes.last().unwrap() & 0x80 != 0; // true if negative
    let sign_byte = if sign { 0xffu8 } else { 0x00u8 };

    let mut vec: Vec<u8> = bytes.to_vec();
    vec.extend(vec![sign_byte; 8 - bytes.len()]);

    i64::from_le_bytes(vec.try_into().unwrap())
}

#[test]
fn sign_extend_test() {
    assert_eq!(sign_extend_le((5u8).to_le_bytes().as_slice()), 5);
    assert_eq!(sign_extend_le((-25i8).to_le_bytes().as_slice()), -25);
}