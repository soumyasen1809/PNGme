use core::str;
use std::fmt::Display;

use crate::chunk_type::ChunkType;

const ALGORITHM_CRC: crc::Algorithm<u32> = crc::CRC_32_ISO_HDLC; // This algorithm is the one for the Unit Tests

#[derive(Debug)]
pub struct Chunk {
    data_length: u32,
    chunk_type: ChunkType,
    message_bytes: Vec<u8>,
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let crc = crc::Crc::<u32>::new(&ALGORITHM_CRC);
        let data_to_crc: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect();
        let crc_val = crc.checksum(&data_to_crc);

        Self {
            data_length: data.len() as u32,
            chunk_type,
            message_bytes: data,
            crc: crc_val,
        }
    }
    pub fn length(&self) -> u32 {
        self.data_length
    }
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    pub fn data(&self) -> &[u8] {
        &self.message_bytes
    }
    pub fn crc(&self) -> u32 {
        self.crc
    }
    pub fn data_as_string(&self) -> Result<String, ()> {
        Ok(str::from_utf8(&self.message_bytes).unwrap().to_string())
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        (self.message_bytes.len() as u32) // as u32 needed else it returns [u8;8] instead of [u8;4]
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.message_bytes.iter())
            .chain(self.crc().to_be_bytes().iter())
            .cloned()
            .collect()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = &'static str;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let (split_data_length, remaining_after_length) = value.split_at(4);
        let data_length = u32::from_be_bytes(
            // passes with be, fails with le
            split_data_length
                .try_into()
                .expect("slice with incorrect length"),
        ); // convert from &[u8] to u32 in rust

        let (split_type_code, remaining_after_type) = remaining_after_length.split_at(4);
        let chunk_type = ChunkType {
            type_code: [
                split_type_code[0],
                split_type_code[1],
                split_type_code[2],
                split_type_code[3],
            ],
        };

        let (split_message_bytes, remaining_after_data) =
            remaining_after_type.split_at(data_length.try_into().unwrap());
        let message_bytes: Vec<u8> = split_message_bytes.to_vec();

        if remaining_after_data.len() < 4 {
            return Err("Wrong data length");
        }

        let (split_crc, _remaining_after_crc) = remaining_after_data.split_at(4);
        let crc = crc::Crc::<u32>::new(&ALGORITHM_CRC);
        let data_to_crc: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .chain(message_bytes.iter())
            .copied()
            .collect();
        let crc_val_computed = crc.checksum(&data_to_crc);

        let crc_val_from_bytes = u32::from_be_bytes(split_crc.try_into().expect("Crc Error"));
        // The CRC from the calculation and from the last 4 bytes should match
        if crc_val_computed != crc_val_from_bytes {
            return Err("Crc values do not match");
        };

        let chunk = Chunk {
            data_length, // convert from &[u8] to u32 in rust
            chunk_type,
            message_bytes,
            crc: crc_val_from_bytes,
        };

        Ok(chunk)
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;
    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;
        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }
    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }
    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }
    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }
    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }
    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }
    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;
        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }
    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;
        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        let chunk = Chunk::try_from(chunk_data.as_ref());
        assert!(chunk.is_err());
    }
    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;
        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        let _chunk_string = format!("{}", chunk);
    }
}
