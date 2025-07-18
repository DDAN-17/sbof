use std::io::Read;

use crate::{Error, Result, buf::Buf, sign_extend_le};

use serde::{
    Deserialize,
    de::{self, IntoDeserializer, value::U32Deserializer},
};

pub fn from_bytes<'de, T: Deserialize<'de>>(bytes: &'de [u8]) -> Result<T> {
    let version = bytes[0];
    if version > 0 {
        return Err(Error::UnsupportedVersion);
    }
    let high_precision = bytes[1] & (1 << 0) != 0;
    from_bytes_settings(&bytes[2..], version, high_precision)
}

pub fn from_bytes_settings<'de, T: Deserialize<'de>>(
    bytes: &'de [u8],
    version: u8,
    high_precision: bool,
) -> Result<T> {
    let mut deserializer = Deserializer {
        input: Buf::new(bytes),
        version,
        high_precision,
    };
    T::deserialize(&mut deserializer)
}

// Used in testing, obviously
#[allow(unused)]
fn from_bytes_testing<'de, T: Deserialize<'de>>(bytes: &'de [u8]) -> Result<T> {
    from_bytes_settings(bytes, 0, false)
}

pub struct Deserializer<'de> {
    input: Buf<'de>,

    #[allow(unused)]
    version: u8,
    // Feature flags
    high_precision: bool,
}

impl<'de> Deserializer<'de> {
    fn deserialize_int(&mut self, max_length: u8) -> Result<i128> {
        let byte = self.input.read_u8()?;
        if byte > max_length || byte == 0 {
            Ok(i8::from_le_bytes([byte]) as i128)
        } else {
            let mut buf = vec![0; byte as usize];
            self.input.read_exact(&mut buf)?;
            Ok(sign_extend_le(&buf))
        }
    }

    fn deserialize_uint(&mut self, max_length: u8) -> Result<u128> {
        let byte = self.input.read_u8()?;
        if byte > max_length || byte == 0 {
            Ok(byte as u128)
        } else {
            let mut buf = vec![0; 16];
            self.input.read_exact(&mut buf[..byte as usize])?;
            Ok(u128::from_le_bytes(buf.try_into().unwrap()))
        }
    }

    fn deserialize_byte_arr(&mut self) -> Result<&[u8]> {
        let len = self.deserialize_uint(u8::MAX)? as usize; // Infinitely sized integer
        self.input.read_slice(len)
    }
}

