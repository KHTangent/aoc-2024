{
	description = "My Advent of Code 2024 solutions";

	inputs = {
		nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
		flake-utils.url = "github:numtide/flake-utils";
	};

	outputs = inputs@{ self, nixpkgs, flake-utils, ... }:
	flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
		let
			pkgs = nixpkgs.legacyPackages.${system};
		in {
			devShell = pkgs.mkShell {
				CARGO_INSTALL_ROOT = "${toString ./.}/.cargo";

				buildInputs = with pkgs; [ cargo rustc git rust-analyzer rustfmt python3 ];
			};
		}
	);
}
