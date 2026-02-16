{
  perSystem = {
    self',
    pkgs,
    env,
    ...
  }: let
    pname = "namada";

    cargoToml = builtins.fromTOML (builtins.readFile ../../Cargo.toml);
    lockFile = ../../Cargo.lock;
  in {
    packages.default = pkgs.rustPlatform.buildRustPackage {
      inherit pname env;
      inherit (cargoToml.workspace.package) version;

      src = ../.;

      cargoLock = {
        inherit lockFile;
      };

      nativeBuildInputs = with pkgs; [
        binaryen # wasm-opt.
        blst
        clang.cc # Has to be unwrapped as we're targeting WASM.
        rustup
        gnumake
        pkg-config
        protobuf
        python3

        self'.packages.cometbft
      ];

      buildInputs = with pkgs; [
        openssl
        rocksdb
        systemd # libudev.
      ];

      # TODO: Remove this once we're using a proper build system.
      # Disable the check phase as Namada depends on some compiled WASM modules being present.
      doCheck = false;
    };
  };
}
