use std::io::prelude::*;

use byteorder::{ BigEndian, LittleEndian, ReadBytesExt };

#[derive(Debug)]
pub struct MainHeader {
    pub file_length: i32,
    pub version: i32,
    pub shape_type: i32,
    pub bbox_xmin: f64,
    pub bbox_ymin: f64,
    pub bbox_xmax: f64,
    pub bbox_ymax: f64,
}

pub fn read_header(mut file: &std::fs::File) -> Result<MainHeader, std::io::Error> {
    let mut buffer = [0; 100];

    file.read_exact(&mut buffer)?;

    Ok(MainHeader {
        file_length:  (&buffer[24..28]).read_i32::<BigEndian>().unwrap(),
        version: (&buffer[28..32]).read_i32::<LittleEndian>().unwrap(),
        shape_type: (&buffer[32..36]).read_i32::<LittleEndian>().unwrap(),
        bbox_xmin: (&buffer[36..44]).read_f64::<LittleEndian>().unwrap(),
        bbox_ymin: (&buffer[44..52]).read_f64::<LittleEndian>().unwrap(),
        bbox_xmax: (&buffer[52..60]).read_f64::<LittleEndian>().unwrap(),
        bbox_ymax: (&buffer[60..68]).read_f64::<LittleEndian>().unwrap(),
    })
}
