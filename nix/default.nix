{
  lib,
  rustPlatform,
  fetchFromGitHub,
}:
rustPlatform.buildRustPackage rec {
  pname = "rsdd";
  version = "0.1.0";

  src = ../.;

  cargoHash = "sha256-GYO554P3VHilJm2z/BM8noiPKG/HQM5jSELwuXAHJAA=";

  cargoPatches = [./0001-Cargo.lock.patch];

  meta = with lib; {
    description = "Rust decision diagrams";
    homepage = "https://github.com/neuppl/rsdd";
    license = licenses.mit;
    maintainers = with maintainers; [ stites ];
  };
}
