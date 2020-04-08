with (import <nixpkgs> {});

mkShell rec {
  nativeBuildInputs = [
    cargo 
  ];
}
