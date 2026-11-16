# Environnement de développement : ./gate.sh (tests et clippy Rust, tests Lua et Python, modèle du pilote C) et
# `cargo run`. Avec direnv : `use flake` dans un .envrc (non suivi).
{ pkgs }:
let
  # srt.rs vise libsrt sous le nom de fichier de Debian : ce nom pointe vers la libsrt de nixpkgs, pour
  # l'édition de liens comme pour l'exécution.
  srtCompat = pkgs.runCommand "libsrt-gnutls-compat" { } ''
    mkdir -p $out/lib
    ln -s ${pkgs.srt}/lib/libsrt.so.1.5 $out/lib/libsrt-gnutls.so.1.5
  '';
in
pkgs.mkShell {
  packages = with pkgs; [
    cargo
    rustc
    clippy
    rustfmt
    pkg-config
    libusb1
    python3
    lua5_4
    gnumake
    # lancés par l'appli
    ffmpeg
    gst_all_1.gstreamer
    pipewire
    pulseaudio
    xdg-user-dirs
  ];

  RUSTFLAGS = "-L native=${srtCompat}/lib";

  GST_PLUGIN_SYSTEM_PATH_1_0 = pkgs.lib.makeSearchPath "lib/gstreamer-1.0" [
    pkgs.gst_all_1.gstreamer.out
    pkgs.gst_all_1.gst-plugins-base
    pkgs.pipewire.out
  ];
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (
    with pkgs;
    [
      srtCompat
      srt
      wayland
      libxkbcommon
      libGL
      vulkan-loader
      libx11
      libxcursor
      libxi
      libxrandr
    ]
  );
}
