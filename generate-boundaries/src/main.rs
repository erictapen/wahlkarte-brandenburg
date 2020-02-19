use osm_boundaries_utils::build_boundary;
use osmpbfreader::{Relation, RelationId};
use std::fs::File;

use geojson::{Feature, GeoJson, Geometry};

fn main() {
    let raw_file = File::open("raw/brandenburg-latest.osm.pbf").expect("Couldn't open pbf file.");
    let mut pbf_reader = osmpbfreader::OsmPbfReader::new(raw_file);
    println!("Decoding pbf file...");
    let objs = pbf_reader
        .get_objs_and_deps(|obj| {
            obj.id() == RelationId(62353).into()
            // || obj.id() == RelationId(1240588).into()
            // || obj.id() == RelationId(365821).into()
        })
        .expect("Failed to decode pbf file.");

    println!("Build boundaries...");

    let mittelmark: &Relation = objs
        .get(&RelationId(62353).into())
        .unwrap()
        .relation()
        .unwrap();
    let multi_polygon = build_boundary(&mittelmark, &objs).unwrap();
    // println!("{:?}", multi_polygon);

    let geometry = Geometry::new(geojson::Value::from(&multi_polygon));
    let geojson = GeoJson::Feature(Feature {
        bbox: None,
        geometry: Some(geometry),
        id: None,
        properties: None,
        foreign_members: None,
    });

    use std::io::Write;
    let mut out_file = File::create("out.geo.json").unwrap();
    out_file.write_all(geojson.to_string().as_bytes()).unwrap();
}
