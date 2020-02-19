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
      in pkgs.runCommand "wahlkarte" {
        src = ./.;
      } ''
        mkdir -p $out
        cp index.html $out/
      '';

    };

}
