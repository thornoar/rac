{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs: with inputs;
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        pname = "amyc";
        version = "0.1.0";
      in {
        packages.${system} = rec {
          amyc = pkgs.callPackage (stdenv: stdenv.mkDerivation rec {
            inherit pname;
            inherit version;
            src = ./.;
            buildPhase = ''
              cargo build
            '';
            installPhase = ''
              mkdir -p $out/bin
              mv amyc $out/bin/
            '';
            nativeBuildDepends = with pkgs; [
              cargo
            ];
          });
          default = amyc;
        };
        apps.${system}.default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/amyc";
        };
      }
    );
}
