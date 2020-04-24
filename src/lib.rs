use std::path::Path;
use std::fs::File;
use std::io::{Read, self};

const CHUNK_SIZE: usize = 4 * 1024 *1024;

#[non_exhaustive]
pub struct Metadata{

}

struct ChunkIter{
    f: File,
    buffer: [u8; CHUNK_SIZE],
}

impl ChunkIter{
    pub fn new(f: File) -> Self{
        ChunkIter{f: f, buffer: [0;CHUNK_SIZE]}
    }
}

impl Iterator for ChunkIter{
    type Item = Result<Box<[u8]>, io::Error>;
    fn next(&mut self) -> Option<Result<Box<[u8]>, io::Error>>{
        match self.f.read(&mut self.buffer){
            Ok(_i) => Some(Ok(Box::new(self.buffer.clone()))),
            Err(e) => Some(Err(e))
        }
    }
}

pub fn discombobulate<F: AsRef<Path>>(
    file: F,
    chunk_len: u64,
    transform: fn(Box<[u8]>) -> (Option<Box<[u8]>>, Option<Metadata>)) -> (){

}

pub fn identity_transform(input: Box<[u8]>) -> (Option<Box<[u8]>>, Option<Metadata>){
    return (Some(input), None)
}

fn flay<F: AsRef<Path>>(file: F, chunk_len: u64) -> Result<Box<dyn Iterator<Item = Result<Box<[u8]>, io::Error>>>, io::Error>{
    match File::open(file){
        Err(e) => {return Err(e)}
        Ok(mut f) => {
            return Ok(Box::new(ChunkIter::new(f)))
        }
    }
}
