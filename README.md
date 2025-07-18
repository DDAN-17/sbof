# SBOF: Small Binary Object Format
SBOF is a binary object format that I made because I didn't like any of the other formats. It's not self-describing, so you can't deserialize data without knowing what format it is. This repo is an implementation of SBOF for rust, using serde. If the following specification is too confusing, feel free to open an issue (or a PR if you think you know what it is, and feel like fixing it), and I will respond as soon as possible.

## Header
Every SBOF stream starts with a small header. The header consists of a single unsigned byte of the version. The version this specification specificies is version 0. Following the version is another unsiged byte, this time for "feature flags." Since the creator of SBOF is indecicive, she decided to leave some decisions up to the user. Currently, the feature flags (from LSB to MSB in the feature flag byte), are as follows:
### High Precision Floats
The "High Precision Floats" feature flags turns off SBOF encoding for floating point values. This should be enabled if floats more precise than 2 decimal places are being serialized regularly.


## Boolean
`true: 01`<br>
`false: 00`

## 8-bit Integers
8-bit integers are stored as 1 byte.

## Unsigned Integers Larger than 8 Bits
Unsigned integers larger than 8 bits are stored as little-endian variable-length integers. The length of each integer can be determined by a byte previous to the integer. However. this byte can be ommitted if the length of the integer is 1 byte, and the value of the integer is not between 1, and the amount of bytes used in the integer type. The length can be made smaller by removing trailing zeros from the number.

## Signed Integers Larger than 8 Bits
Signed integers larger than 8 bits are stored in a similar way to unsigned integers. However, the number can only be made smaller by removing trailing zeros if the number is positive. If the number is negative, trailing 0xff's should be removed instead.

## Floating Point Values
Floating point values are stored by a transformed version of their mantissa and significand. In order to serialize a number to SBOF, convert the mantissa to a signed two's complement format (size dependant on mantissa size), and reverse the bits of the significand. Make sure to negate th significand based on the sign bit. Then, store the significand, then mantissa, in that order. The sizes of the values are dependant on the size of the values in the floating point value format you are using. For IEEE 754 Single-Precision values, the mantissa is a signed 8-bit integer, and the significand is a signed 32 bit integer. For IEEE 754 Double-Precision values, the mantissa is a signed 16-bit integer, and the significand is a signed 64-bit integer.<br><br>

If the "High Precision Floats" feature flag is enabled, the previous paragraph can be ignored. Instead, the floats are serialized as standard IEEE 754 floats. This should only be enabled if high precision floats are being serialized regularly, since this turns off what is supposed to be an optimization. In this case, "high precision" means about 3 decimal places.

## Characters
Characters are stored as unsigned 32-bit integers, through UTF-32.

## Strings
Strings are stored as length prefixed UTF-8. The length is stored as an infinitely sized unsigned integer[^1].

## Byte Arrays
Byte arrays are stored in the same way as strings. They are stored as raw bytes, prefixed by a length as an infinitely sized unsigned integer[^1].

## Optional Values
Optional values are stored as a value prefixed by a boolean. However, if the value doesn't start with a zero or a one, and there is a value, then the boolean can be ommitted.

## ZSTs (Zero Sized Types)
Zero Sized Types are not serialized.

## Enumerations
Enumerations are stored as their index as an infinitely sized unsigned integer, followed by their data if any.

## Structures
Structures are stored as their data in a constant order.

## Sequences
Sequences (any type wrapping a variable amount of elements) are stored as the values, all prefixed by the amount of elements as an infinitely sized unsigned integer[^1].

## Tuples
Tuples (any type wrapping a constant amount of elements) are stored as the values, without being prefixed by the amount of elements. since the amount of elements is defined by the structure being serialized.

## Maps
Maps are stored as an array of key-value pairs, prefixed by their length in pairs as an infinitely sized integer[^1].

[^1]: An infinitely sized integer is an integer with no upper bound to it's size. However, the size is bounded by the maximum value a byte can store, so there actually is an upper limit.
