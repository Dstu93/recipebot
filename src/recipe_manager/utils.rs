
extern crate base64;

use self::base64::{decode,DecodeError};

/// adds a line break to the string
pub fn new_line(s : &mut String){
    s.push_str("\n");
}

/// encodes a given base64 string
pub fn base64_decode(s: &String) -> Result<Vec<u8>, DecodeError> {
    decode(s)
}