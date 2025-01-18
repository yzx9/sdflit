{
  description = "sdflit";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      fenix,
      ...
    }:

    let
      inherit (nixpkgs) lib;

      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forEachSupportedSystem = lib.genAttrs supportedSystems;
    in
    {
      # nix develop
      devShells = forEachSupportedSystem (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [
              fenix.overlays.default
            ];
          };

          fenix' = fenix.packages.${system};
          toolchain = fenix'.complete;
          rust = fenix'.combine [
            toolchain.cargo
            toolchain.rustc
            toolchain.rust-src
            toolchain.clippy
            toolchain.rustfmt
          ];

        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              # rust
              rust
              rust-analyzer
              maturin

              # python
              python313
              python3Packages.pytest

              just
            ];

            shellHook = ''
              # Specify the rust-src path (many editors rely on this)
              export RUST_SRC_PATH="${toolchain.rust-src}/lib/rustlib/src/rust/library";
            '';
          };
        }
      );
    };
}
