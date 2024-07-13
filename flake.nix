{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    fenix.url = "github:nix-community/fenix/monthly";
    crane.url = "github:ipetkov/crane";
  };

  outputs =
    inputs@{ self
    , flake-parts
    , fenix
    , crane
    , nixpkgs
    , ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      perSystem =
        { pkgs, system, ... }:
        let
          _module.args.pkgs = import nixpkgs {
            inherit system;
            overlays = [ fenix.overlays.default ];
          };

          nativeBuildInputs = with pkgs; [
            pkg-config
            openssl
          ];

          buildInputs = with pkgs; [ ];

          craneLib = (crane.mkLib pkgs).overrideToolchain
            fenix.packages.${system}.minimal.toolchain;
          src = craneLib.cleanCargoSource (craneLib.path ./.);
          cargoArtifacts = craneLib.buildDepsOnly {
            inherit buildInputs nativeBuildInputs src;
          };

          bo = craneLib.buildPackage {
            inherit cargoArtifacts buildInputs nativeBuildInputs src;
          };

          devPackages = [
            fenix.packages.${system}.complete.toolchain
          ];
        in
        {
          packages = {
            inherit bo;
            default = bo;
          };

          devShells = {
            default = pkgs.mkShell {
              inherit buildInputs nativeBuildInputs;
              packages = devPackages;
            };
          };
        };
    };
}
