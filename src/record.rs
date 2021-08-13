use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::{ Error, ErrorKind };
use std::convert::TryInto;

use crate::polygon;
use crate::record_header;
use crate::shapes;

use byteorder::{ LittleEndian, ReadBytesExt };

#[derive(Debug)]
pub struct Record {
    pub header: record_header::RecordHeader,
    pub shape_type: i32,
    pub shape_data: shapes::Shape
}

pub fn read_record(mut file: &std::fs::File, start: u64) -> Result<Record, std::io::Error> {
    let header = record_header::read_header(&file, start).unwrap();
    let len: u64 = header.content_length.try_into().unwrap();
    let end = &len * 2;

    let mut buffer = vec![0; end as usize];
    file.seek(SeekFrom::Start(start + 8))?;
    file.read_exact(&mut buffer)?;

    let shape_type = (&buffer[0..4]).read_i32::<LittleEndian>().unwrap();

    match shape_type {
        0 => {
            Ok(Record {
                header,
                shape_type,
                shape_data: shapes::Shape::NullShape
            })
        },
        5 => {
            let shape_data = polygon::polygon(&buffer, end);
            
            Ok(Record {
                header,
                shape_type,
                shape_data
            })
        },
        _ => { Err(Error::new(ErrorKind::Other, "well.....fack!")) }
    }


}