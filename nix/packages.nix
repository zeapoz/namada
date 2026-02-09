{
  perSystem = {
    pkgs,
    env,
    ...
  }: let
    pname = "namada";

    cargoToml = builtins.fromTOML (builtins.readFile ../Cargo.toml);
    lockFile = ../Cargo.lock;

    rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ../rust-toolchain.toml;
  in {
    packages.default = pkgs.rustPlatform.buildRustPackage {
      inherit pname env;
      inherit (cargoToml.package) version;

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
    };
  };
}
