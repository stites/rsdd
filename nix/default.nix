{
  lib,
  rustPlatform,
  buildType ? "debug"
}:
rustPlatform.buildRustPackage rec {
  pname = "rsdd";
  version = "unstable-2023-11-11";

  src = ../.;

  cargoHash = "sha256-Dfiu7FKL5R72SgxmLCGDZnB+Zlo5RL6XGuzfgwokHWs=";
  cargoLock.lockFile = ./Cargo.lock;
  postPatch = "ln -s ${./Cargo.lock} Cargo.lock";

  useNextest = true;
  buildFeatures = [ "ffi" ];
  inherit buildType;

  meta = with lib; {
    description = "Rust decision diagrams";
    homepage = "https://github.com/neuppl/rsdd";
    license = licenses.mit;
    maintainers = with maintainers; [ stites ];
  };
}
