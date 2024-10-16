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
        buildInputs = inputs.hc-infra.outputs.lib.holochainDeps {
          inherit lib;
          pkgs = inputs'.webkitgtknixpkgs.legacyPackages;
        } ++ self.lib.tauriAppDeps.buildInputs {
          inherit lib;
          pkgs = inputs'.webkitgtknixpkgs.legacyPackages;
        };
        nativeBuildInputs = (self.lib.tauriAppDeps.nativeBuildInputs {
          inherit lib;
          pkgs = inputs'.webkitgtknixpkgs.legacyPackages;
        });

        # TODO: remove this if possible
        # Without this build fails on MacOs
        postPatch = ''
          mkdir -p "$TMPDIR/nix-vendor"
          cp -Lr "$cargoVendorDir" -T "$TMPDIR/nix-vendor"
          sed -i "s|$cargoVendorDir|$TMPDIR/nix-vendor/|g" "$TMPDIR/nix-vendor/config.toml"
          chmod -R +w "$TMPDIR/nix-vendor"
          cargoVendorDir="$TMPDIR/nix-vendor"
        '';
      };
      # cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
      #   pname = crate;
      #   version = cargoToml.package.version;
      # });
      binary = craneLib.buildPackage (commonArgs // {
        pname = crate;
        version = cargoToml.package.version;
        # inherit cargoArtifacts;
      });
    in pkgs.runCommandLocal "wrap-${crate}" {
      buildInputs = [ pkgs.makeWrapper ];

    } ''
      mkdir $out
      mkdir $out/bin
      # Because we create this ourself, by creating a wrapper
      makeWrapper ${binary}/bin/hc-pilot $out/bin/hc-pilot \
        --set WEBKIT_DISABLE_DMABUF_RENDERER 1
    '';
  };
}
