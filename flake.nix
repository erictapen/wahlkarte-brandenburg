{
  description = "Wahlkarte Brandenburg";

  inputs = {
    nixpkgs.url = "github:NixOS/Nixpkgs/nixos-unstable";
    brandenburg-pbf = {
      type = "git";
      url = "file:///home/justin/git/mmz/wahlkarte/generate-boundaries/raw";
      flake = false;
    };
  };

  outputs =
    { self, nixpkgs, brandenburg-pbf }: rec {

      defaultPackage.x86_64-linux = packages.x86_64-linux.wahlkarte;

      packages.x86_64-linux = let
        pkgs = import nixpkgs {
          system = "x86_64-linux";
        };
      in
        rec {

          generate-boundaries = (
            import ./generate-boundaries/Cargo.nix {
              inherit pkgs;
            }
          ).rootCrate.build;

          wahlergebnisse-brandenburg = pkgs.runCommand "wahlergebnisse-brandenburg.json" {
            src = ./wahlergebnisse-brandenburg;
            buildInputs = with pkgs.python3.pkgs; [
              openpyxl
            ];
          } ''
            python3 $src/generate.py $src/raw/DL_BB_EU2019.xlsx $out
          '';
          boundaries = pkgs.runCommand "boundaries.json" {
            buildInputs = [ generate-boundaries ];
          } ''
            generate-boundaries \
              --ags-file ${wahlergebnisse-brandenburg} \
              --pbf-file ${brandenburg-pbf}/brandenburg-latest.osm.pbf $out
          '';

          wahlkarte = let
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
          in
            pkgs.runCommand "wahlkarte" {
              src = builtins.filterSource
                (
                  path: type:
                    baseNameOf path != "generate-boundaries" && baseNameOf path != "wahlergebnisse-brandenburg"
                )
                ./.;
            } ''
              mkdir -p $out
              cp $src/index.html $out/
              cp $src/wahlkarte.js $out/
              mkdir -p $out/leaflet@${leaflet.version}/dist
              ln -s ${leaflet.js}  $out/leaflet@${leaflet.version}/dist/leaflet.js
              ln -s ${leaflet.css} $out/leaflet@${leaflet.version}/dist/leaflet.css
              cp ${boundaries} $out/boundaries.json
              mkdir -p $out/elections
              cp ${wahlergebnisse-brandenburg} $out/elections/eu2019.json
            '';

          deploy = pkgs.writeScript "deploy-wahlkarte" ''
            #! /usr/bin/env bash
            ${pkgs.openssh}/bin/scp -r ${wahlkarte}/* mmz.erictapen.name:/webroot/mmz.erictapen.name/
          '';
        };

      devShell.x86_64-linux = let
        pkgs = import nixpkgs {
          system = "x86_64-linux";
        };
      in
        pkgs.mkShell {
          buildInputs = with pkgs; [
            darkhttpd
          ];
        };
    };

}
