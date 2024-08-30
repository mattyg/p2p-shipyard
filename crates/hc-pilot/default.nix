{ inputs, self, ... }:

{
  perSystem = { inputs', pkgs, system, lib, ... }: {
    packages.hc-pilot = let
      craneLib = inputs.crane.mkLib pkgs;

      cratePath = ./.;

      cargoToml =
        builtins.fromTOML (builtins.readFile "${cratePath}/Cargo.toml");
      crate = cargoToml.package.name;

      commonArgs = {
        src =
          (self.lib.cleanTauriSource { inherit lib; }) (craneLib.path ../../.);
        doCheck = false;
        buildInputs = inputs.hc-infra.outputs.lib.holochainAppDeps.buildInputs {
          inherit pkgs lib;
        } ++ self.lib.tauriAppDeps.buildInputs {
          inherit lib;
          pkgs = inputs'.webkitgtknixpkgs.legacyPackages;
        };
        nativeBuildInputs = (self.lib.tauriAppDeps.nativeBuildInputs {

          inherit lib;
          pkgs = inputs'.webkitgtknixpkgs.legacyPackages;
        }) ++ (inputs.hc-infra.outputs.lib.holochainAppDeps.nativeBuildInputs {
          inherit pkgs lib;
        });

        # TODO: remove this if possible
        postPatch = ''
          mkdir -p "$TMPDIR/nix-vendor"
          cp -Lr "$cargoVendorDir" -T "$TMPDIR/nix-vendor"
          sed -i "s|$cargoVendorDir|$TMPDIR/nix-vendor/|g" "$TMPDIR/nix-vendor/config.toml"
          chmod -R +w "$TMPDIR/nix-vendor"
          cargoVendorDir="$TMPDIR/nix-vendor"
        '';
      };
    in craneLib.buildPackage (commonArgs // {
      pname = crate;
      version = cargoToml.package.version;
    });
  };
}
