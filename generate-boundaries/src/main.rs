#[macro_use]
extern crate serde;

use docopt::Docopt;
use osm_boundaries_utils::build_boundary;
use osmpbfreader::{OsmId, OsmObj};
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use geojson::{Feature, FeatureCollection, GeoJson, Geometry};

#[derive(Deserialize)]
struct CliArgs {
    flag_ags_file: String,
    flag_pbf_file: String,
    arg_geojson_output: String,
}

const USAGE: &'static str = "
Usage: generate-boundaries --ags-file AGSFILE --pbf-file PBFFILE <geojson-output>
       generate-boundaries --help

Options:
    -h --help                     Show this message.
    --ags-file AGSFILE           JSON file with AGS as keys at the 2nd level dict.
    --pbf-file PBFFILE           Openstreetmap PBF file containing the geometries of said AGS.
";

fn import_ags_ids(ags_file: PathBuf) -> HashSet<String> {
    use regex::Regex;
    use serde_json::{Map, Value};

    let json_file = File::open(ags_file).unwrap();
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
                }
                None => {}
            },
            None => {}
        }
    }
    result
}

fn main() {
    let args: CliArgs = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let mut osm_ids: Vec<OsmId> = Vec::new();
    let ags_set: HashSet<String> = import_ags_ids(PathBuf::from(args.flag_ags_file));
    println!("{:?}, len: {}", ags_set, ags_set.len());
    let mut found_ags: HashSet<String> = HashSet::new();

    let raw_file = File::open(args.flag_pbf_file).expect("Couldn't open pbf file.");
    let mut pbf_reader = osmpbfreader::OsmPbfReader::new(raw_file);
    println!("Decoding pbf file...");
    let objs = pbf_reader
        .get_objs_and_deps(
            |obj| match &obj.tags().get("de:amtlicher_gemeindeschluessel") {
                Some(ags) => {
                    if ags_set.contains(ags.as_str()) {
                        osm_ids.push(obj.id());
                        found_ags.insert(ags.to_string());
                    }
                    true
                }
                None => false,
            },
        )
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
        let relation_obj: &OsmObj = objs
            .get(&osm_id)
            .expect(format!("OsmId {:?} was not in objs.", &osm_id).as_str());
        let relation = match relation_obj.relation() {
            Some(r) => r,
            None => continue,
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

    let geojson = GeoJson::FeatureCollection(FeatureCollection {
        bbox: None,
        features: features,
        foreign_members: None,
    });

    use std::io::Write;
    let mut out_file = File::create(args.arg_geojson_output).unwrap();
    let js_str = format!("var geojsonFeature = {}", geojson.to_string());
    out_file.write_all(js_str.as_bytes()).unwrap();
}
