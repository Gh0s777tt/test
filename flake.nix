{
  description = "Vantis OS Dev Environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ (import rust-overlay) ];
      };
    in {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          qemu
          nasm
          (rust-bin.nightly.latest.default.override {
            extensions = [ "rust-src" "llvm-tools-preview" ];
          })
          just
        ];

        shellHook = ''
          echo "🔮 VANTIS OS ENV LOADED (NIX)"
          echo ">> Run 'just run' to boot."
        '';
      };
    };
}
