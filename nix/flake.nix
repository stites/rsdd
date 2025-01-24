{
  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:nixos/nixpkgs/release-24.11";
  };
  outputs = inputs @ {...}:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
      ];
      systems = ["x86_64-linux"];
      perSystem = {
        config,
        pkgs,
        ...
      }: {
        packages = rec {
          default = rsdd-debug;
          rsdd-debug = pkgs.callPackage ./. { buildType = "debug"; };
          rsdd-debug-nocheck = config.packages.rsdd-debug.overrideAttrs (_: {doCheck = false;});
          rsdd-release = pkgs.callPackage ./. { buildType = "release"; };
          rsdd-release-nocheck = config.packages.rsdd-release.overrideAttrs (_: {doCheck = false;});
          render-graphviz = let
            py = pkgs.python3.withPackages (p: [p.graphviz]);
          in
            pkgs.writeScriptBin "render-graphviz" ''
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
      };
    };
}
