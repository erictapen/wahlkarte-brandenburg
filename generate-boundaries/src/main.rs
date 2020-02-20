use osm_boundaries_utils::build_boundary;
use osmpbfreader::{OsmId, Relation, RelationId};
use std::fs::File;

use geojson::{Feature, FeatureCollection, GeoJson, Geometry};

fn main() {
    let osm_ids: Vec<OsmId> = {
        vec![62353, 1240588, 365821]
            .iter()
            .map(|&v| RelationId(v).into())
            .collect()
    };

    let raw_file = File::open("raw/brandenburg-latest.osm.pbf").expect("Couldn't open pbf file.");
    let mut pbf_reader = osmpbfreader::OsmPbfReader::new(raw_file);
    println!("Decoding pbf file...");
    let objs = pbf_reader
        .get_objs_and_deps(|obj| osm_ids.contains(&obj.id()))
        .expect("Failed to decode pbf file.");

    println!("Building boundaries...");

    let mut features: Vec<Feature> = Vec::new();
    for osm_id in osm_ids {
        let relation: &Relation = objs.get(&osm_id).unwrap().relation().unwrap();
        let multi_polygon = build_boundary(&relation, &objs).unwrap();

        let geometry = Geometry::new(geojson::Value::from(&multi_polygon));
        let feature = Feature {
            bbox: None,
            geometry: Some(geometry),
            id: None,
            properties: None,
            foreign_members: None,
        };
        features.push(feature);
    }

    let geojson = GeoJson::FeatureCollection( FeatureCollection {
        bbox: None,
        features: features,
        foreign_members: None,
    });

    use std::io::Write;
    let mut out_file = File::create("out.geo.json").unwrap();
    out_file.write_all(geojson.to_string().as_bytes()).unwrap();
}
