{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };
  outputs = inputs@{ ... }: inputs.flake-parts.lib.mkFlake { inherit inputs; } {
    imports = [  ];
    systems = [ "x86_64-linux" ];
    perSystem = { config, pkgs, ... }: {
      packages = rec {
        default = rsdd-debug;
        rsdd-release = pkgs.callPackage ./. {};
        rsdd-debug = (pkgs.callPackage ./. {}).overrideAttrs (o: { buildType = "debug"; });
        render-graphviz = let
          py = pkgs.python3.withPackages (p: [p.graphviz]);
        in pkgs.writeScriptBin "render-graphviz" ''
          ${py}/bin/python ${../scripts/render_graphviz.py}
        '';
      };
      apps = {
        render-graphviz = {
          type = "app";
          program = "${config.packages.render-graphviz}/scripts/render-graphviz";
        };
        bottomup_cnf_to_bdd = {
          type = "app";
          program = "${config.packages.rsdd}/bin/bottomup_cnf_to_bdd";
        };
        bottomup_formula_to_bdd = {
          type = "app";
          program = "${config.packages.rsdd}/bin/bottomup_formula_to_bdd";
        };
        weighted_model_count = {
          type = "app";
          program = "${config.packages.rsdd}/bin/weighted_model_count";
        };
      };
      devShells.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          rustc
          cargo
          cargo-nextest
          rustfmt
          rust-analyzer
          clippy

          # extras
          lldb
          cargo-watch
          cargo-expand # expand macros and inspect the output
          cargo-llvm-lines # count number of lines of LLVM IR of a generic function
          cargo-criterion
        ]
        ++ lib.optionals stdenv.isDarwin []
        ++ lib.optionals stdenv.isLinux [
          cargo-rr
          rr-unstable
        ];
      };
    };
  };
}
