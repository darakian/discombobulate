use std::path::Path;

pub fn discombobulate<F: AsRef<Path>>(file: F, chunk_len: u64, transform: fn([u8]) -> [u8]) -> (){

}
