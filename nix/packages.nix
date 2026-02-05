{
  perSystem = {pkgs, ...}: let
    pname = "namada";
    lockFile = ../Cargo.lock;

    inherit ((builtins.fromTOML (builtins.readFile ../Cargo.toml)).workspace.package) version;

    rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ../rust-toolchain.toml;
  in {
    packages.default = pkgs.rustPlatform.buildRustPackage {
      inherit pname version;
      src = ../.;

      cargoLock = {
        inherit lockFile;
      };

      nativeBuildInputs = with pkgs; [
        rustToolchain
        gnumake
        pkg-config
        protobuf
      ];

      buildInputs = with pkgs; [
        openssl
        rocksdb
        systemd # libudev.
      ];

      # Disable the check phase as Namada depends on some compiled WASM modules being present.
      doCheck = false;

      # Namada uses RUSTUP_TOOLCHAIN during build process, fails if not set.
      RUSTUP_TOOLCHAIN = "";

      LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";

      ROCKSDB_LIB_DIR = "${pkgs.rocksdb}/lib";
      ROCKSDB_INCLUDE_DIR = "${pkgs.rocksdb}/include";
    };
  };
}
