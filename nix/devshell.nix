{
  perSystem = {pkgs, ...}: {
    devShells.default = with pkgs;
      pkgs.mkShell {
        packages = [
          rustup # Namada uses RUSTUP_TOOLCHAIN during build process.
          gnumake

          # Build dependencies.
          openssl
          pkg-config
          protobuf
          rocksdb
          systemd # libudev.
        ];

        LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";

        ROCKSDB_LIB_DIR = "${pkgs.rocksdb}/lib";
        ROCKSDB_INCLUDE_DIR = "${pkgs.rocksdb}/include";
      };
  };
}
