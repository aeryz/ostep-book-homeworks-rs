{
  description = "Aya description";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
      flake-utils.lib.eachSystem
      (with flake-utils.lib.system; [ x86_64-linux ])
      (system: 
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs { inherit system overlays; };
          llvmPackages = pkgs.llvmPackages_21;
          rust-toolchain = pkgs.rust-bin.nightly."2025-12-15".default.override {
            extensions = [
              "rust-src"
              "rust-analyzer"
            ];
          };

          rustfmt = pkgs.rust-bin.stable.latest.rustfmt;
        in
    {
    devShells.default = pkgs.mkShell {
      packages = with pkgs; [
        clang
        openssl
        pkg-config
      ] ++ [
        rust-toolchain
        rustfmt
      ];

      shellHook = ''
        export LIBCLANG_PATH=${llvmPackages.libclang.lib}/lib
      '';
    };
  });
}
