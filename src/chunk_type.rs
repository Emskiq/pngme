use std::{fmt,  str::FromStr, error::Error};

// Chunk Type consisting of 4 bytes (A-Z or a-z)
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ChunkType {
    byte1: char, // Ancillary bit
    byte2: char, // Private bit
    byte3: char, // Reserved bit
    byte4: char, // Safe to copy bit
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseChunkTypeError;

impl Error for ParseChunkTypeError { }

impl fmt::Display for ParseChunkTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error in parsing chunk type")
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        if (value[0] as char).is_ascii_alphabetic() && (value[1] as char).is_ascii_alphabetic() &&
            (value[2] as char).is_ascii_alphabetic() && (value[3] as char).is_ascii_alphabetic() {
                Ok(ChunkType {byte1: value[0] as char, byte2: value[1] as char,
                              byte3: value[2] as char, byte4: value[3] as char})
        }
        else {
            Err("ChunkType only accepts array of A-Z or a-z chars/u8 bytes (65-90 or 97-122)")
        }
    }
}

impl FromStr for ChunkType {
    type Err = ParseChunkTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 4 {
            if s.is_ascii() {
                let byte_array: [u8; 4] = s.as_bytes().try_into().unwrap();
                let tmp = ChunkType::try_from(byte_array);
                match tmp {
                    Ok(res) => return Ok(res),
                    Err(_e) => return Err(ParseChunkTypeError),
                };
            }
        }
        Err(ParseChunkTypeError)
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}{}", self.byte1, self.byte2, self.byte3, self.byte4)
    }
}

// Custom functions for ChunkType
impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        [self.byte1 as u8, self.byte2 as u8, self.byte3 as u8, self.byte4 as u8]
    }
    
    // ChunkType is considered valid if all of the bytes are represented as
    // ASCII Alphabetic character and also if the 5th bit of the 3rd byte is
    // 0 (or just if the 3rd character is Uppercase)
    fn is_valid(&self) -> bool {
        self.byte1.is_ascii_alphabetic() && self.byte2.is_ascii_alphabetic() &&
            self.byte3.is_ascii_alphabetic() && self.byte4.is_ascii_alphabetic() &&
            self.byte3.is_ascii_uppercase()
    }

    fn is_critical(&self) -> bool {
        self.byte1.is_ascii_uppercase()
    }

    fn is_public(&self) -> bool {
        self.byte2.is_ascii_uppercase()
    }

    fn is_reserved_bit_valid(&self) -> bool {
        self.byte3.is_ascii_uppercase()
    }

    fn is_safe_to_copy(&self) -> bool {
        self.byte4.is_ascii_lowercase()
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
