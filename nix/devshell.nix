{
  perSystem = {
    self',
    pkgs,
    env,
    ...
  }: {
    devShells.default = pkgs.mkShell {
      inputsFrom = [self'.packages.default];
      inherit env;
    };
  };
}
