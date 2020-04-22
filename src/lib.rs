use std::path::Path;

#[non_exhaustive]
pub struct Metadata{

}

pub fn discombobulate<F: AsRef<Path>>(
    file: F,
    chunk_len: u64,
    transform: fn([u8]) -> (Option<[u8]>, Option<Metadata>)) -> (){

}
