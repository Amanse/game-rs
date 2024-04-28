{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    # naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";

    umu = {
      url = "git+https://github.com/Open-Wine-Components/umu-launcher/?dir=packaging\/nix&submodules=1";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    flake-utils,
    nixpkgs,
    crane,
    umu,
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
          cargoExtraArgs = "--features nixos";

          propagatedBuildInputs = [umu.packages.${pkgs.system}.umu];

          # Add extra inputs here or any other derivation settings
          # doCheck = true;
          # buildInputs = [];
          # nativeBuildInputs = [];
        };

        # For `nix develop`:
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [cargo rustc rustfmt rust-analyzer clippy umu.packages.${pkgs.system}.umu python3];
          shellHook = ''
            export PATH="$PATH:/home/me/.cargo/bin"
          '';
        };
      }
    );
}
