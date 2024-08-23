use core::str;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct ChunkType {
    pub type_code: [u8; 4], // Chunk Type: A 4-byte chunk type code
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.type_code
    }

    fn is_critical(&self) -> bool {
        // Ancillary bit: bit 5 of first byte
        // 0 (uppercase) = critical, 1 (lowercase) = ancillary.
        let bytes_type_code = self.bytes();
        // let ancillary_bit = bytes_type_code[0] >> 4 & 1;
        // Uppercase- true, lowercase- false
        if bytes_type_code[0].is_ascii_uppercase() {
            true
        } else {
            false
        }
    }
    fn is_public(&self) -> bool {
        // Private bit: bit 5 of second byte
        // 0 (uppercase) = public, 1 (lowercase) = private.
        let bytes_type_code = self.bytes();
        // Uppercase- true, lowercase- false
        if bytes_type_code[1].is_ascii_uppercase() {
            true
        } else {
            false
        }
    }
    fn is_reserved_bit_valid(&self) -> bool {
        // Reserved bit: bit 5 of third byte
        // Must be 0 (uppercase) in files conforming to this version of PNG.
        let bytes_type_code = self.bytes();
        // Uppercase- true, lowercase- false
        if bytes_type_code[2].is_ascii_uppercase() {
            true
        } else {
            false
        }
    }
    fn is_safe_to_copy(&self) -> bool {
        // Safe-to-copy bit: bit 5 of fourth byte
        // 0 (uppercase) = unsafe to copy, 1 (lowercase) = safe to copy.
        let bytes_type_code = self.bytes();
        // Uppercase- false, lowercase- true
        if bytes_type_code[3].is_ascii_uppercase() {
            false
        } else {
            true
        }
    }
    fn is_valid(&self) -> bool {
        self.bytes().is_ascii()
            && self.is_reserved_bit_valid()
            && self.bytes()[0].is_ascii_alphabetic()
            && self.bytes()[1].is_ascii_alphabetic()
            && self.bytes()[2].is_ascii_alphabetic()
            && self.bytes()[3].is_ascii_alphabetic()
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        // https://doc.rust-lang.org/std/convert/trait.TryFrom.html
        let chunk = Self { type_code: value };

        Ok(chunk)
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err("String must be exactly 4 characters long");
        }
        if !s.as_bytes()[0].is_ascii_alphabetic()
            || !s.as_bytes()[1].is_ascii_alphabetic()
            || !s.as_bytes()[2].is_ascii_alphabetic()
            || !s.as_bytes()[3].is_ascii_alphabetic()
        {
            return Err("String not completely alphabetic");
        }
        let bytes = s.trim().as_bytes();
        let bytes_array: [u8; 4] = [bytes[0], bytes[1], bytes[2], bytes[3]];
        ChunkType::try_from(bytes_array).map_err(|_| "Failed to convert to ChunkType")
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", str::from_utf8(&self.type_code).unwrap().trim())
    }
}

impl PartialEq for ChunkType {
    fn eq(&self, other: &Self) -> bool {
        self.type_code == other.type_code
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
        // Test for impl Display
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
