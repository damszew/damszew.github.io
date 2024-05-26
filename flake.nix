{
  description = "A dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    deploy-rs = {
      url = "github:serokell/deploy-rs";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, deploy-rs, ... }@inputs:
    let
      inherit (nixpkgs.lib) recursiveUpdate;
    in
    recursiveUpdate
      (
        flake-utils.lib.eachDefaultSystem (system:
          let
            overlays = [ ];
            pkgs = import nixpkgs {
              inherit system overlays;
            };

            # Only compile time deps
            nativeBuildInputs = with pkgs; [
              zola
              deploy-rs.packages.${system}.deploy-rs
            ];

            # Compile & runtime deps
            buildInputs = with pkgs; [
            ];

            damszew-dev = pkgs.stdenv.mkDerivation
              {
                name = "Zola build";

                src = self;

                inherit nativeBuildInputs;

                installPhase = ''
                  zola build --output-dir $out
                '';

              };

            activate = pkgs.writeShellScriptBin "activate" ''
              set -euo pipefail
              rm -rf /var/www/damszew-dev
              mkdir -p /var/www/damszew-dev
              cp -R ${damszew-dev}/* /var/www/damszew-dev/
              systemctl reload caddy.service
            '';
          in
          {
            # `nix flake check`
            # checks = {
            #   inherit damszew-dev;
            # };

            # `nix build .`
            packages = {
              inherit damszew-dev activate;

              default = damszew-dev;
            };

            # `nix run .`
            # apps.default = flake-utils.lib.mkApp {
            #   drv = blog_server;
            # };

            # `nix develop`
            devShells.default = pkgs.mkShell {
              inherit buildInputs nativeBuildInputs;
            };
          }
        )
      )
      {
        nixosModules = { default = import ./module.nix inputs; };


        # nix run github:serokell/deploy-rs .
        deploy = {
          magicRollback = true;
          autoRollback = true;
          nodes.perun =
            let
              system = "x86_64-linux";
              activate = self.packages.${system}.activate;
            in
            {
              hostname = "perun";
              profiles.damszew-dev = {
                sshUser = "deploy";
                user = "deploy";
                path = deploy-rs.lib.${system}.activate.custom
                  self.packages.${system}.damszew-dev
                  "sudo ${activate}/bin/activate";
              };
            };
        };

        # `nix flake check`
        checks = builtins.mapAttrs (_: lib: lib.deployChecks self.deploy) deploy-rs.lib;
      };
}
