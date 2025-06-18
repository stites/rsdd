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
  cargoHash = "sha256-sMMxm+++gYEE+ODyYNaJo5Yz8vJysvFEjyoSJlnwCm4=";
  buildFeatures = [ "ffi" "extras" ];
  # buildType = "debug";  # note that there is a bug in release mode for the ffi

  meta = with lib; {
    description = "Rust decision diagrams";
    homepage = "https://github.com/neuppl/rsdd";
    license = licenses.mit;
    maintainers = with maintainers; [];
  };
}
