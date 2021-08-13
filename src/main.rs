use std::fs::File;
use std::io::{ Error };
use std::vec::Vec;
use std::convert::TryInto;

mod main_header;
mod polygon;
mod record_header;
mod record;
mod shapes;

fn main() -> Result<(), Error> {
    let filepath = "C:\\Users\\mthel\\Julia\\src_data\\Map Shape Files\\BLM_CO_ST_BNDY_SHP\\BLM_CO_ST_BNDY_20170109.shp";
    let file = File::open(&filepath)?;

    let header = main_header::read_header(&file).unwrap();
    let mut records: Vec<record::Record> = Vec::new();
    let mut start: u64 = 100;
    
    while start < (header.file_length * 2).try_into().unwrap() {
        let rcrd = record::read_record(&file, start).unwrap();
        let len: u64 = rcrd.header.content_length.try_into().unwrap();
        records.push(rcrd);
        start = start + (len * 2) + 8;
    }
    println!("{:#?}", records[0]);
    Ok(())
}