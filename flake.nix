{
  description = "wayland-shell";

  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        rust = pkgs.rustPlatform;
      in
      rec {
        defaultPackage = rust.buildRustPackage rec {
          pname = "wayland-shell";
          version = "0.1.0";
          src = pkgs.fetchFromGitHub {
            owner = "nilsherzig";
            repo = pname;
            rev = "b7e0ad694091bdd8ae7aa2c954bb46808fb9169a";
            hash = "sha256-0atKUeTQo4TK/bi6+fdmOAPkFXo6e6Si3gkqimA13RM=";
          };
          cargoHash = "sha256-5gfjU43SA19FzLYgD518N0DQE9MimyktswmJAQS2jSM=";

          nativeBuildInputs = with pkgs; [
            cairo.dev
            gdk-pixbuf.dev
            glib.dev
            graphene.dev
            gtk4-layer-shell
            gtk4.dev
            harfbuzz.dev
            pango.dev
            pkg-config
          ];

          PKG_CONFIG_PATH = builtins.concatStringsSep ":" (
            map (x: "${x}/lib/pkgconfig") nativeBuildInputs
          );
        };

        gtk4-layer-shell = pkgs.stdenv.mkDerivation rec {
          pname = "gtk4-layer-shell";
          version = "1.0.0";
          src = pkgs.fetchFromGitHub {
            owner = "wmww";
            repo = pname;
            rev = "v${version}";
            hash = "sha256-8bf7O/y9gQohd9ZLc7wygUeZxtU5RAsn1PW8pg0NcAc=";
          };
          nativeBuildInputs = with pkgs; [
            gnome2.gtk-doc
            gobject-introspection.dev
            gtk4.dev
            meson
            ninja
            pkg-config
            vala
            wayland-protocols
          ];
          configurePhase = ''
            meson setup \
              -Dexamples=true \
              -Dprefix="$out" \
              build
          '';
          buildPhase = ''
            ninja -C build
          '';
          installPhase = ''
            ninja -C build install
          '';
          meta.pkgConfigModules = [ "gtk4-layer-shell-0" ];
        };
      });
}
