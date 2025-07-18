use std::io::Write;

use crate::{Error, Result};

use serde::{Serialize, ser};

pub fn to_bytes<T: Serialize + ?Sized>(value: &T) -> Result<Vec<u8>> {
    to_bytes_settings(value, true, false)
}

// Used for testing, obviously
#[allow(unused)]
fn to_bytes_testing<T: Serialize + ?Sized>(value: &T) -> Result<Vec<u8>> {
    to_bytes_settings(value, false, false)
}

pub fn to_bytes_settings<T: Serialize + ?Sized>(
    value: &T,
    header: bool,
    high_precision: bool,
) -> Result<Vec<u8>> {
    let mut serializer = if header {
        generate_header(high_precision)
    } else {
        Serializer::new(Vec::new(), high_precision)
    };

    value.serialize(&mut serializer)?;

    Ok(serializer.inner)
}

fn generate_header(high_precision: bool) -> Serializer {
    let mut feature_flags = 0x00;
    if high_precision {
        feature_flags |= 1 << 0;
    }
    // Version 0
    let header = vec![0x00, feature_flags];
    Serializer::new(header, high_precision)
}

/// No header
fn to_bytes_with_settings<T: Serialize + ?Sized>(
    settings: &Serializer,
    value: &T,
) -> Result<Vec<u8>> {
    let mut serializer = settings.dup();

    value.serialize(&mut serializer)?;

    Ok(serializer.inner)
}

pub struct Serializer {
    inner: Vec<u8>,
    temp_bytes: Vec<u8>,
    temp_len: usize,
    high_precision: bool,
}

impl Serializer {
    fn new(inner: Vec<u8>, high_precision: bool) -> Self {
        Serializer {
            inner,
            temp_bytes: Vec::new(),
            temp_len: 0,
            high_precision,
        }
    }

    fn dup(&self) -> Self {
        Self::new(Vec::new(), self.high_precision)
    }

    fn serialize_uint(&mut self, bytes: &[u8]) -> Result<()> {
        let mut end = bytes.len();
        while end > 1 && bytes[end - 1] == 0 {
            end -= 1;
        }

        let slice = &bytes[..end];
        if slice.len() != 1 || (1..=bytes.len() as u8).contains(&slice[0]) {
            self.inner.write_all(&[end as u8])?;
        }
        self.inner.write_all(slice)?;

        Ok(())
    }

