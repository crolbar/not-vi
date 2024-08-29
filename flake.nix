{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    ...
  }: let
    pkgs = import nixpkgs {
      system = "x86_64-linux";
      overlays = [(import rust-overlay)];
    };
  in
    with pkgs; {
      devShells.${pkgs.system}.default = mkShell {
        buildInputs = [
          pkg-config

          (rust-bin.stable.latest.default.override {
            extensions = ["rust-src" "rust-analyzer"];
          })
        ];
      };
    };
}
