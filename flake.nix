{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.05";
    utils.url = "github:gytis-ivaskevicius/flake-utils-plus";
  };

  outputs = inputs@{ self, utils, ... }:
    utils.lib.mkFlake rec {
      inherit self inputs;

      supportedSystems = [
        "aarch64-linux"
        "aarch64-darwin"
        "i686-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];

      outputsBuilder = channels: with channels; {
        packages = with nixpkgs; { 
          inherit (nixpkgs) package-from-overlays;

          photosphere = rustPlatform.buildRustPackage rec {
            pname = "photosphere";
            version = "v0.2.0";
            doCheck = true;
            src = ./.;
            checkInputs = [ rustfmt cargo-nextest clippy ];
            checkPhase = ''
              runHook preCheck

              cargo check
              rustfmt --check src/main.rs
              cargo clippy
              cargo nextest run

              runHook postCheck
            '';
            cargoLock = {
              lockFile = ./Cargo.lock;
            };
            meta = with nixpkgs.lib; {
              description = "An easy way to start a Photosphere project";
              homepage = "https://github.com/solfacil/photosphere-cli";
              license = licenses.bsd3;
              maintainers = [ maintainers.zoedsoupe ];
            };
          };
        };

        devShell = nixpkgs.mkShell {
          name = "photosphere";

          buildInputs = with nixpkgs; [
            rustc cargo rustfmt clippy cargo-nextest
          ];
        };
      };
    };
}
