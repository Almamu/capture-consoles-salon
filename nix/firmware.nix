# Firmwares du boîtier (audio et vidéo), chargés en RAM à chaque branchement. Ils ne sont pas dans le dépôt
# (licence d'AVerMedia) : on les extrait du pilote Windows du GL310 archivé sur archive.org, le même fichier que
# tools/fetch-vendor.sh (md5 6aa363d7d7458dd08f021277353d6f2f). Licence non libre : à autoriser par son nom
# (nixpkgs.config.allowUnfreePredicate, nom « gl310-firmware »).
{
  lib,
  stdenvNoCC,
  fetchurl,
  unzip,
}:

stdenvNoCC.mkDerivation {
  pname = "gl310-firmware";
  version = "3.2802.64.40";

  src = fetchurl {
    name = "gl310-driver.zip";
    url = "https://archive.org/download/aver-media-gl-310-usb-hd-capture-device-v-3.2802.64.40/AVerMedia%20GL310%20USB%20HD%20Capture%20Device%20v3.2802.64.40.zip";
    hash = "sha256-iUojKCkebX3v/Hnk9Ws90dy8sFw4CUgRtJ3ljEG4Aik=";
  };

  nativeBuildInputs = [ unzip ];

  unpackPhase = ''
    unzip -q $src qpaudfwusb.bin qpvidfwusb.bin
  '';

  installPhase = ''
    install -Dm644 -t $out qpaudfwusb.bin qpvidfwusb.bin
  '';

  meta = {
    description = "Audio and video firmware for the AVerMedia Live Gamer Portable Lite (GL310)";
    homepage = "https://archive.org/details/aver-media-gl-310-usb-hd-capture-device-v-3.2802.64.40";
    license = lib.licenses.unfree;
    sourceProvenance = [ lib.sourceTypes.binaryFirmware ];
    platforms = lib.platforms.all;
  };
}
