# SBOF: Small Binary Object Format
SBOF is a binary object format that I made because I didn't like any of the other formats. It's not self-describing, so you can't deserialize data without knowing what format it is. This repo is an implementation of SBOF for rust, using serde. If the following specification is too confusing, feel free to open an issue (or a PR if you think you know what it is, and feel like fixing it), and I will respond as soon as possible.

## Boolean
`true: 01`<br>
`false: 00`

## 8-bit Integers
8-bit integers are stored as 1 byte.

## Unsigned Integers Larger than 8 Bits
Unsigned integers larger than 8 bits are stored as little-endian variable-length integers. The length of each integer can be determined by a byte previous to the integer. However. this byte can be ommitted in the length of the integer is 1 byte, and the value of the integer is not between 1, and the amount of bytes used in the integer type. The length can be made smaller by removing trailing zeros from the number.

## Signed Integers Larger than 8 Bits
Signed integers larger than 8 bits are stored in a similar way to unsigned integers. However, the number can only be made smaller by removing trailing zeros if the number is positive. If the number is negative, trailing 0xff's should be removed instead.

## Floating Point Values
Floating point values are stored by a transformed version of their mantissa and significand. In order to serialize a number to SBIF, convert the mantissa to a signed two's complement format (size dependant on mantissa size), and reverse the bits of the significand. Then, store the significand, then mantissa, in that order. The sizes of the values are dependant on the size of the values in the floating point value format you are using. For IEEE 754 Single-Precision values, the mantissa is a signed 8-bit integer, and the significand is an unsigned 32 bit integer. For IEEE 754 Double-Precision values, the mantissa is a signed 16-bit integer

## Characters
Characters are stored in a rather convoluted format.
Each character can be in a different format based on it's length in UTF-8.
If the character is 1 byte long and it's value is not in between 1 and 4 (inclusive), then it's encoded as it's value.
If the character is 1 byte long and it's value is in between 1 and 4 (inclusive), then it's encoded as it's length (1 byte) stored as a `u8`, and then it's value.
If the character is more than 1 byte long, then it's encoded as it's length stored as a `u8`, and then it's value.<br><br>

TL; DR;<br>
Characters are stored with their length in bytes before the actual bytes, unless the length is 1 byte and the value is not 1-4 (inclusive).

## Strings
Strings are stored in a similar format to characters.
Each string is length-prefixed by a variable-length integer. The length of that integer is decided by a single byte before the length. However, if the length of the string is 1-byte long, and has a value of 0, or greater than 5, the integer length byte can be ommited.
