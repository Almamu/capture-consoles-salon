# GL310 Studio construit depuis ce dépôt. Les firmwares viennent de ./firmware.nix (dans le store) quand
# ~/.local/share/gl310-studio/firmware n'en a pas ; ffmpeg, GStreamer et les outils PipeWire sont dans le PATH
# de l'appli.
{
  lib,
  rustPlatform,
  makeWrapper,
  pkg-config,
  srt,
  libusb1,
  wayland,
  libxkbcommon,
  libGL,
  vulkan-loader,
  libx11,
  libxcursor,
  libxi,
  libxrandr,
  ffmpeg,
  gst_all_1,
  pipewire,
  pulseaudio,
  xdg-user-dirs,
  gl310-firmware,
}:

rustPlatform.buildRustPackage {
  pname = "gl310-studio";
  version = "0.1.0";

  src = lib.fileset.toSource {
    root = ../studio-rs;
    fileset = lib.fileset.unions [
      ../studio-rs/Cargo.toml
      ../studio-rs/Cargo.lock
      ../studio-rs/src
      ../studio-rs/assets
    ];
  };

  cargoLock.lockFile = ../studio-rs/Cargo.lock;

  postPatch = ''
    # srt.rs vise le nom de fichier de Debian (pas de paquet -dev là-bas) ; nixpkgs a libsrt tout court.
    substituteInPlace src/srt.rs \
      --replace-fail '#[link(name = "libsrt-gnutls.so.1.5", kind = "dylib", modifiers = "+verbatim")]' \
                     '#[link(name = "srt")]'
    # Sans copie dans ~/.local/share/gl310-studio/firmware, les firmwares du store plutôt que vendor/.
    substituteInPlace src/main.rs \
      --replace-fail 'project.join("vendor/gl310-driver/extracted")' \
                     'PathBuf::from("${gl310-firmware}")'
  '';

  nativeBuildInputs = [
    makeWrapper
    pkg-config
  ];

  buildInputs = [
    srt
    libusb1
  ];

  postInstall = ''
    install -Dm644 assets/gl310-studio.svg $out/share/icons/hicolor/scalable/apps/gl310-studio.svg
    install -Dm644 assets/gl310-studio.png $out/share/icons/hicolor/256x256/apps/gl310-studio.png
    install -Dm644 /dev/stdin $out/share/applications/gl310-studio.desktop <<DESKTOP
    [Desktop Entry]
    Type=Application
    Name=GL310 Studio
    GenericName=HDMI capture
    GenericName[es]=Captura HDMI
    GenericName[fr]=Capture HDMI
    Comment=Preview, record and share the AVerMedia Live Gamer Portable Lite as a PipeWire camera
    Comment[es]=Vista previa, grabación y cámara PipeWire de la AVerMedia Live Gamer Portable Lite
    Comment[fr]=Aperçu, enregistrement et caméra virtuelle du boîtier AVerMedia Live Gamer Portable Lite
    Exec=gl310-studio
    Icon=gl310-studio
    Terminal=false
    Categories=AudioVideo;Video;Recorder;
    StartupWMClass=gl310-studio
    DESKTOP
  '';

  # winit et wgpu chargent les bibliothèques d'affichage et de GPU à l'exécution ; l'appli lance ffmpeg,
  # gst-launch-1.0, pactl, pw-cat, pw-link et xdg-user-dir par leur nom.
  postFixup = ''
    patchelf --add-rpath ${
      lib.makeLibraryPath [
        wayland
        libxkbcommon
        libGL
        vulkan-loader
        libx11
        libxcursor
        libxi
        libxrandr
      ]
    } $out/bin/gl310-studio
    wrapProgram $out/bin/gl310-studio \
      --prefix PATH : ${
        lib.makeBinPath [
          ffmpeg
          gst_all_1.gstreamer
          pipewire
          pulseaudio
          xdg-user-dirs
        ]
      } \
      --prefix GST_PLUGIN_SYSTEM_PATH_1_0 : ${
        lib.makeSearchPath "lib/gstreamer-1.0" [
          gst_all_1.gstreamer.out
          gst_all_1.gst-plugins-base
          pipewire.out
        ]
      }
  '';

  meta = {
    description = "Capture app for the AVerMedia Live Gamer Portable Lite (GL310) with a userspace USB driver";
    homepage = "https://github.com/Almamu/capture-consoles-salon";
    license = lib.licenses.mit;
    mainProgram = "gl310-studio";
    platforms = lib.platforms.linux;
  };
}
