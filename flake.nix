{
  description = "Wahlkarte Brandenburg";

  inputs = {
    nixpkgs.url = "github:NixOS/Nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    brandenburg-pbf = {
      type = "git";
      url = "file:///home/kerstin/git/mmz/wahlkarte/generate-boundaries/raw";
      flake = false;
    };
  };

  outputs =
    { self, nixpkgs, flake-utils, brandenburg-pbf }:
    flake-utils.lib.eachSystem
      [ "x86_64-linux" "aarch64-linux" ]
      (system:
      let pkgs = nixpkgs.legacyPackages.${system}; in
      {

        packages =
          let
            inherit (self.packages.${system})
              boundaries wahlergebnisse-brandenburg generate-boundaries wahlkarte;
          in
          {

            default = self.packages.${system}.wahlkarte;

            generate-boundaries = (
              import ./generate-boundaries/Cargo.nix {
                inherit pkgs;
              }
            ).rootCrate.build;

            wahlergebnisse-brandenburg = pkgs.runCommand "wahlergebnisse-brandenburg"
              {
                src = ./wahlergebnisse-brandenburg;
                buildInputs = with pkgs.python3.pkgs; [
                  openpyxl
                ];
              } ''
              mkdir -p $out
              python3 $src/generate.py $src/raw/ $out/
            '';
            boundaries = pkgs.runCommand "boundaries.json"
              {
                buildInputs = [ generate-boundaries ];
              } ''
              generate-boundaries \
                --ags-file ${wahlergebnisse-brandenburg}/eu2019.json \
                --pbf-file ${brandenburg-pbf}/brandenburg-latest.osm.pbf $out
            '';

            wahlkarte =
              let
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
              pkgs.runCommand "wahlkarte"
                {
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
                ln -s ${boundaries} $out/boundaries.json
                ln -s ${wahlergebnisse-brandenburg} $out/elections
              '';

            deploy = pkgs.writeScript "deploy-wahlkarte" ''
              #! /usr/bin/env bash
              ${pkgs.openssh}/bin/scp -r ${wahlkarte}/* mmz.erictapen.name:/webroot/mmz.erictapen.name/
            '';
          };

        devShells.default =
          let
            pkgs = import nixpkgs {
              system = "x86_64-linux";
            };
          in
          pkgs.mkShell {
            buildInputs = with pkgs; [
              darkhttpd
              cargo
              rustc
              rustfmt
              clippy
              crate2nix
              python3Packages.openpyxl
            ];
          };
      });

}
