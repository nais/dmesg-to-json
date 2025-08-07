{
  description = "A Nix-flake-based Rust development environment";

  inputs = {
    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = inputs:
    inputs.flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import inputs.nixpkgs {
        inherit system;
        overlays = [(import inputs.rust-overlay)];
      };

      rustToolchain = pkgs.rust-bin.stable.latest.default;
      # Set-up build dependencies and configure rust
      craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;

      # Shamelessly stolen from:
      # https://github.com/fedimint/fedimint/blob/66519d5e978e22bb10a045f406101891e1bb7eb5/flake.nix#L99
      filterSrcWithRegexes = regexes: src: let
        basePath = toString src + "/";
      in
        pkgs.lib.cleanSourceWith {
          filter = (
            path: type: let
              relPath = pkgs.lib.removePrefix basePath (toString path);
              includePath =
                (type == "directory")
                || pkgs.lib.any
                (re: builtins.match re relPath != null)
                regexes;
            in
              # uncomment to debug:
              # builtins.trace "${relPath}: ${lib.boolToString includePath}"
              includePath
          );
          inherit src;
        };

      cargo-details = pkgs.lib.importTOML ./Cargo.toml;
      binary-name = cargo-details.package.name;
      commonArgs = {
        nativeBuildInputs = with pkgs; [pkg-config];
      };

      # Compile and cache only cargo dependencies
      dep-files-filter = ["Cargo.lock" "Cargo.toml"];
      cargo-deps = craneLib.buildDepsOnly (commonArgs
        // {
          src = filterSrcWithRegexes dep-files-filter ./.;
          pname = "${binary-name}-deps";
        });

      # Compile and cache only workspace code (seperately from 3rc party dependencies)
      package-file-filter = dep-files-filter ++ [".*\.rs" "tests/data/.+\.txt"];
      cargo-package = craneLib.buildPackage (commonArgs
        // {
          inherit cargo-deps;
          src = filterSrcWithRegexes package-file-filter ./.;
          pname = binary-name;
        });
    in {
      checks = {
        inherit
          cargo-package
          ;
      };
      devShells.default = pkgs.mkShell {
        nativeBuildInputs =
          (with pkgs; [
            cargo-audit
            cargo-deny
            cargo-cross
            cargo-outdated

            # Editor stuffs
            lldb
            rust-analyzer
          ])
          ++ [
            # Packages made in this flake
            cargo-package
            rustToolchain # `cargo`, `rustfmt`, `rustc`, etc...
          ]
          ++ pkgs.lib.optionals (pkgs.stdenv.isLinux) (with pkgs; [cargo-watch]); # Currently broken on macOS

        shellHook = ''
          ${rustToolchain}/bin/cargo --version
        '';
      };
      packages = {
        rust = cargo-package;
        docker = pkgs.dockerTools.buildImage {
          name = binary-name;
          tag = "v${cargo-details.package.version}";
          extraCommands = ''mkdir -p data'';
          config = {
            Cmd = "--help";
            Entrypoint = ["${cargo-package}/bin/${binary-name}"];
          };
        };
      };
      packages.default = cargo-package;

      # Now `nix fmt` works!
      formatter = pkgs.alejandra;
    });
}
