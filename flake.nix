{
  description = "GL310 Studio: capture app for the AVerMedia Live Gamer Portable Lite (GL310)";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { self, nixpkgs }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      # `nix build` / `nix run` sur ce dépôt acceptent les firmwares d'AVerMedia, et seulement eux.
      forAllSystems =
        f:
        nixpkgs.lib.genAttrs systems (
          system:
          f (
            import nixpkgs {
              inherit system;
              config.allowUnfreePredicate = pkg: nixpkgs.lib.getName pkg == "gl310-firmware";
            }
          )
        );
    in
    {
      packages = forAllSystems (pkgs: rec {
        gl310-firmware = pkgs.callPackage ./nix/firmware.nix { };
        gl310-studio = pkgs.callPackage ./nix/package.nix { inherit gl310-firmware; };
        default = gl310-studio;
      });

      nixosModules.default = ./nix/module.nix;

      devShells = forAllSystems (pkgs: {
        default = import ./nix/shell.nix { inherit pkgs; };
      });

      formatter = forAllSystems (pkgs: pkgs.nixfmt-tree);
    };
}
