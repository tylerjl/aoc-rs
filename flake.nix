{
  description = "Advent of Code solutions in Rust";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    nci.url = "github:yusdacra/nix-cargo-integration?rev=333fa0edf43e8047832eb23f86bb12954e1f0d1e";
    nci.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, nci, ... }:
    nci.lib.makeOutputs {
      root = ./.;
      overrides = {
        shell = common: prev: {
          packages = prev.packages ++ (with common.pkgs; [
            rust-analyzer
            cargo-watch
          ]);
        };
      };
    };
}