impl<'de> de::Deserializer<'de> for &mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::Unsupported {
            name: "deserialize_any",
            reason: "SBOF is not a self-describing format",
        })
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let byte = self.input.read_u8()?;
        let val = match byte {
            0 => false,
            1 => true,
            _ => {
                return Err(Error::InvalidValue {
                    value: byte as u32,
                    reason: "expected bool",
                });
            }
        };
        visitor.visit_bool(val)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i8(self.input.read_i8()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i16(self.deserialize_int(2)? as i16)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i32(self.deserialize_int(4)? as i32)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i64(self.deserialize_int(8)? as i64)
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i128(self.deserialize_int(16)?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u8(self.input.read_u8()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u16(self.deserialize_uint(2)? as u16)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u32(self.deserialize_uint(4)? as u32)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u64(self.deserialize_uint(8)? as u64)
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u128(self.deserialize_uint(16)?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if self.high_precision {
            let mut buf = [0; 4];
            self.input.read_exact(&mut buf)?;
            return visitor.visit_f32(f32::from_le_bytes(buf));
        }

        let significand = self.deserialize_int(4)? as i32;
        let (sign, significand) = if significand.is_negative() {
            (1u32, ((-significand << 9).reverse_bits()) as u32)
        } else {
            (0u32, (significand << 9).reverse_bits() as u32)
        };
        let mantissa = (self.input.read_i8()?.wrapping_add(127)) as u8;

        let bits = significand | (mantissa as u32) << 23 | sign << 31;

        visitor.visit_f32(f32::from_bits(bits))
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if self.high_precision {
            let mut buf = [0; 8];
            self.input.read_exact(&mut buf)?;
            return visitor.visit_f64(f64::from_le_bytes(buf));
        }

        let significand = self.deserialize_int(8)? as i64;
        let mantissa = self.deserialize_int(2)? as i16;
        let (sign, significand) = if significand.is_negative() {
            (1u64, ((-significand << 12).reverse_bits()) as u64)
        } else {
            (0u64, (significand << 12).reverse_bits() as u64)
        };
        let mantissa = (mantissa.wrapping_add(1023) & 0x7ff) as u16;

        let bits = significand | (mantissa as u64) << 52 | sign << 63;

        visitor.visit_f64(f64::from_bits(bits))
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let int = self.deserialize_uint(4)? as u32;
        let char = char::from_u32(int).ok_or(Error::InvalidValue {
            value: int,
            reason: "expected valid character",
        })?;
        visitor.visit_char(char)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_str(
            str::from_utf8(self.deserialize_byte_arr()?).map_err(|_| Error::InvalidUTF8)?,
        )
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_string(
            str::from_utf8(self.deserialize_byte_arr()?)
                .map_err(|_| Error::InvalidUTF8)?
                .to_string(),
        )
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_bytes(self.deserialize_byte_arr()?)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_byte_buf(self.deserialize_byte_arr()?.to_vec())
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if self.input.peek_u8()? == 0 {
            self.input.read_u8()?;
            visitor.visit_none()
        } else if self.input.peek_u8()? == 1 {
            self.input.read_u8()?;
            visitor.visit_some(self)
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(self, _: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let len_left = self.deserialize_uint(u8::MAX)? as usize;
        visitor.visit_seq(SbofSeq::new(self, len_left))
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(SbofSeq::new(self, len))
    }

    fn deserialize_tuple_struct<V>(
        self,
        _: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let len_left = self.deserialize_uint(u8::MAX)? as usize;
        visitor.visit_map(SbofMap::new(self, len_left))
    }

    fn deserialize_struct<V>(
        self,
        _: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_map(SbofStruct::new(self, fields))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_enum(&mut SbofEnum::new(self))
    }

    fn deserialize_identifier<V>(self, _: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::Unsupported {
            name: "deserialize_identifier",
            reason: "SBOF is not a self-describing format",
        })
    }

    fn deserialize_ignored_any<V>(self, _: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::Unsupported {
            name: "deserialize_ignored_any",
            reason: "SBOF is not a self-describing format",
        })
    }
}

struct SbofSeq<'a, 'de> {
    de: &'a mut Deserializer<'de>,
    len_left: usize,
}

impl<'a, 'de> SbofSeq<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>, len_left: usize) -> Self {
        SbofSeq { de, len_left }
    }
}

impl<'a, 'de> de::SeqAccess<'de> for SbofSeq<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.len_left == 0 {
            return Ok(None);
        }
        self.len_left -= 1;
        seed.deserialize(&mut *self.de).map(Some)
    }
}

struct SbofMap<'a, 'de> {
    de: &'a mut Deserializer<'de>,
    len_left: usize,
}

impl<'a, 'de> SbofMap<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>, len_left: usize) -> Self {
        SbofMap { de, len_left }
    }
}

impl<'a, 'de> de::MapAccess<'de> for SbofMap<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        if self.len_left == 0 {
            return Ok(None);
        }
        self.len_left -= 1;
        seed.deserialize(&mut *self.de).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }
}

struct SbofStruct<'a, 'de> {
    de: &'a mut Deserializer<'de>,
    fields: &'static [&'static str],
    cursor: usize,
}

impl<'a, 'de> SbofStruct<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>, fields: &'static [&'static str]) -> Self {
        SbofStruct {
            de,
            fields,
            cursor: 0,
        }
    }
}

impl<'a, 'de> de::MapAccess<'de> for SbofStruct<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        if self.cursor >= self.fields.len() {
            Ok(None)
        } else {
            // Pass the field name to the seed's deserialize method
            let field = self.fields[self.cursor];
            self.cursor += 1;
            seed.deserialize(field.into_deserializer()).map(Some)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }
}

struct SbofEnum<'a, 'de> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> SbofEnum<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        SbofEnum { de }
    }
}

impl<'a, 'de> de::EnumAccess<'de> for &'a mut SbofEnum<'a, 'de> {
    type Error = Error;
    type Variant = &'a mut SbofEnum<'a, 'de>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: de::DeserializeSeed<'de>,
    {
        // Read the variant index from the input
        let idx = self.de.deserialize_uint(4)? as u32;
        let val = seed.deserialize::<U32Deserializer<Error>>(idx.into_deserializer())?;
        Ok((val, self))
    }
}

