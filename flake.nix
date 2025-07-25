{
  description = "BLAZINGLY FAST AND MEMORY SAFE library for the legendary is_even and is_odd functions! 🔥";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };

    cargo2nix = {
      url = "github:cargo2nix/cargo2nix/release-0.12";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
        rust-overlay.follows = "rust-overlay";
      };
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      cargo2nix,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ cargo2nix.overlays.default ];
        };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };
        rustComponents = rustToolchain.passthru.availableComponents;
        rustPkgs = pkgs.rustBuilder.makePackageSet {
          packageFun = import ./Cargo.nix;
          inherit rustToolchain;
        };
        buildDeps = with pkgs; [
          cmake
          makeWrapper
          pkg-config
        ];
        runtimeDeps = with pkgs; [
          openssl
        ];
        workspaceShell = rustPkgs.workspaceShell {
          packages = (with pkgs; [
            rustToolchain

            pre-commit
          ])
          ++ [ cargo2nix.packages.${system}.cargo2nix ]
          ++ runtimeDeps ++ buildDeps;
        };

        mkBashApp = script: flake-utils.lib.mkApp {
          drv = pkgs.writers.writeBashBin "cargo2nix-gen" script;
        };

        pre-commit-exe = "${pkgs.pre-commit}/bin/pre-commit";
      in rec {
        packages = {
          default = (rustPkgs.workspace.pido-rs {}).bin;
        };

        apps = {
          default = { type = "app"; program = "${packages.default}/bin/pido-rs"; };

          cargo2nix-gen = mkBashApp ''
            ${cargo2nix.packages.${system}.cargo2nix}/bin/cargo2nix -o -l
          '';

          pre-commit-install = mkBashApp ''
            ${pre-commit-exe} install
          '';

          pre-commit-update = mkBashApp ''
            ${pre-commit-exe} autoupdate
          '';

          pre-commit-run = mkBashApp ''
            ${pre-commit-exe} run --all-files
          '';
        };

        devShells.default = workspaceShell;
      }
    );
}
