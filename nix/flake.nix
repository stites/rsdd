{
  inputs = {
    #devenv.url = "github:cachix/devenv";
    #
    nci.url = "github:yusdacra/nix-cargo-integration";
    flake-parts.follows = "nci/parts";
    nixpkgs.follows = "nci/nixpkgs";
    ## devenv-root = {
    ##   url = "file+file:///dev/null";
    ##   flake = false;
    ## };
  };
  outputs = inputs @ {...}:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        inputs.nci.flakeModule
      ];
      systems = ["x86_64-linux"];
      perSystem = {
        self',
        config,
        pkgs,
        ...
      }: {
        packages = rec {
          default = rsdd;
          rsdd = pkgs.callPackage ./. {};
          rsdd-nocheck = self'.packages.rsdd.overrideAttrs (_: {doCheck = false;});
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
        #devenv.shells.default = {
        #  # devenv.root = let
        #  #   devenvRootFileContent = builtins.readFile inputs.devenv-root.outPath;
        #  # in
        #  #   pkgs.lib.mkIf (devenvRootFileContent != "") devenvRootFileContent;

        #  pre-commit.hooks = {
        #    shellcheck.enable = true;
        #    clippy.enable = true;
        #    hunspell.enable = true;
        #    alejandra.enable = true;
        #    rustfmt.enable = true;
        #    typos.enable = true;
        #  };
        #  languages.rust.enable = true;
        #  #languages.rust.version = "stable";
        #  scripts.repl.exec = "${pkgs.evcxr}/bin/evcxr";
        #  packages = with pkgs;
        #    [
        #      lldb
        #      cargo
        #      rustc
        #      rustfmt
        #      rust-analyzer
        #      clippy
        #      cargo-watch
        #      cargo-nextest
        #      cargo-expand # expand macros and inspect the output
        #      cargo-llvm-lines # count number of lines of LLVM IR of a generic function
        #      cargo-inspect
        #      cargo-criterion
        #      evcxr # make sure repl is in a gc-root
        #      cargo-play # quickly run a rust file that has a maint function
        #    ]
        #    ++ lib.optionals stdenv.isDarwin []
        #    ++ lib.optionals stdenv.isLinux [
        #      cargo-rr
        #      rr-unstable
        #    ];
        #};
      };
    };
}
