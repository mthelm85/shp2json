#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

#[derive(Debug)]
pub enum Shape {
    NullShape,
    MultiPoint {
        bbox_xmin: f64,
        bbox_ymin: f64,
        bbox_xmax: f64,
        bbox_ymax: f64,
        num_points: i32,
        points: Vec<Point>
    },
    PolyLine {
        bbox_xmin: f64,
        bbox_ymin: f64,
        bbox_xmax: f64,
        bbox_ymax: f64,
        num_parts: i32,
        num_points: i32,
        parts: Vec<i32>,
        points: Vec<Point>
    },
    Polygon {
        bbox_xmin: f64,
        bbox_ymin: f64,
        bbox_xmax: f64,
        bbox_ymax: f64,
        num_parts: i32,
        num_points: i32,
        parts: Vec<i32>,
        points: Vec<Point>
    }
}