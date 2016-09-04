with import <nixpkgs> {}; {
  rustGameDevEnv = stdenv.mkDerivation {
    name = "rust-game-dev-env";
    buildInputs = [
      rustc
      cargo

      stdenv
      cmake
      xorg.libX11
      xorg.libXrandr
      xorg.libXinerama
      #libXext
      xorg.libXcursor
      xorg.libXxf86vm
      xorg.libXi
      mesa
    ];

    shellHook = ''
        unset http_proxy
        unset SSL_CERT_FILE
        export LD_LIBRARY_PATH="${xorg.libX11.out}/lib:\
        ${xorg.libX11.out}/lib:\
        ${xorg.libXrandr.out}/lib:\
        ${xorg.libXinerama.out}/lib:\
        ${xorg.libXxf86vm.out}/lib:\
        ${xorg.libXi.out}/lib:\
        ${xorg.libXcursor.out}/lib:$LD_LIBRARY_PATH"
    '';
  };
}
