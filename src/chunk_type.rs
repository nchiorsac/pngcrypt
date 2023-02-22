use std::convert::TryFrom;
use std::str::FromStr;
use std::fmt;

// PNG chunk type
#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType([u8; 4]);

impl ChunkType {
    // Chunk type as bytes
    pub fn bytes(&self) -> [u8; 4] {
        self.0
    }

    // Check if chunk type consists only of uppercase & lowercase ASCII letters
    pub fn is_valid(&self) -> bool {
        for i in 0..3 {
            if !self.bytes()[i].is_ascii_alphabetic() || !self.is_reserved_bit_valid() {
                return false
            }
        }
        true
    }

    // Check if first letter of chunk type is uppercase ASCII, indicating chunk is strictly necessary when displaying file
    pub fn is_critical(&self)-> bool {
        self.bytes()[0].is_ascii_uppercase()
    }

    // Check if second letter of chunk type is uppercase ASCII, indicating chunk is part of PNG specification or registered in list of PNG special-purpose chunk types
    pub fn is_public(&self) -> bool {
        self.bytes()[1].is_ascii_uppercase()
    }

    // Check if third letter of chunk type is uppercase ASCII.
    pub fn is_reserved_bit_valid(&self) -> bool {
        self.bytes()[2].is_ascii_uppercase()
    }

    // Check if fourth letter is lowercase, inidcating chunk is safe to be copied
    pub fn is_safe_to_copy(&self) -> bool {
        self.bytes()[3].is_ascii_lowercase()
    }

}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        for i in 0..3 {
            if !bytes[i].is_ascii_alphabetic() {
                return Err("Chunk type must only consist of ASCII letters.")
            }
        }
        Ok(ChunkType(bytes))
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 4 {
            return Err("Chunk type must be 4 bytes long.");
        }

        for c in s.chars() {
            if !c.is_ascii_alphabetic(){
                return Err("Chunk type must only consist of ASCII letters.")
            }
        }

        let mut bytes = [0; 4];
        bytes.clone_from_slice(s.as_bytes());

        Ok(ChunkType(bytes))
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.0).expect("Error displaying chunk type"))
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