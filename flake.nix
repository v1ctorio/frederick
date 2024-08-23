{
  description = "A Nix-flake-based rust development environment for frederick";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self , nixpkgs ,... }: let
    system = "x86_64-linux";
  in {
    devShells."${system}".default = let
      pkgs = import nixpkgs {
        inherit system;
      };
    in pkgs.mkShell {
      packages = with pkgs; [
        rustc
        rust-analyzer
        rustfmt
        cargo
        git
        
        pkg-config
        openssl
      ];

      shellHook = ''
        alias fred ./target/debug/frederick
        echo "Rust 🦀 `${pkgs.rustc}/bin/rustc --version`"
      '';
    };
  };
}
