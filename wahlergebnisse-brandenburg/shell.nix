with import <nixpkgs> {};
pkgs.mkShell {
  buildInputs = [
    python3Packages.openpyxl
  ];
}
