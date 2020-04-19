use geo::{Coordinate, LineString, MultiPolygon, Polygon, Point};
use geojson::{Feature, FeatureCollection, GeoJson, Geometry, Value};

use serde_json::{json, Map};

use std::iter::FromIterator;

use super::helper::xy;
use super::compact_geojson::write_compact_geojson;

pub fn generate_rect_centered(center: Coordinate<f64>, w: f64, h: f64) -> Polygon<f64> {
    let w_half = w / 2.0;
    let h_half = h / 2.0;
    Polygon::new(
        LineString(vec![
            xy(center.x - w_half, center.y - h_half),
            xy(center.x + w_half, center.y - h_half),
            xy(center.x + w_half, center.y + h_half),
            xy(center.x - w_half, center.y + h_half),
            xy(center.x - w_half, center.y - h_half),
        ]),
        vec![],
    )
}

pub fn generate_grid(min: f64, max: f64, rect_size: f64, num_rects: i32) -> MultiPolygon<f64> {
    assert!(num_rects >= 2);

    let positions: Vec<_> = (0 .. num_rects)
        .map(|i| min + (max - min) * i as f64 / ((num_rects - 1) as f64))
        .collect();

    let mut polygons = Vec::with_capacity((num_rects * num_rects) as usize);
    for x in &positions {
        for y in &positions {
            polygons.push(generate_rect_centered(Coordinate{x: *x, y: *y}, rect_size, rect_size));
        }
    }

    MultiPolygon(polygons)
}

pub fn convert_to_feature(p: &MultiPolygon<f64>, operation: Option<String>) -> Feature {
    Feature {
        geometry: Some(Geometry::new(Value::from(p))),
        bbox: None,
        id: None,
        properties: operation.map(
            |operation| Map::from_iter(
                std::iter::once(("operation".to_string(), json!(operation)))
            )
        ),
        foreign_members: None,
    }
}

/*
pub fn write_testcase(polygons: &[MultiPolygon<f64>], filename: &str,) {
    let features: Vec<_> = polygons.iter().map(|p| convert_to_feature(p)).collect();
    write_compact_geojson(&features, filename);
}
*/