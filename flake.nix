{
  description = "A lighter way to list files and directories";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    let
      cerium =
        { lib, rustPlatform, pkg-config, file }:
        rustPlatform.buildRustPackage {
          pname = "cerium";
          version = (lib.importTOML ./Cargo.toml).package.version;

          src = lib.cleanSource ./.;
          cargoLock.lockFile = ./Cargo.lock;

          # The magic feature links libmagic, which `file` provides.
          nativeBuildInputs = [ pkg-config ];
          buildInputs = [ file ];

          buildFeatures = [ "magic" "checksum" ];

          # wcwidth reads the locale, so the width tests need a UTF-8 one.
          preCheck = ''
            export LC_ALL=C.UTF-8
          '';

          meta = {
            description = "A lighter way to list files and directories";
            homepage = "https://codeberg.org/rly0nheart/cerium";
            license = lib.licenses.mit;
            mainProgram = "ce";
            platforms = lib.platforms.unix;
          };
        };

      overlay = final: prev: {
        cerium = final.callPackage cerium { };
      };
    in
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ overlay ];
        };
        rustToolchain = pkgs.symlinkJoin {
          name = "cerium-rust-toolchain";
          paths = with pkgs; [
            rustc
            cargo
            rustfmt
            clippy
            rust-analyzer
          ];
        };
      in
      {
        packages.default = pkgs.cerium;
        packages.cerium = pkgs.cerium;

        apps.default = flake-utils.lib.mkApp {
          drv = self.packages.${system}.default;
          name = "ce";
        };

        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rustToolchain
            pkg-config
            file
          ];
        };
      }) // {
      overlays.default = overlay;
    };
}
