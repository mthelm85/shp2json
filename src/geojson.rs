use serde_json::json;

use crate::record;
use crate::shapes;

pub fn to_geojson(rcrds: &Vec<record::Record>) -> serde_json::Value {
    let mut geometries = Vec::new();

    for r in rcrds {
      
        let geo = json!({
            "type": match r.shape_type {
                5 => "Polygon",
                _ => "Polygon"
            },
            "coordinates": match &r.shape_data {
                shapes::Shape::Polygon {
                    bbox_xmin: _,
                    bbox_ymin: _,
                    bbox_xmax: _,
                    bbox_ymax: _,
                    num_parts: _,
                    num_points: _,
                    parts: _,
                    points
                } => {
                    let mut ps = vec![];
                    for p in points {
                        ps.push([p.x,p.y])
                    }
                    [ps]
                },
                _ => [vec![[1.0,2.0]]]
            }
        });

        geometries.push(geo)
    }

    let json_str = json!({
        "type": "GeometryCollection",
        "geometries": geometries
    });

    json_str

}