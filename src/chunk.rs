// Chunk Layout:
//
// 1) Length - 4 bytes Integer: indicate the length og Chunk Data
// 2) Chunk Type - 4 bytes (A-Z and a-z, or 65-90 and 97-122 decimal)
// 3) Chunk Data - Data appropriate to the type (can be 0)
// 4) CRC - calculated for the type and data (always will be present)

use std::fmt;
use crc::{Crc, CRC_32_ISO_HDLC};
use crate::chunk_type::{ChunkType, ParseChunkTypeError};

pub const CRC_ALGORITHM: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

pub const CRC_SIZE: usize = 4;
pub const CHUNK_TYPE_SIZE: usize = 4;
pub const CHUNK_LEN_SIZE: usize = 4;

// Table of CRCs of all 8-bit messages.
#[derive(Debug, Clone)]
pub struct Chunk {
    len : u32,
    chunk_type : ChunkType,
    data : Vec<u8>,
    crc : u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseChunkDataError;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseChunkError;

impl TryFrom<&[u8]> for Chunk {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < CHUNK_LEN_SIZE + CHUNK_TYPE_SIZE + CRC_SIZE {
            Err("Provided bytes are less than the minimum size of 1 Chunk which is 16!")
        }
        else {
            let mut data = value;

            let chunk_len = read_be_u32(&mut data);
            // if data.len() - (CRC_SIZE + CHUNK_TYPE_SIZE) != chunk_len as usize {
            //     return Err("The length provided does not correspond to the actual length of bytes passed!");
            // }
            
            let res_chunk_type : ChunkType;
            match read_chunk_type(&mut data) {
                Ok(chunk_type) => res_chunk_type = chunk_type,
                Err(_) => return Err("Error in parsing chunk type!"),
            };

            let chunk_data = read_chunk_data(chunk_len as usize, &mut data);
            let chunk_crc = read_be_u32(&mut data);

            if chunk_crc != CRC_ALGORITHM.checksum(&[res_chunk_type.bytes().as_slice(), &chunk_data].concat()) {
                return Err("CRC provided is not correct!");
            }

            Ok(Chunk {len: chunk_len,
                      chunk_type: res_chunk_type,
                      data: chunk_data,
                      crc: chunk_crc})
        }
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Chunk (len: {}, chunk_type: {}, data: {:?}, crc: {})",
                self.len, self.chunk_type, self.data, self.crc)
    }
}

impl Chunk {

    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        Chunk { len : data.len() as u32,
                crc : CRC_ALGORITHM.checksum(&[chunk_type.bytes().as_slice(), &data].concat()),
                chunk_type : chunk_type,
                data: data,
        }
    }

    pub fn length(&self) -> u32 { self.len }
    pub fn chunk_type(&self) -> &ChunkType { &self.chunk_type }
    pub fn data(&self) -> &[u8] { &self.data }
    pub fn crc(&self) -> u32 { self.crc }

    pub fn data_as_string(&self) -> Result<String, ParseChunkDataError> {
        let res = String::from_utf8(self.data.clone());

        match res {
            Ok(ref _string) => return Ok(res.unwrap()),
            Err(_e) => return Err(ParseChunkDataError),
        };
    }

    pub fn as_bytes(&self) -> Vec<u8> {
         self.length().to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }

}

fn read_be_u32(input: &mut &[u8]) -> u32 {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<u32>());
    *input = rest;
    u32::from_be_bytes(int_bytes.try_into().unwrap())
}

fn read_chunk_type(input: &mut &[u8]) -> Result<ChunkType, ParseChunkTypeError> {
    let (chunk_bytes, rest) = input.split_at(CHUNK_TYPE_SIZE);
    *input = rest;

    let fixed_chunk_bytes: [u8; 4] = chunk_bytes.try_into().unwrap();
    let tmp = ChunkType::try_from(fixed_chunk_bytes);

    match tmp {
        Ok(result) => Ok(result),
        Err(_) => Err(ParseChunkTypeError),
    }
}

fn read_chunk_data(read_size: usize, input: &mut &[u8]) -> Vec<u8> {
    let (data_bytes, rest) = input.split_at(read_size);
    *input = rest;
    data_bytes.to_owned()
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
        let data = "This is where your secret message will be!".as_bytes().to_vec();
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


//
//[src/chunk.rs:55] value.clone() = [
//     0,
//     0,
//     0,
//     42,
//     82,
//     117,
//     83,
//     116,
//     84,
//     104,
//     105,
//     115,
//     32,
//     105,
//     115,
//     32,
//     119,
//     104,
//     101,
//     114,
//     101,
//     32,
//     121,
//     111,
//     117,
//     114,
//     32,
//     115,
//     101,
//     99,
//     114,
//     101,
//     116,
//     32,
//     109,
//     101,
//     115,
//     115,
//     97,
//     103,
//     101,
//     32,
//     119,
//     105,
//     108,
//     108,
//     32,
//     98,
//     101,
//     33,
//     171,
//     209,
//     216,
//     78,
// ]
// [src/chunk.rs:63] chunk_len.clone() = 42
