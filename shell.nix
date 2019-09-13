with import <nixpkgs> {};

runCommand "dummy" {
	buildInputs = [
		clang
    libGL
		wayland-protocols
    wayland
    #xorg.libX11
		pkg-config
	];
	# LD_LIBRARY_PATH = "/nix/store/8z6xdhf6z2lb1y6h7380p3q8x2ya38d2-wayland-1.17.0/lib";
	ELECTRON_TRASH = "gio";
} ""
