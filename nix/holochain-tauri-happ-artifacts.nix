{ craneLib, buildInputs, nativeBuildInputs, debug ? false }:
let
  src = craneLib.cleanCargoSource (craneLib.path ./..);
  commonArgs = {
    inherit src buildInputs nativeBuildInputs;
    CARGO_PROFILE = "release";
    cargoLock = ./reference-tauri-happ/Cargo.lock;

    doCheck = false;
    cargoExtraArgs = "--tests -p reference-tauri-happ";
  };
  cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
    pname = "tauri-happ";
    version = "for-holochain-0.3.2";
  });
in cargoArtifacts
