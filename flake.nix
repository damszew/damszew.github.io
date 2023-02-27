{
  description = "A dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Only compile time deps
        nativeBuildInputs = with pkgs; [
          zola
        ];

        # Compile & runtime deps
        buildInputs = with pkgs; [
        ];

      in
      with pkgs;
      {
        # `nix develop`
        devShells.default = mkShell {
          inherit buildInputs nativeBuildInputs;
        };
      }
    );
}
