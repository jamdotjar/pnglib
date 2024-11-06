use std::{fs, path::Path, str::FromStr};

use crate::{
    chunk::{self, Chunk},
    chunk_type::ChunkType,
    png::Png,
    Error, Result,
};

/*
pngme encode ./dice.png ruSt "This is a secret message!

pngme decode ./dice.png ruSt

pngme remove ./dice.png ruSt

pngme print ./dice.png
*/

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
