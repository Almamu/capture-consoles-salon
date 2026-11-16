# Module NixOS : `programs.gl310-studio.enable = true;` installe GL310 Studio et la règle udev qui donne le
# boîtier à l'utilisateur connecté (uaccess, comme tools/setup-phase1.sh, sans le groupe plugdev de Debian).
# Les firmwares ne sont pas libres : autoriser « gl310-firmware » dans nixpkgs.config.allowUnfreePredicate.
{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.programs.gl310-studio;
in
{
  options.programs.gl310-studio = {
    enable = lib.mkEnableOption "GL310 Studio, capture app for the AVerMedia Live Gamer Portable Lite (GL310)";
    package = lib.mkOption {
      type = lib.types.package;
      default = pkgs.callPackage ./package.nix { gl310-firmware = pkgs.callPackage ./firmware.nix { }; };
      defaultText = lib.literalMD "GL310 Studio built from this repository, with the firmware from archive.org";
      description = "The GL310 Studio package to install.";
    };
  };

  config = lib.mkIf cfg.enable {
    environment.systemPackages = [ cfg.package ];
    # c835 : le boîtier au branchement ; d835 : identifiant probable une fois le firmware lancé.
    services.udev.packages = [
      (pkgs.writeTextDir "lib/udev/rules.d/70-avermedia-gl310.rules" ''
        SUBSYSTEM=="usb", ATTR{idVendor}=="07ca", ATTR{idProduct}=="c835", TAG+="uaccess"
        SUBSYSTEM=="usb", ATTR{idVendor}=="07ca", ATTR{idProduct}=="d835", TAG+="uaccess"
      '')
    ];
  };
}
