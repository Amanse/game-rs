{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    # naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
  };

  outputs = {
    self,
    flake-utils,
    nixpkgs,
    crane,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = (import nixpkgs) {
          inherit system;
        };
        #
        # naersk' = pkgs.callPackage naersk {};
        craneLib = crane.mkLib pkgs;
      in {
        # For `nix build` & `nix run`:
        packages.default = craneLib.buildPackage {
          src = craneLib.cleanCargoSource (craneLib.path ./.);
          propagatedBuildInputs = [pkgs.umu-launcher];

          # Add extra inputs here or any other derivation settings
          # doCheck = true;
          # buildInputs = [];
          # nativeBuildInputs = [];
        };

        # For `nix develop`:
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [cargo rustc rustfmt rust-analyzer clippy umu-launcher];
          shellHook = ''
            export PATH="$PATH:/home/me/.cargo/bin"
          '';
        };
      }
    );
}
