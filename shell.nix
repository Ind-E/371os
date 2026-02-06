{
  pkgs ? import <nixpkgs> {
    overlays = [
      (import (fetchTarball {
        url = "https://github.com/oxalica/rust-overlay/archive/master.tar.gz";
      }))
    ];
  },
}:

pkgs.mkShell {
  strictDeps = true;
  nativeBuildInputs = with pkgs; [
    (rust-bin.nightly.latest.default.override {
      extensions = [
        "rust-src"
        "rust-analyzer"
        "llvm-tools-preview"
      ];
    })
    cargo-bootimage
  ];

}
