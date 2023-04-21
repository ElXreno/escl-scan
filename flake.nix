{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    utils.url = "github:numtide/flake-utils";
    gitignore = {
      url = "github:hercules-ci/gitignore.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, utils, naersk, rust-overlay, gitignore }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rust-toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        naersk-lib = pkgs.callPackage naersk {
          cargo = rust-toolchain;
          rustc = rust-toolchain;
        };

        escl-scan-cli = naersk-lib.buildPackage {
          inherit nativeBuildInputs buildInputs;
          pname = "escl-scan-cli";
          src = gitignore.lib.gitignoreSource ./.;
        };

        nativeBuildInputs = with pkgs; [ pkg-config ];
        buildInputs = with pkgs; [ openssl ];
      in
      {
        packages.escl-scan-cli = escl-scan-cli;
        packages.default = self.packages.${system}.escl-scan-cli;

        devShells.default = with pkgs; mkShell {
          inherit nativeBuildInputs;
          buildInputs = buildInputs ++ [ (rust-toolchain.override { extensions = [ "rust-src" ]; }) ];
        };

        checks = {
          escl-scan-cli = self.packages.${system}.default.overrideAttrs (super: { doCheck = true; });
        };
      });

  nixConfig = {
    extra-experimental-features = "nix-command flakes";
    extra-substituters = [ "https://elxreno-rust.cachix.org" ];
    extra-trusted-public-keys = [ "elxreno-rust.cachix.org-1:cfUElkBCai6A6hqku/tOCrYt9qF+vQtAV8+8MF16gf8=" ];
  };
}
