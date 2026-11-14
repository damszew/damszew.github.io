{ pkgs, ... }:

{
  languages.javascript = {
    enable = true;
    package = pkgs.nodejs_24;
    pnpm = {
      enable = true;
      install.enable = true;
    };
  };

  scripts = {
    dev.exec = "pnpm run dev";
    build.exec = "pnpm run build";
  };
}
