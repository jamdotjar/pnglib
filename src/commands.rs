use std::{fs, path::Path, str::FromStr};

use crate::{
    chunk::{self, Chunk},
    chunk_type::ChunkType,
    png::Png,
    Error, Result,
};


//Adds a chunk of type 'chunk_type' to
pub fn encode(path: &Path, chunk_type: String, message: String) -> Result<String> {
    let mut png = Png::from_file(path)?;
    let chunk = Chunk::new(
        ChunkType::from_str(&chunk_type)?,
        message.as_bytes().to_vec(),
    );
    png.append_chunk(chunk);
    fs::write(path, png.as_bytes())?;
    Ok("Sucessfully encoded message".to_string())
}
pub fn decode(path: &Path, chunk_type: String) -> Result<String> {
    let png = Png::from_file(path)?;
    let chunk = png.chunk_by_type(&*chunk_type);
 match chunk {
        Some(chunk) => Ok(chunk.data_as_string()?),
        _ => Ok(format!("No chunk with type {} found", chunk_type)),
    }
}

pub fn remove(path: &Path, chunk_type: String) -> Result<String> {
    let mut png = Png::from_file(path)?;
    png.remove_first_chunk(&chunk_type)?;
    fs::write(path, png.as_bytes())?;
    Ok(format!("Sucessfully removed first {} chunk", chunk_type))
}
pub fn print(path: &Path) -> Result<String> {
    let png = Png::from_file(path)?;
    let mut out = "".to_string();
    for chunk in png.chunks() {
        out = format!("out \n{}", chunk);
    }
    Ok(format!("{}", out))
}
