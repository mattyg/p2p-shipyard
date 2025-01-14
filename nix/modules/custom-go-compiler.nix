{ ... }:

{
  perSystem = { inputs', lib, self', system, pkgs, ... }: {
    packages.custom-go-wrapper = let
      go = lib.overrideDerivation pkgs.go (attrs: rec {
        version = "1.21";
        name = "custom-go-${version}-dev";
        src = let
          gitSrc = pkgs.fetchgit {
            url = "https://github.com/wlynxg/go";
            rev = "bff8d409ebfb8d4c8488325f13cb212b07cf6bb4";
            sha256 = "i5MnEkFSEhy+D4C+Syyc0Xkch248VD75ccvQlsMB/6U=";
          };
          finalGo = pkgs.runCommandNoCC "custom-go" { } ''
            mkdir $out
            cd ${gitSrc}
            cp -R . $out
            ls $out

            echo "go${version}" > $out/VERSION
          '';
        in finalGo;
        buildInputs = with pkgs; [ pcre git ];
        nativeBuildInputs = with pkgs; [ pcre git ];
      });
    in go;
  };
}
