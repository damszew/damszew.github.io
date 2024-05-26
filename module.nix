{ self, ... }@ inputs:
{ config, pkgs, lib, ... }:
let
  inherit (lib) mkEnableOption mkOption mkIf types;
  cfg = config.services.damszew-dev;
in
{
  options.services.damszew-dev = {
    enable = mkEnableOption "damszew.dev server";
  };
  config = mkIf (cfg.enable) {
    services.caddy.virtualHosts."damszew.dev" = {
      extraConfig = ''
        root * /var/www/damszew-dev
        file_server

        handle_errors {
          rewrite * /{err.status_code}.html
          file_server
        }
      '';
    };

  };
}
