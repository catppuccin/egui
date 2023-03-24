{
  description = "";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          name = "catppuccin-egui";
          packages = with pkgs; [
            cargo
            rustc
            clippy
            rustfmt
            cargo-semver-checks
            rust-analyzer
          ];
          # I use Wayland so this probably won't work if you're on X. Please
          # send a PR if you do get it to work for X.
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [
            wayland
            libGL
            libxkbcommon
          ]);
        };
      }
    );
}
