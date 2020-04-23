use std::path::Path;
use std::fs;
use std::io;

#[non_exhaustive]
pub struct Metadata{

}

pub fn discombobulate<F: AsRef<Path>>(
    file: F,
    chunk_len: u64,
    transform: fn(Box<[u8]>) -> (Option<Box<[u8]>>, Option<Metadata>)) -> (){

}

pub fn identity_transform(input: Box<[u8]>) -> (Option<Box<[u8]>>, Option<Metadata>){
    return (Some(input), None)
}

fn flay<F: AsRef<Path>>(file: F, chunk_len: u64) -> Box<dyn Iterator<Item = Result<Box<[u8]>, io::Error>>>{
    match fs::File::open(file){
        Err(e) => {return Box::new(vec![e].iter())}
        Ok(mut f) => {}
    }
}
