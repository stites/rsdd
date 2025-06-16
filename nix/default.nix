{
  lib,
  rustPlatform,
  fetchFromGitHub,
}:
rustPlatform.buildRustPackage rec {
  pname = "rsdd";
  version = "unstable-2025-06-16";

  src = ../.;

  cargoPatches = [./0001-Cargo.lock.patch];
  cargoHash = "sha256-H/82SUvZ2h9FPZOJiNpQa1VTiXmOrRI8K+6srhIcQPA=";
  buildFeatures = [ "ffi" ];
  # buildType = "debug";  # note that there is a bug in release mode for the ffi

  meta = with lib; {
    description = "Rust decision diagrams";
    homepage = "https://github.com/neuppl/rsdd";
    license = licenses.mit;
    maintainers = with maintainers; [];
  };
}
