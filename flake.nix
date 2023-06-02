{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    # naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/release-22.05";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
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
        craneLib = crane.lib.${system};
      in {
        # For `nix build` & `nix run`:
        packages.default = craneLib.buildPackage {
          src = craneLib.cleanCargoSource (craneLib.path ./.);

          # Add extra inputs here or any other derivation settings
          # doCheck = true;
          # buildInputs = [];
          # nativeBuildInputs = [];
        };

        # For `nix develop`:
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [rustup rust-analyzer];
          shellHook = ''
            export PATH="$PATH:/home/me/.cargo/bin"
          '';
        };
      }
    );
}
