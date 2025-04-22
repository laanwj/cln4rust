{
  description = "A simple core lightning plugin that simplifies the Splice operation";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
       pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlay ];
        };

       clightning = pkgs.clightning.overrideAttrs (oldAttrs: {
          version = "master-abfe";
          src = pkgs.fetchgit {
            url = "https://github.com/ElementsProject/lightning";
            rev = "abfe55e2147ad6ff8d0a155cd49c90a6a659a164";
            sha256 = "sha256-UDkrlss4ufd70zYWf6IESiJQ/yo9J7BSdVH5UKrIBbQ=";
            fetchSubmodules = true;
          };
          configureFlags = [ "--disable-rust" "--disable-valgrind" ];
       });
      in {
        packages = {
          default = pkgs.gnumake;
        };
        formatter = pkgs.nixpkgs-fmt;

        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            # build dependencies
            libcap
            gcc
            pkg-config
            git

            gnumake

            rustc
            cargo

            # integration test dependencies
            clightning
            bitcoind
          ];

          shellHook = ''
            export HOST_CC=gcc
            export PWD="$(pwd)"
            export BITCOIND_EXE="${pkgs.bitcoind}/bin/bitcoind"
          '';
        };
      }
    );
}
