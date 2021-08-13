use std::io::prelude::*;
use std::io::SeekFrom;

use byteorder::{ BigEndian, ReadBytesExt };

#[derive(Debug)]
pub struct RecordHeader {
    pub record_number: i32,
    pub content_length: i32
}

pub fn read_header(mut file: &std::fs::File, start: u64) -> Result<RecordHeader, std::io::Error> {
    let mut buffer = [0; 8];
    file.seek(SeekFrom::Start(start))?;
    file.read_exact(&mut buffer)?;
    Ok(RecordHeader {
        record_number: (&buffer[0..4]).read_i32::<BigEndian>().unwrap(),
        content_length: (&buffer[4..8]).read_i32::<BigEndian>().unwrap()
    })
}