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
    if v >= 0 {
        if v <= u8::MAX as i128 {
            (v as u8).to_le_bytes().to_vec()
        } else if v <= u16::MAX as i128 {
            (v as u16).to_le_bytes().to_vec()
        } else if v <= u32::MAX as i128 {
            (v as u32).to_le_bytes().to_vec()
        } else if v <= u64::MAX as i128 {
            (v as u64).to_le_bytes().to_vec()
        } else {
            (v as u128).to_le_bytes().to_vec()
        }
    } else if v >= i8::MIN as i128 && v <= i8::MAX as i128 {
        (v as i8).to_le_bytes().to_vec()
    } else if v >= i16::MIN as i128 && v <= i16::MAX as i128 {
        (v as i16).to_le_bytes().to_vec()
    } else if v >= i32::MIN as i128 && v <= i32::MAX as i128 {
        (v as i32).to_le_bytes().to_vec()
    } else if v >= i64::MIN as i128 && v <= i64::MAX as i128 {
        (v as i64).to_le_bytes().to_vec()
    } else {
        v.to_le_bytes().to_vec()
    }
}

#[test]
fn smallest_test() {
    assert_eq!(smallest_le_bytes(5u32), (5u8).to_le_bytes());
    assert_eq!(smallest_le_bytes(-1i128), (-1i8).to_le_bytes());
    assert_eq!(smallest_le_bytes(-256i32), (-256i16).to_le_bytes());
}