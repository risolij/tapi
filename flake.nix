{
  description = "Rust Environment";
  nixConfig.bash-prompt = "\[tapi\]$ ";

  inputs = {
    nixpkgs.url      = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        system = "x86_64-linux";
        rustVersion = pkgs.rust-bin.selectLatestNightlyWith;

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustVersion;
          rustc = rustVersion;
        };

        myRustBuild = rustPlatform.buildRustPackage {
          pname = "tapi";
          version = "0.1.0";
          src = ./tapi;
          cargoLock.lockFile = ./Cargo.lock;
        };

        ## postgresFromDockerHub = pkgs.dockerTools.pullImage {
        ##   imageName = "postgres";
        ##   imageDigest = "sha256:3e2eba0a6efbeb396e086c332c5a85be06997d2cf573d34794764625f405df4e";
        ##   sha256 = "3e2eba0a6efbeb396e086c332c5a85be06997d2cf573d34794764625f405df4e";
        ##   finalImageTag = "latest";
        ##   finalImageName = "postgres-latest";
        ## }

        dockerImage = pkgs.dockerTools.buildImage {
          name = "tapi";
          config = { 
            Cmd = [ 
              "${myRustBuild}/bin/tapi" 
            ]; 
            WorkingDir = "/";
          };
        };

        lib = nixpkgs.lib;
      in
      {
        packages = {
          rustPackage = myRustBuild;
          docker = dockerImage;
        };

        defaultPackage = dockerImage;

        devShell = pkgs.mkShell {
          buildInputs = [ 
            (rustVersion ( toolchain: toolchain.default.override { extensions = [ "rust-src" ]; }))
            pkgs.rust-analyzer 
            pkgs.sqlx-cli
            pkgs.postgresql
          ];
        };
      }
    );
}
