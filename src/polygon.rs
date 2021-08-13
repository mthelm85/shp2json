use byteorder::{ LittleEndian, ReadBytesExt };

use crate::shapes;

pub fn polygon(b: &std::vec::Vec<u8>, end: u64) -> shapes::Shape {
    let num_parts = (&b[36..40]).read_i32::<LittleEndian>().unwrap();
    let num_points = (&b[40..44]).read_i32::<LittleEndian>().unwrap();
    let mut parts: Vec<i32> = Vec::with_capacity(num_parts as usize);
    let points_start = 44 + (4 * num_parts);
    let mut points: Vec<shapes::Point> = Vec::with_capacity(num_points as usize);

    for p in (44..points_start).step_by(4) {
        parts.push((&b[p as usize..(p + (4 * num_parts)) as usize]).read_i32::<LittleEndian>().unwrap())
    }

    for p in (points_start..end as i32).step_by(16) {
        let point = shapes::Point {
            x: ((&b[p as usize..(p + 8) as usize]).read_f64::<LittleEndian>().unwrap()),
            y: ((&b[(p + 8) as usize..(p + 16) as usize]).read_f64::<LittleEndian>().unwrap())
        };
        points.push(point)
    }

    shapes::Shape::Polygon {
        bbox_xmin: (&b[4..12]).read_f64::<LittleEndian>().unwrap(),
        bbox_ymin: (&b[12..20]).read_f64::<LittleEndian>().unwrap(),
        bbox_xmax: (&b[20..28]).read_f64::<LittleEndian>().unwrap(),
        bbox_ymax: (&b[28..36]).read_f64::<LittleEndian>().unwrap(),
        num_parts,
        num_points,
        parts,
        points
    }
}