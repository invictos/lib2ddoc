use std::{fs::File, io::BufReader};

use r2ddoc::PackOrder;

pub struct Image {
    pub pixels: Vec<u8>,
    pub width: u16,
    pub height: u16,
    pub format: PackOrder,
}

pub trait ImageFileDecoder{
    fn decode(path: &str) -> Image;
    
    fn read_file(path: &str) -> BufReader<File>{
        let file = File::open(path).expect("Error, cannot open the file");
        BufReader::new(file)
    }
}


/*
    To implement a new decoder, add a module here, and implement the ImageFileDecoder trait on a struct. 
    Convention: Name the struct Decoder, and always reference it with decoder::<mod>::Decoder
*/
pub mod jpeg;

pub mod png;