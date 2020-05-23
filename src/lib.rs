use std::path::Path;
use std::fs::File;
use std::io::{Read, self};

// Use 1MB buffer for reads
const CHUNK_SIZE: usize = 1024 *1024;

#[non_exhaustive]
pub struct Metadata{

}

pub struct ChunkIter{
    f: File,
    chunk_len: u64,
    buffer: [u8; CHUNK_SIZE],
}

impl ChunkIter{
    pub fn new(f: File, len: u64) -> Self{
        ChunkIter{f: f, chunk_len: len, buffer: [0;CHUNK_SIZE]}
    }
}

impl Iterator for ChunkIter{
    type Item = Result<Vec<u8>, io::Error>;
    fn next(&mut self) -> Option<Result<Vec<u8>, io::Error>>{
        match self.f.by_ref()
            .take(self.chunk_len)
            .read(&mut self.buffer){
            Ok(i) => {
                if i == 0 {
                    return None
                } else {
                    Some(Ok(self.buffer[..i].to_vec()))}
                },
            Err(e) => Some(Err(e))
        }
    }
}

pub fn discombobulate<F: AsRef<Path>>(file: F, chunk_len: u64) -> Result<Box<dyn Iterator<Item = Result<Vec<u8>, io::Error>>>, io::Error>{
    match File::open(file){
        Err(e) => {return Err(e)}
        Ok(mut f) => {
            return Ok(Box::new(ChunkIter::new(f, chunk_len)))
        }
    }
}
