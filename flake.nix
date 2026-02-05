{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {
    flake-parts,
    nixpkgs,
    rust-overlay,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [./nix/devshell.nix ./nix/packages.nix];

      systems = ["x86_64-linux"];

      perSystem = {system, ...}: {
        _module.args.pkgs = import nixpkgs {
          inherit system;
          overlays = [rust-overlay.overlays.default];
        };
      };
    };
}
