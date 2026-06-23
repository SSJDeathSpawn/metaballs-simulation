{
  description = "Rust development shell";

  inputs = {
    nixpkgs.url = "nixpkgs";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { 
    self, 
    nixpkgs,
    fenix,
  }: 
  let 
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      overlays = [fenix.overlays.default];
    };
    rust = fenix.packages.${system}.stable.withComponents [
      "cargo"
      "clippy"
      "rust-src"
      "rustc"
      "rustfmt"
      "rust-analyzer"
    ];
  in
  {

    devShells.x86_64-linux.default = pkgs.mkShell {
      shellHook = ''
        export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath [ pkgs.SDL2 ]}:$LD_LIBRARY_PATH
      '';
      packages = [
          rust
          pkgs.SDL2
      ];
    };
  };
}
