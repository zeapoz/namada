{
  perSystem = {
    pkgs,
    env,
    ...
  }: {
    packages.cometbft = pkgs.buildGoModule rec {
      pname = "cometbft";
      version = "0.37.17";

      src = pkgs.fetchFromGitHub {
        owner = "cometbft";
        repo = "cometbft";
        rev = "v${version}";
        hash = "sha256-5+pY9vSh4VAZMtLBRO4SVU0RvvGSFeKpZZbH6FhOJac=";
      };

      vendorHash = "sha256-F6km3YpvfdpPeIJB1FwA5lQvPda11odny0EHPD8B6kw=";

      # TODO: Revisit later, looks like some non-deterministic net issues.
      doCheck = false;
    };
  };
}
