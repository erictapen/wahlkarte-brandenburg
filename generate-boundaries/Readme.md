# Generate boundaries

Dieses Rust-Projekt arbeitet auf einem Openstreetmap-Dump und einer Liste von Amtlichen Gemeindeschlüsseln (AGS) und generiert daraus eine GeoJSON-Datei mit den boundaries dieser Gemeinden.

```
generate-boundaries --ags-file ../wahlergebnisse-brandenburg/out.json --pbf-file raw/brandenburg-latest.osm.pbf geojson.js
```