    fn serialize_int(&mut self, bytes: &[u8], v: i128) -> Result<()> {
        let mut len = bytes.len();

        while len > 1 && sign_extend_le(&bytes[..len - 1]) == v {
            len -= 1;
        }

        let new = &bytes[..len];

        if new.len() != 1 || (1..bytes.len() as u8 / 8).contains(&new[0]) {
            self.inner.write_all(&[new.len() as u8])?;
        }
        self.inner.write_all(new)?;

        Ok(())
    }
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
        self.serialize_uint(bytes.as_slice())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        let bytes = v.to_le_bytes();
        self.serialize_int(&bytes, v as i128)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        let bytes = v.to_le_bytes();
        self.serialize_uint(bytes.as_slice())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        let bytes = v.to_le_bytes();
        self.serialize_int(&bytes, v as i128)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        let bytes = v.to_le_bytes();
        self.serialize_uint(bytes.as_slice())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        let bytes = v.to_le_bytes();
        self.serialize_int(&bytes, v as i128)
    }

    fn serialize_u128(self, v: u128) -> std::result::Result<Self::Ok, Self::Error> {
        let bytes = v.to_le_bytes();
        self.serialize_uint(bytes.as_slice())
    }

    fn serialize_i128(self, v: i128) -> std::result::Result<Self::Ok, Self::Error> {
        let bytes = v.to_le_bytes();
        self.serialize_int(bytes.as_slice(), v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        let bits = v.to_bits();
        if self.high_precision {
            self.inner.write_all(&bits.to_le_bytes())?;
            return Ok(());
        }
        let sign = bits & (1 << 31) != 0;
        let mantissa = (((bits & (0xff << 23)) >> 23) as i32 - 127) as i8;
        let significand = if sign {
            -(((bits & 0x7fffff).reverse_bits() >> 9) as i32)
        } else {
            ((bits & 0x7fffff).reverse_bits() >> 9) as i32
        };

        let significand_rep = f32::from_bits(bits & 0x7fffff | 0x3f800000);
        println!("{v}: {significand_rep} * 2^{mantissa}");

        significand.serialize(&mut *self)?;
        mantissa.serialize(&mut *self)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        let bits = v.to_bits();
        if self.high_precision {
            self.inner.write_all(&bits.to_le_bytes())?;
            return Ok(());
        }
        let sign = (bits & (1 << 63)) << 63 != 0;
        let mantissa = ((((bits & (0x7ff << 52)) >> 52) as i64) - 1023) as i16;
        let significand = if sign {
            -(((bits & 0xfffffffffffff).reverse_bits() >> 12) as i32)
        } else {
            ((bits & 0xfffffffffffff).reverse_bits() >> 12) as i32
        };

        let significand_rep = f64::from_bits(bits & 0xfffffffffffff | 0x3ff0000000000000);
        println!("{v}: {significand_rep} * 2^{mantissa}");

        significand.serialize(&mut *self)?;
        mantissa.serialize(&mut *self)
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
        v.len().serialize(&mut *self)?;
        self.inner.write_all(v)?;
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        false.serialize(self)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + serde::Serialize,
    {
        let bytes = to_bytes_with_settings(self, value)?;
        if bytes.starts_with(&[0x00]) || bytes.starts_with(&[0x01]) {
            true.serialize(&mut *self)?;
        }
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _: &'static str,
        variant_index: u32,
        _: &'static str,
    ) -> Result<Self::Ok> {
        variant_index.serialize(self)
    }

    fn serialize_newtype_struct<T>(self, _: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _: &'static str,
        variant_index: u32,
        _: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + serde::Serialize,
    {
        variant_index.serialize(&mut *self)?;
        value.serialize(self)
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq> {
        self.temp_bytes.clear();
        self.temp_len = 0;
        Ok(self)
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple> {
        self.temp_bytes.clear();
        self.temp_len = 0;
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.temp_bytes.clear();
        self.temp_len = 0;
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        variant_index: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        variant_index.serialize(&mut *self)?;
        self.temp_bytes.clear();
        self.temp_len = 0;
        Ok(self)
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap> {
        self.temp_bytes.clear();
        self.temp_len = 0;
        Ok(self)
    }

    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct> {
        self.temp_bytes.clear();
        self.temp_len = 0;
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        variant_index: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant> {
        variant_index.serialize(&mut *self)?;
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
        T: ?Sized + ser::Serialize,
    {
        let bytes = to_bytes_with_settings(self, key)?;
        self.temp_len += 1;
        self.temp_bytes.write_all(&bytes)?;
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + ser::Serialize,
    {
        let bytes = to_bytes_with_settings(self, value)?;
        self.temp_bytes.write_all(&bytes)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        let len = self.temp_len;
        len.serialize(&mut *self)?;
        self.inner.write_all(&self.temp_bytes)?;
        Ok(())
    }
}

impl ser::SerializeSeq for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + ser::Serialize,
    {
        let bytes = to_bytes_with_settings(self, value)?;
        self.temp_len += 1;
        self.temp_bytes.write_all(&bytes)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        let len = self.temp_len;
        len.serialize(&mut *self)?;
        self.inner.write_all(&self.temp_bytes)?;
        Ok(())
    }
}

impl ser::SerializeStruct for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + ser::Serialize,
    {
        let bytes = to_bytes_with_settings(self, value)?;
        self.temp_bytes.write_all(&bytes)?;
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

    fn serialize_field<T>(&mut self, _: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + ser::Serialize,
    {
        let bytes = to_bytes_with_settings(self, value)?;
        self.temp_bytes.write_all(&bytes)?;
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
        T: ?Sized + ser::Serialize,
    {
        let bytes = to_bytes_with_settings(self, value)?;
        self.temp_bytes.write_all(&bytes)?;
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
        T: ?Sized + ser::Serialize,
    {
        let bytes = to_bytes_with_settings(self, value)?;
        self.temp_bytes.write_all(&bytes)?;
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
        T: ?Sized + ser::Serialize,
    {
        let bytes = to_bytes_with_settings(self, value)?;
        self.temp_bytes.write_all(&bytes)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        self.inner.write_all(&self.temp_bytes)?;
        Ok(())
    }
}

fn sign_extend_le(bytes: &[u8]) -> i128 {
    if bytes.len() > 16 || bytes.is_empty() {
        panic!("invalid bytes length {}", bytes.len());
    }
    if bytes.len() == 16 {
        return i128::from_le_bytes(bytes.try_into().unwrap());
    }

    let sign = bytes.last().unwrap() & 0x80 != 0; // true if negative
    let sign_byte = if sign { 0xffu8 } else { 0x00u8 };

    let mut vec: Vec<u8> = bytes.to_vec();
    vec.extend(vec![sign_byte; 16 - bytes.len()]);

    i128::from_le_bytes(vec.try_into().unwrap())
}

#[test]
fn char_test() -> Result<()> {
    assert_eq!(to_bytes_testing(&'c')?, b"c");
    assert_eq!(to_bytes_testing(&'\0')?, b"\0");
    assert_eq!(to_bytes_testing(&'\x01')?, b"\x01\x01");
    assert_eq!(to_bytes_testing(&'ß')?, "\x02ß".as_bytes());
    assert_eq!(to_bytes_testing(&'ℝ')?, "\x03ℝ".as_bytes());
    assert_eq!(to_bytes_testing(&'💣')?, "\x04💣".as_bytes());
    Ok(())
}

#[test]
fn integer_test() -> Result<()> {
    assert_eq!(to_bytes_testing(&5u16)?, [0x05]);
    assert_eq!(to_bytes_testing(&16u16)?, [0x10]);
    assert_eq!(to_bytes_testing(&256u16)?, [0x02, 0x00, 0x01]);
    assert_eq!(to_bytes_testing(&-5i16)?, [0xfb]);
    assert_eq!(to_bytes_testing(&-16i16)?, [0xf0]);
    assert_eq!(to_bytes_testing(&256i16)?, [0x02, 0x00, 0x01]);
    assert_eq!(to_bytes_testing(&-256i16)?, [0x02, 0x00, 0xff]);

    Ok(())
}

#[test]
fn float_test() -> Result<()> {
    assert_eq!(to_bytes_testing(&5.0f64)?, [0x02, 0x02]);
    assert_eq!(to_bytes_testing(&5.0f32)?, [0x02, 0x02]);
    assert_eq!(to_bytes_testing(&-5.0f32)?, [0xfe, 0x02]);
    assert_eq!(to_bytes_testing(&0.5f32)?, [0x00, 0xff]);
    assert_eq!(to_bytes_testing(&0.5f64)?, [0x00, 0xff]);
    assert_eq!(to_bytes_testing(&0.25f64)?, [0x00, 0xfe]);
    assert_eq!(to_bytes_testing(&0.25f64)?, [0x00, 0xfe]);
    assert_eq!(to_bytes_testing(&1f32)?, [0x00, 0x00]);
    assert_eq!(to_bytes_testing(&2.0f32)?, [0x00, 0x01]);
    assert_eq!(
        to_bytes_settings(&3.563f32, false, true)?,
        [0x31, 0x08, 0x64, 0x40]
    );

    Ok(())
}

#[test]
fn sign_extend_test() {
    assert_eq!(sign_extend_le((5u8).to_le_bytes().as_slice()), 5);
    assert_eq!(sign_extend_le((-25i8).to_le_bytes().as_slice()), -25);
}