impl<'a, 'de> de::VariantAccess<'de> for &'a mut SbofEnum<'a, 'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(SbofSeq::new(&mut *self.de, len))
    }

    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_map(SbofStruct::new(&mut *self.de, fields))
    }
}

#[test]
fn integer_test() -> Result<()> {
    assert_eq!(from_bytes_testing::<i16>(&[0x01, 0x05])?, 5);
    assert_eq!(from_bytes_testing::<u16>(&[0x01, 0x05])?, 5);
    assert_eq!(from_bytes_testing::<u16>(&[0x02, 0x01, 0x02])?, 513);
    assert_eq!(from_bytes_testing::<i32>(&[0x01, 0x02])?, 2);
    assert_eq!(from_bytes_testing::<u32>(&[0x01, 0x02])?, 2);
    assert_eq!(from_bytes_testing::<i64>(&[0x01, 0x02])?, 2);
    assert_eq!(from_bytes_testing::<u64>(&[0x01, 0x02])?, 2);
    assert_eq!(from_bytes_testing::<i128>(&[0x01, 0x02])?, 2);
    assert_eq!(from_bytes_testing::<u128>(&[0x01, 0x02])?, 2);

    Ok(())
}

#[test]
fn float_test() -> Result<()> {
    assert_eq!(from_bytes_testing::<f32>(&[0x00, 0xff])?, 0.5);
    assert_eq!(from_bytes_testing::<f32>(&[0x01, 0x02, 0x02])?, 5.0);
    assert_eq!(from_bytes_testing::<f32>(&[0xfe, 0x02])?, -5.0);
    assert_eq!(from_bytes_testing::<f32>(&[0x00, 0xfe])?, 0.25);
    assert_eq!(from_bytes_testing::<f32>(&[0x00, 0x01])?, 2.0);
    assert_eq!(from_bytes_testing::<f32>(&[0x00, 0x00])?, 1.0);
    assert_eq!(from_bytes_testing::<f32>(&[0x05, 0x05])?, 52.0);
    assert_eq!(
        from_bytes_settings::<f32>(&[0x31, 0x08, 0x64, 0x40], 0, true)?,
        3.563f32
    );

    assert_eq!(from_bytes_testing::<f64>(&[0x00, 0xff])?, 0.5);
    assert_eq!(from_bytes_testing::<f64>(&[0x01, 0x02, 0x01, 0x02])?, 5.0);
    assert_eq!(from_bytes_testing::<f64>(&[0xfe, 0x01, 0x02])?, -5.0);
    assert_eq!(from_bytes_testing::<f64>(&[0x00, 0xfe])?, 0.25);
    assert_eq!(from_bytes_testing::<f64>(&[0x00, 0x01, 0x01])?, 2.0);
    assert_eq!(from_bytes_testing::<f64>(&[0x00, 0x00])?, 1.0);
    assert_eq!(from_bytes_testing::<f64>(&[0x01, 0x05, 0x05])?, 52.0);
    assert_eq!(from_bytes_testing::<f64>(&[0x07, 0x9b, 0x0b, 0xe7, 0xbd, 0xe2, 0x0d, 0x0f, 0xfd])?, 0.2313554863585172f64);

    Ok(())
}

#[test]
fn enum_test() -> Result<()> {
    #[derive(serde::Deserialize, PartialEq, Debug)]
    enum Testing {
        One(u32),
        Two(u64),
        Struct {
            field1: usize,
            field2: u32,
            field3: i8,
        },
        Unit,
    }

    assert_eq!(from_bytes_testing::<Testing>(&[0x01, 0x03])?, Testing::Unit);
    assert_eq!(
        from_bytes_testing::<Testing>(&[0x01, 0x02, 0x00, 0x05, 0xff])?,
        Testing::Struct {
            field1: 0,
            field2: 5,
            field3: -1,
        }
    );
    assert_eq!(
        from_bytes_testing::<Testing>(&[0x01, 0x01, 0x32])?,
        Testing::Two(50)
    );
    assert_eq!(
        from_bytes_testing::<Testing>(&[0x00, 0x03, 0x01, 0x00, 0x00])?,
        Testing::One(1)
    );

    Ok(())
}
