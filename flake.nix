{
	description = "DegenTech website generator-ish";
	inputs.nixpkgs.url = "nixpkgs/nixos-25.11";

	outputs = { self, nixpkgs }:
	let
		supportedSystems = [ "x86_64-linux" ]; # "x86_64-darwin" "aarch64-linux" "aarch64-darwin" ]
		forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
	in
	{
		devShells =
		forAllSystems
		(
			system:
			let
				pkgs = import nixpkgs { inherit system; };
				overrides = (builtins.fromTOML (builtins.readFile (self + "/rust-toolchain.toml")));
			in
			{
				default = pkgs.mkShell
				rec {
					nativeBuildInputs = [ pkgs.pkg-config ];
					buildInputs = with pkgs; [ rustup clang llvmPackages.bintools ];

					RUSTC_VERSION = overrides.toolchain.channel;

					# https://github.com/rust-lang/rust-bindgen#environment-variables
					LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];

					shellHook = ''
						export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
						export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
					'';

					# Add precompiled library to rustc search path
					RUSTFLAGS = (builtins.map (a: ''-L ${a}/lib'') [
						# add libraries here (e.g. pkgs.libvmi)
					]);

					LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (buildInputs ++ nativeBuildInputs);


					# Add glibc, clang, glib, and other headers to bindgen search path
					BINDGEN_EXTRA_CLANG_ARGS =
					# Includes normal include path
					(builtins.map (a: ''-I"${a}/include"'') [
						# add dev libraries here (e.g. pkgs.libvmi.dev)
						pkgs.glibc.dev
					])
					# Includes with special directory paths
					++ [
						''-I"${pkgs.llvmPackages_latest.libclang.lib}/lib/clang/${pkgs.llvmPackages_latest.libclang.version}/include"''
						''-I"${pkgs.glib.dev}/include/glib-2.0"''
						''-I${pkgs.glib.out}/lib/glib-2.0/include/''
					];
				};
			}
		);
	};
}
