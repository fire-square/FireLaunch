{
  description = "FireLaunch Minecraft Launcher";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    import-cargo.url = github:edolstra/import-cargo;
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, import-cargo, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustVersion = pkgs.rust-bin.stable.latest.default;
        inherit (import-cargo.builders) importCargo;

        nativeBuildInputs = with pkgs; [
          pkg-config
        ] ++ [
          rustVersion
        ];

        buildInputs = with pkgs; [
          graphene
          gtk4
          libadwaita
          openssl
        ];

        firelaunch = pkgs.stdenv.mkDerivation {
          name = "firelaunch";
          src = self;

          inherit buildInputs;

          nativeBuildInputs = [
            (importCargo { lockFile = ./Cargo.lock; inherit pkgs; }).cargoHome
          ] ++ nativeBuildInputs;

          buildPhase = ''
            cargo build --release --offline
          '';

          installPhase = ''
            install -Dm775 ./target/release/firesquare-launcher $out/bin/firelaunch
          '';
        };
      in
      {
        packages = {
          default = firelaunch;
          firelaunch = firelaunch;
        };

        devShells.default = pkgs.mkShell {
          buildInputs = nativeBuildInputs ++ buildInputs;
        };
      }
    );
}