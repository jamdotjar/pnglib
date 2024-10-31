use std::{error::Error, fmt::Display, u32};

use crate::chunk_type::ChunkType;

#[derive(Debug)]
pub struct Chunk {
    data_length: u32,
    chunk_type: ChunkType,
    crc: u32,
    chunk_data: Vec<u8>,
}
#[derive(Debug)]
pub enum ChunkError {
    InvalidLength,
    InvalidCrc,
    InvalidChunkType,
}
impl Display for ChunkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ChunkError {}
impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let data_length = data.len() as u32;
        let crc: u32 = 4;
        let chunk_data = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.bytes.iter())
            .chain(data.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk {
            data_length: data.len() as u32,
            chunk_type,
            crc: u32::MAX,
            chunk_data,
        }
    }

    pub fn length(&self) -> u32 {
        todo!()
    }

    pub fn chunk_type(&self) -> &ChunkType {
        todo!()
    }

    pub fn data(&self) -> &[u8] {
        todo!()
    }

    pub fn crc(&self) -> u32 {
        let data: Vec<u8> = self
            .chunk_type
            .bytes
            .iter()
            .cloned()
            .chain(self.chunk_data.iter().cloned())
            .collect();
        let crc32 = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        crc32.checksum(&data)
    }

    pub fn data_as_string(&self) -> Result<String, String> {
        todo!()
    }

    /// Returns this chunk as a byte sequences described by the PNG spec.
    /// The following data is included in this byte sequence in order:
    /// 1. Length of the data *(4 bytes)*
    /// 2. Chunk type *(4 bytes)*
    /// 3. The data itself *(`length` bytes)*
    /// 4. The CRC of the chunk type and data *(4 bytes)*
    pub fn as_bytes(&self) -> Vec<u8> {
        todo!()
    }
}


impl TryFrom<&[u8]> for Chunk {
    type Error = ChunkError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 12 {
            return Err(ChunkError::InvalidLength);
        }

        let (length, bytes) = bytes.split_at(4);
        let data_length =
            u32::from_be_bytes(length.try_into().map_err(|_| ChunkError::InvalidLength)?);

        if bytes.len() < (data_length as usize + 8) {
            return Err(ChunkError::InvalidLength);
        }

        let (chunk_type, bytes) = bytes.split_at(4);
        let (data, crc) = bytes.split_at(data_length as usize);
        let crc = crc.get(..4).ok_or(ChunkError::InvalidCrc)?;

        let chunk_type_bytes: [u8; 4] = chunk_type
            .try_into()
            .map_err(|_| ChunkError::InvalidChunkType)?;
        let chunk_type =
            ChunkType::try_from(chunk_type_bytes).map_err(|_| ChunkError::InvalidChunkType)?;

        let crc = u32::from_be_bytes(crc.try_into().map_err(|_| ChunkError::InvalidCrc)?);

        Ok(Chunk {
            data_length,
            chunk_type,
            crc,
            chunk_data: data.to_vec(),
        })
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self);
        Ok(())
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