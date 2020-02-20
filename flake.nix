{
  edition = 201909;

  description = "Olga Website";

  inputs = {
    nixpkgs.flake = false;
  };

  outputs =
    { self, nixpkgs }: rec {

      defaultPackage.x86_64-linux = packages.x86_64-linux.wahlkarte;

      packages.x86_64-linux.wahlkarte = let 
        pkgs = import nixpkgs {
          system = "x86_64-linux";
        };
        leaflet = rec {
          version = "1.6.0";
          js = pkgs.fetchurl {
            url = "https://unpkg.com/leaflet@${version}/dist/leaflet.js";
            sha256 = "sha256:1jnadhs874xis8gg91433m29yb1q15ijclmk7nc6pn0g16pi3nkw";
          };
          css = pkgs.fetchurl {
            url = "https://unpkg.com/leaflet@${version}/dist/leaflet.css";
            sha256 = "sha256:18vn46pcig8aw3yggpfzbcmjbl7z263a7811lf98wkwji44hcws8";
          };
        };
      in pkgs.runCommand "wahlkarte" {
        src = ./.;
      } ''
        mkdir -p $out
        cp $src/index.html $out/
        mkdir -p $out/leaflet@${leaflet.version}/dist
        ln -s ${leaflet.js}  $out/leaflet@${leaflet.version}/dist/leaflet.js
        ln -s ${leaflet.css} $out/leaflet@${leaflet.version}/dist/leaflet.css
        cp $src/generate-boundaries/geojson.js $out/
      '';

    };

}
