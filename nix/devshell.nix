{
  perSystem = {
    self',
    pkgs,
    env,
    ...
  }: {
    devShells.default = pkgs.mkShell {
      inputsFrom = [self'.packages.default];

      packages = with pkgs; [
        (python3.withPackages (python-pkgs:
          with python-pkgs; [
            toml
          ]))
      ];

      inherit env;
    };
  };
}
