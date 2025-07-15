# SBOF: Small Binary Object Format
SBOF is a binary object format that I made because I didn't like any of the other formats. It's not self-describing, so you can't deserialize data without knowing what format it is. If the following specification is too confusing, feel free to open an issue, and I will respond as soon as possible.

## Boolean
`true: 01`<br>
`false: 00`

## Integers and Floats
All integers and floats are stored in Little Endian format.

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