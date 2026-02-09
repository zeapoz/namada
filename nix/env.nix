{
  perSystem = {pkgs, ...}: {
    _module.args.env = {
      # Namada uses RUSTUP_TOOLCHAIN during build process, fails if not set.
      RUSTUP_TOOLCHAIN = "";

      LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";

      ROCKSDB_LIB_DIR = "${pkgs.rocksdb}/lib";
      ROCKSDB_INCLUDE_DIR = "${pkgs.rocksdb}/include";
    };
  };
}
