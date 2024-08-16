{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:
  let pkgs = import nixpkgs{ system = "x86_64-linux"; };

  in
  {
    devShell.x86_64-linux = pkgs.mkShell {
    packages = [
      pkgs.rustfmt
      pkgs.cargo
      pkgs.rust-analyzer
      pkgs.rustc
    ];
    };
  };

}
