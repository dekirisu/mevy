{
  description = "mevy documentation site — powered by mdBook";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  }: flake-utils.lib.eachDefaultSystem (system: let
    pkgs = import nixpkgs { inherit system; };
  in {
    devShells.default = pkgs.mkShell {
      name = "mevy-docs";
      buildInputs = [ pkgs.mdbook ];
      shellHook = ''
        cd "$PWD"
        echo "✨ mevy-docs dev shell"
        echo "   Run 'mdbook serve' to start the dev server"
        echo "   Run 'mdbook build' to build the site"
      '';
    };

    packages.dev = self.devShells.${system}.default;
  });
}
