use osm_boundaries_utils::build_boundary;
use osmpbfreader::{OsmId, OsmObj, Relation, RelationId};
use std::fs::File;
use std::io::BufReader;
use std::collections::HashSet;

use geojson::{Feature, FeatureCollection, GeoJson, Geometry};

fn import_ags_ids() -> HashSet<String> {
    use serde_json::{Result, Value, Map};
    use regex::Regex;

    let json_file = File::open("../wahlergebnisse-brandenburg/out.json").unwrap();
    let json_reader = BufReader::new(json_file);
    let json_data: Value = serde_json::from_reader(json_reader).unwrap();
    let id_map: &Map<String, Value> = json_data["AfD"].as_object().unwrap();

    let mut result = HashSet::new();
    for id in id_map.keys() {
        // Here we want to make sure that we have at least an AGS with 8 numbers.
        let re = Regex::new(r"(........).*").unwrap();
        match re.captures(id) {
            Some(matches) => match matches.get(1) {
                Some(ags) => {
                    result.insert(ags.as_str().to_string());
                },
                None => {}
            },
            None => {}
        }
    }
    result
}

fn main() {
    // let osm_ids: Vec<OsmId> = {
    //     vec![62353, 1240588, 365821, 1274193]
    //         .iter()
    //         .map(|&v| RelationId(v).into())
    //         .collect()
    // };
    let mut osm_ids: Vec<OsmId> = Vec::new();
    let ags_set: HashSet<String> = import_ags_ids();
    println!("{:?}, len: {}", ags_set, ags_set.len());
    let mut found_ags: HashSet<String> = HashSet::new();

    let raw_file = File::open("raw/brandenburg-latest.osm.pbf").expect("Couldn't open pbf file.");
    let mut pbf_reader = osmpbfreader::OsmPbfReader::new(raw_file);
    println!("Decoding pbf file...");
    let objs = pbf_reader
        .get_objs_and_deps(|obj| {
                           match &obj.tags().get("de:amtlicher_gemeindeschluessel") {
                               Some(ags) => {
                                   if ags_set.contains(ags.as_str()) {
                                       osm_ids.push(obj.id());
                                       found_ags.insert(ags.to_string());
                                   }
                                   true
                               },
                               None => false,
                           }
        })
        .expect("Failed to decode pbf file.");

    for ags in ags_set {
        if !found_ags.contains(&ags) {
            println!("AGS {} was not found in the Osm data", ags);
        }
    }

    println!("{:?}, len: {}", osm_ids, osm_ids.len());
    println!("Building boundaries...");

    let mut features: Vec<Feature> = Vec::new();
    for osm_id in osm_ids {
        let relation_obj: &OsmObj = objs.get(&osm_id)
            .expect(format!("OsmId {:?} was not in objs.", &osm_id).as_str());
        let relation = match relation_obj.relation() {
            Some(r) => r,
            None => { continue }
        };
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
    let mut out_file = File::create("geojson.js").unwrap();
    let js_str = format!("var geojsonFeature = {}", geojson.to_string());
    out_file.write_all(js_str.as_bytes()).unwrap();
}
