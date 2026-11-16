//! Traductions de l'interface et du journal. Le français reste dans le code (`t("…")`, `tf("… {} …", &[&x])`) et
//! sert de clé ; l'anglais et l'espagnol sont dans `TABLE`. La langue vient de LC_ALL, LC_MESSAGES puis LANG
//! (fr → français, es → espagnol, autre → anglais). Une clé absente reste en français.

use std::fmt::Display;
use std::sync::OnceLock;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lang {
    Fr,
    En,
    Es,
}

/// Langue d'après les variables de locale, dans l'ordre de priorité de POSIX (la première non vide gagne).
pub fn lang_from(var: impl Fn(&str) -> Option<String>) -> Lang {
    let locale = ["LC_ALL", "LC_MESSAGES", "LANG"].iter().find_map(|v| var(v).filter(|s| !s.is_empty())).unwrap_or_default();
    match locale.get(..2) {
        Some("fr") => Lang::Fr,
        Some("es") => Lang::Es,
        _ => Lang::En,
    }
}

pub fn lang() -> Lang {
    static LANG: OnceLock<Lang> = OnceLock::new();
    *LANG.get_or_init(|| lang_from(|v| std::env::var(v).ok()))
}

fn translate(lang: Lang, fr: &'static str) -> &'static str {
    let Some(&(_, en, es)) = TABLE.iter().find(|(key, _, _)| *key == fr) else { return fr };
    match lang {
        Lang::Fr => fr,
        Lang::En => en,
        Lang::Es => es,
    }
}

/// Texte sans valeur à insérer.
pub fn t(fr: &'static str) -> &'static str {
    translate(lang(), fr)
}

/// Texte avec des valeurs : chaque `{}` reçoit la suivante, dans l'ordre.
pub fn tf(fr: &'static str, args: &[&dyn Display]) -> String {
    fill(translate(lang(), fr), args)
}

fn fill(text: &str, args: &[&dyn Display]) -> String {
    let mut out = String::with_capacity(text.len() + 16);
    let mut args = args.iter();
    let mut rest = text;
    while let Some(at) = rest.find("{}") {
        out.push_str(&rest[..at]);
        if let Some(arg) = args.next() {
            out.push_str(&arg.to_string());
        }
        rest = &rest[at + 2..];
    }
    out.push_str(rest);
    out
}

/// (français, anglais, espagnol).
const TABLE: &[(&str, &str, &str)] = &[
    // Interface
    ("Encodeur", "Encoder", "Codificador"),
    ("Arrêter l'encodeur", "Stop encoder", "Detener codificador"),
    ("Démarrer l'encodeur", "Start encoder", "Iniciar codificador"),
    ("Enregistrement", "Recording", "Grabación"),
    ("Arrêter l'enregistrement", "Stop recording", "Detener grabación"),
    ("Enregistrer", "Record", "Grabar"),
    ("Conversion en MP4…", "Converting to MP4…", "Convirtiendo a MP4…"),
    ("Réglages", "Settings", "Ajustes"),
    ("Débit vidéo", "Video bitrate", "Tasa de vídeo"),
    ("Images clés", "Keyframes", "Fotogramas clave"),
    ("toutes les {} s", "every {} s", "cada {} s"),
    ("Débit audio", "Audio bitrate", "Tasa de audio"),
    ("Son sur la prise AUDIO OUT", "Sound on the AUDIO OUT jack", "Sonido por la toma AUDIO OUT"),
    ("Appliquer", "Apply", "Aplicar"),
    (
        "« Appliquer » relance l'encodeur (1 à 2 s de coupure).",
        "“Apply” restarts the encoder (1 to 2 s gap).",
        "«Aplicar» reinicia el codificador (corte de 1 a 2 s).",
    ),
    (
        "Luminosité, contraste : filtre « Correction colorimétrique » dans OBS.",
        "Brightness, contrast: the “Color Correction” filter in OBS.",
        "Brillo, contraste: filtro «Corrección de color» en OBS.",
    ),
    ("Sorties", "Outputs", "Salidas"),
    ("Caméra PipeWire : GL310", "PipeWire camera: GL310", "Cámara PipeWire: GL310"),
    ("Son du jeu (entrée audio) : GL310-Audio", "Game sound (audio input): GL310-Audio", "Sonido del juego (entrada de audio): GL310-Audio"),
    (
        "OBS : « Capture de périphérique vidéo (PipeWire) » et « Capture d'entrée audio »",
        "OBS: “Video Capture Device (PipeWire)” and “Audio Input Capture”",
        "OBS: «Dispositivo de captura de vídeo (PipeWire)» y «Captura de entrada de audio»",
    ),
    (
        "SRT, latence fixe (OBS « Source média » sur la tour) :",
        "SRT, fixed latency (OBS “Media Source” on another PC):",
        "SRT, latencia fija (OBS «Fuente multimedia» en otro PC):",
    ),
    ("Copier l'adresse SRT", "Copy SRT address", "Copiar dirección SRT"),
    ("HTTP (secours, app web) :", "HTTP (fallback, web app):", "HTTP (respaldo, app web):"),
    ("Copier l'adresse HTTP", "Copy HTTP address", "Copiar dirección HTTP"),
    ("Journal", "Log", "Registro"),
    ("Pas d'image", "No picture", "Sin imagen"),
    ("{}×{} à {} Hz", "{}×{} at {} Hz", "{}×{} a {} Hz"),
    ("pas de signal HDMI", "no HDMI signal", "sin señal HDMI"),
    ("Boîtier non branché", "Device not connected", "Capturadora no conectada"),
    ("Démarrage du boîtier : {}…", "Starting the device: {}…", "Arrancando la capturadora: {}…"),
    ("Prêt, encodeur arrêté — {}", "Ready, encoder stopped — {}", "Lista, codificador detenido — {}"),
    ("En direct — {} — {} Mbit/s", "Live — {} — {} Mbit/s", "En directo — {} — {} Mbit/s"),
    ("Erreur : {}", "Error: {}", "Error: {}"),
    // Journal : appli et réglages
    ("réglages de {} ignorés : {}", "settings in {} ignored: {}", "ajustes de {} ignorados: {}"),
    ("réglages non sauvegardés ({}) : {}", "settings not saved ({}): {}", "ajustes no guardados ({}): {}"),
    ("réglages refusés : {}", "settings rejected: {}", "ajustes rechazados: {}"),
    ("débit vidéo {} kbit/s non proposé {}", "video bitrate {} kbit/s not offered {}", "tasa de vídeo de {} kbit/s no disponible {}"),
    ("intervalle d'images clés {} non proposé {}", "keyframe interval {} not offered {}", "intervalo de fotogramas clave {} no disponible {}"),
    ("débit audio {} kbit/s non proposé {}", "audio bitrate {} kbit/s not offered {}", "tasa de audio de {} kbit/s no disponible {}"),
    ("ligne illisible : {}", "unreadable line: {}", "línea ilegible: {}"),
    ("valeur illisible : {}", "unreadable value: {}", "valor ilegible: {}"),
    ("clé inconnue : {}", "unknown key: {}", "clave desconocida: {}"),
    ("Usage : gl310-studio [--listen 0.0.0.0:4848]", "Usage: gl310-studio [--listen 0.0.0.0:4848]", "Uso: gl310-studio [--listen 0.0.0.0:4848]"),
    // Journal : enregistrement
    ("enregistrement : {}", "recording: {}", "grabación: {}"),
    ("enregistrement prêt : {}", "recording ready: {}", "grabación lista: {}"),
    ("enregistrement : erreur interne", "recording: internal error", "grabación: error interno"),
    ("enregistrement arrêté, conversion en MP4 : {}", "recording stopped, converting to MP4: {}", "grabación detenida, convirtiendo a MP4: {}"),
    ("enregistrement impossible : {}", "cannot record: {}", "no se puede grabar: {}"),
    ("écriture de {} : {}", "writing {}: {}", "escritura de {}: {}"),
    (
        "flux perdu pendant l'enregistrement (disque trop lent ?)",
        "stream lost while recording (disk too slow?)",
        "flujo perdido durante la grabación (¿disco demasiado lento?)",
    ),
    ("ffmpeg introuvable : {}", "ffmpeg not found: {}", "ffmpeg no encontrado: {}"),
    (
        "conversion en MP4 impossible, le flux brut reste dans {}",
        "MP4 conversion failed, the raw stream stays in {}",
        "no se pudo convertir a MP4, el flujo en bruto sigue en {}",
    ),
    // Journal : boîtier et encodeur
    ("démarrage du boîtier impossible : {}", "could not start the device: {}", "no se pudo arrancar la capturadora: {}"),
    (
        "{} — débranche le boîtier 10 s puis rebranche-le",
        "{} — unplug the device for 10 s, then plug it back in",
        "{} — desconecta la capturadora 10 s y vuelve a conectarla",
    ),
    ("liaison USB perdue : {}", "USB link lost: {}", "conexión USB perdida: {}"),
    ("boîtier débranché", "device unplugged", "capturadora desconectada"),
    (
        "boîtier trouvé, firmware déjà en RAM : remise à zéro de l'encodeur",
        "device found, firmware already in RAM: resetting the encoder",
        "capturadora encontrada, firmware ya en RAM: reiniciando el codificador",
    ),
    ("boîtier trouvé : chargement des firmwares", "device found: loading the firmware", "capturadora encontrada: cargando el firmware"),
    ("démarrage : {}", "starting: {}", "arranque: {}"),
    ("firmwares chargés, boîtier prêt", "firmware loaded, device ready", "firmware cargado, capturadora lista"),
    ("encodeur arrêté", "encoder stopped", "codificador detenido"),
    ("encodeur démarré à {} Mbit/s", "encoder started at {} Mbit/s", "codificador iniciado a {} Mbit/s"),
    ("encodeur relancé sur le nouveau signal", "encoder restarted on the new signal", "codificador reiniciado con la nueva señal"),
    (
        "encodeur relancé : {} Mbit/s, images clés toutes les {} images, AAC {} kbit/s, AUDIO OUT {}",
        "encoder restarted: {} Mbit/s, keyframe every {} frames, AAC {} kbit/s, AUDIO OUT {}",
        "codificador reiniciado: {} Mbit/s, fotograma clave cada {} fotogramas, AAC {} kbit/s, AUDIO OUT {}",
    ),
    ("oui", "yes", "sí"),
    ("non", "no", "no"),
    ("signal HDMI : {}×{} à {} Hz", "HDMI signal: {}×{} at {} Hz", "señal HDMI: {}×{} a {} Hz"),
    (
        "pas de signal HDMI (console éteinte ou en veille ?)",
        "no HDMI signal (console off or asleep?)",
        "sin señal HDMI (¿consola apagada o en reposo?)",
    ),
    (
        "accès USB refusé (règle udev 70-avermedia-gl310 absente ?)",
        "USB access denied (udev rule 70-avermedia-gl310 missing?)",
        "acceso USB denegado (¿falta la regla udev 70-avermedia-gl310?)",
    ),
    (
        "boîtier déjà utilisé par un autre programme (ou le pilote noyau gl310)",
        "device already in use by another program (or the gl310 kernel driver)",
        "capturadora ya en uso por otro programa (o el driver de kernel gl310)",
    ),
    ("USB : {}", "USB: {}", "USB: {}"),
    (
        "le boîtier est en mode d835 : débranche-le 10 s puis rebranche-le",
        "the device is in d835 mode: unplug it for 10 s, then plug it back in",
        "la capturadora está en modo d835: desconéctala 10 s y vuelve a conectarla",
    ),
    (
        "écriture du profil permanent du micro ({}) refusée",
        "write to the microcontroller's permanent profile ({}) refused",
        "escritura del perfil permanente del microcontrolador ({}) rechazada",
    ),
    ("réponse courte à la lecture de {}", "short reply reading {}", "respuesta corta al leer {}"),
    ("données vides sur EP 0x81", "empty data on EP 0x81", "datos vacíos en EP 0x81"),
    ("relecture du firmware différente au mot {}", "firmware read-back differs at word {}", "la relectura del firmware difiere en la palabra {}"),
    ("réponse courte du micro (registre {})", "short reply from the microcontroller (register {})", "respuesta corta del microcontrolador (registro {})"),
    ("firmware {} illisible dans {} : {}", "firmware {} unreadable in {}: {}", "firmware {} ilegible en {}: {}"),
    ("taille de firmware non multiple de 4", "firmware size not a multiple of 4", "tamaño de firmware no múltiplo de 4"),
    ("initialisation officielle (206 écritures I2C)", "official initialisation (206 I2C writes)", "inicialización oficial (206 escrituras I2C)"),
    ("firmware audio", "audio firmware", "firmware de audio"),
    ("firmware vidéo", "video firmware", "firmware de vídeo"),
    ("firmware absent après le démarrage de l'ARM", "firmware missing after starting the ARM", "firmware ausente tras arrancar el ARM"),
    // Journal : sorties et réseau
    ("image", "video", "vídeo"),
    ("son", "sound", "sonido"),
    ("{} : {} — relance dans 2 s", "{}: {} — restarting in 2 s", "{}: {} — reinicio en 2 s"),
    ("entrée audio GL310-Audio non créée : {}", "audio input GL310-Audio not created: {}", "entrada de audio GL310-Audio no creada: {}"),
    (
        "entrée audio GL310-Audio non créée (pactl introuvable : {})",
        "audio input GL310-Audio not created (pactl not found: {})",
        "entrada de audio GL310-Audio no creada (pactl no encontrado: {})",
    ),
    ("ffmpeg introuvable ({})", "ffmpeg not found ({})", "ffmpeg no encontrado ({})"),
    ("gst-launch-1.0 introuvable ({})", "gst-launch-1.0 not found ({})", "gst-launch-1.0 no encontrado ({})"),
    ("pw-cat introuvable ({})", "pw-cat not found ({})", "pw-cat no encontrado ({})"),
    (
        "caméra PipeWire GL310 arrêtée : l'aperçu continue sans elle",
        "PipeWire camera GL310 stopped: the preview carries on without it",
        "cámara PipeWire GL310 detenida: la vista previa sigue sin ella",
    ),
    ("caméra PipeWire GL310 à relancer", "PipeWire camera GL310 needs a restart", "hay que reiniciar la cámara PipeWire GL310"),
    (
        "son du jeu : pw-cat arrêté, l'entrée GL310-Audio reste muette",
        "game sound: pw-cat stopped, the GL310-Audio input stays silent",
        "sonido del juego: pw-cat detenido, la entrada GL310-Audio queda muda",
    ),
    ("son du jeu relié à l'entrée audio GL310-Audio", "game sound linked to the GL310-Audio input", "sonido del juego enlazado a la entrada de audio GL310-Audio"),
    ("pw-cat à relancer", "pw-cat needs a restart", "hay que reiniciar pw-cat"),
    ("flux réseau : {}", "network stream: {}", "flujo de red: {}"),
    ("flux réseau indisponible sur {} : {}", "network stream unavailable on {}: {}", "flujo de red no disponible en {}: {}"),
    ("flux SRT : {}", "SRT stream: {}", "flujo SRT: {}"),
    ("flux SRT indisponible : {}", "SRT stream unavailable: {}", "flujo SRT no disponible: {}"),
    ("lecteur réseau {} : déconnecté", "network viewer {}: disconnected", "cliente de red {}: desconectado"),
    ("lecteur réseau {} : {}", "network viewer {}: {}", "cliente de red {}: {}"),
    ("SRT : attente d'appelant impossible ({})", "SRT: cannot wait for callers ({})", "SRT: no se puede esperar a clientes ({})"),
    ("lecteur SRT {} : connecté", "SRT viewer {}: connected", "cliente SRT {}: conectado"),
    ("lecteur SRT {} : déconnecté{}", "SRT viewer {}: disconnected{}", "cliente SRT {}: desconectado{}"),
];

#[cfg(test)]
mod tests {
    use super::*;

    /// Tous les fichiers où l'on traduit, pour vérifier que chaque texte a sa traduction.
    const SOURCES: [&str; 10] = [
        include_str!("app.rs"),
        include_str!("device.rs"),
        include_str!("engine.rs"),
        include_str!("http.rs"),
        include_str!("main.rs"),
        include_str!("model.rs"),
        include_str!("outputs.rs"),
        include_str!("recorder.rs"),
        include_str!("srt.rs"),
        include_str!("hub.rs"),
    ];

    /// Les littéraux passés à `t(` et `tf(` dans un fichier source (le littéral peut être à la ligne suivante).
    fn keys(source: &str) -> Vec<String> {
        let mut found = Vec::new();
        for call in ["t(", "tf("] {
            for (at, _) in source.match_indices(call) {
                if source[..at].chars().next_back().is_some_and(|c| c.is_alphanumeric() || c == '_') {
                    continue;
                }
                let Some(literal) = source[at + call.len()..].trim_start().strip_prefix('"') else { continue };
                found.push(literal[..literal.find('"').expect("littéral fermé")].to_string());
            }
        }
        found
    }

    #[test]
    fn chaque_texte_traduit_a_sa_traduction() {
        // Quand un texte passe par t() ou tf() dans le code, il doit être dans la table, sinon il resterait en
        // français pour tout le monde.
        let missing: Vec<String> =
            SOURCES.iter().flat_map(|s| keys(s)).filter(|k| !TABLE.iter().any(|(fr, _, _)| fr == k)).collect();
        assert!(missing.is_empty(), "absents de TABLE : {missing:?}");
        assert!(SOURCES.iter().map(|s| keys(s).len()).sum::<usize>() > 50, "le repérage des appels marche");
    }

    #[test]
    fn chaque_traduction_garde_les_memes_valeurs() {
        // Si une traduction a plus ou moins de {} que le français, une valeur se perdrait ou se déplacerait.
        for (fr, en, es) in TABLE {
            let n = fr.matches("{}").count();
            assert_eq!(en.matches("{}").count(), n, "anglais : {fr}");
            assert_eq!(es.matches("{}").count(), n, "espagnol : {fr}");
            assert!(!en.is_empty() && !es.is_empty(), "traduction vide : {fr}");
        }
        let mut keys: Vec<&str> = TABLE.iter().map(|(fr, _, _)| *fr).collect();
        keys.sort_unstable();
        let before = keys.len();
        keys.dedup();
        assert_eq!(keys.len(), before, "clé en double");
    }

    #[test]
    fn la_langue_suit_la_locale() {
        // LC_ALL l'emporte sur LC_MESSAGES, qui l'emporte sur LANG ; une variable vide ne compte pas ; une langue
        // non traduite donne l'anglais.
        let env = |pairs: &'static [(&'static str, &'static str)]| {
            move |v: &str| pairs.iter().find(|(k, _)| *k == v).map(|(_, x)| x.to_string())
        };
        assert_eq!(lang_from(env(&[("LANG", "es_ES.UTF-8")])), Lang::Es);
        assert_eq!(lang_from(env(&[("LANG", "fr_FR.UTF-8")])), Lang::Fr);
        assert_eq!(lang_from(env(&[("LANG", "de_DE.UTF-8")])), Lang::En);
        assert_eq!(lang_from(env(&[])), Lang::En);
        assert_eq!(lang_from(env(&[("LC_ALL", "fr_FR"), ("LANG", "es_ES")])), Lang::Fr);
        assert_eq!(lang_from(env(&[("LC_ALL", ""), ("LC_MESSAGES", "es_ES"), ("LANG", "fr_FR")])), Lang::Es);
        assert_eq!(lang_from(env(&[("LANG", "C.UTF-8")])), Lang::En);
    }

    #[test]
    fn les_valeurs_vont_dans_l_ordre() {
        assert_eq!(fill("{}×{} à {} Hz", &[&1920, &1080, &60]), "1920×1080 à 60 Hz");
        assert_eq!(translate(Lang::En, "{}×{} à {} Hz"), "{}×{} at {} Hz");
        assert_eq!(translate(Lang::Es, "texte inconnu"), "texte inconnu", "une clé absente reste en français");
        assert_eq!(fill("sans valeur", &[]), "sans valeur");
    }
}
