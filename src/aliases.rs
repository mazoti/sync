/// String array with all check command alias sorted in lexicographic order
pub const CHECK_SORTED: &[&str] = &[
    "--CHECK", "--check", "-C", "-CHECK", "-c", "-check", "/C", "/CHECK", "/c", "/check", "CHECK",
    "check",
];

/// String array with all duplicate command alias sorted in lexicographic order
#[cfg(feature = "i18n")]
pub const DUPLICATE_SORTED: &[&str] = &[
    "--DUPLICATE",
    "--duplicate",
    "-D",
    "-DUPLICATE",
    "-d",
    "-duplicate",
    "/D",
    "/DUPLICATE",
    "/d",
    "/duplicate",
    "DUPLICATE",
    "duplicate",
];

/// String array with all empty command alias sorted in lexicographic order
#[cfg(feature = "i18n")]
pub const EMPTY_SORTED: &[&str] = &[
    "--EMPTY", "--empty", "-E", "-EMPTY", "-e", "-empty", "/E", "/EMPTY", "/e", "/empty", "EMPTY",
    "empty",
];

/// String array with all force command alias sorted in lexicographic order
pub const FORCE_SORTED: &[&str] = &[
    "--FORCE", "--force", "-F", "-FORCE", "-f", "-force", "/F", "/FORCE", "/f", "/force", "FORCE",
    "force",
];

/// String array with all force command alias sorted in lexicographic order
pub const HASH_SORTED: &[&str] = &[
    "--HASH", "--hash", "-HASH", "-hash", "/HASH", "/hash", "HASH", "hash",
];

/// String array with all help command alias sorted in lexicographic order
#[cfg(feature = "i18n")]
pub const HELP_SORTED: &[&str] = &[
    "--HELP", "--help", "-?", "-H", "-h", "-help", "/?", "/H", "/HELP", "/h", "/help", "HELP",
    "help",
];

/// String array with all join command alias sorted in lexicographic order
pub const JOIN_SORTED: &[&str] = &[
    "--JOIN", "--join", "-J", "-JOIN", "-j", "-join", "/J", "/JOIN", "/j", "/join", "JOIN", "join",
];

/// String array with all move command alias sorted in lexicographic order
pub const MOVE_SORTED: &[&str] = &[
    "--MOVE", "--move", "-M", "-MOVE", "-m", "-move", "/M", "/MOVE", "/m", "/move", "MOVE", "move",
];

/// String array with all simulate command alias sorted in lexicographic order
#[cfg(feature = "i18n")]
pub const SIMULATE_SORTED: &[&str] = &[
    "--SIMULATE",
    "--simulate",
    "-S",
    "-SIMULATE",
    "-s",
    "-simulate",
    "/S",
    "/SIMULATE",
    "/s",
    "/simulate",
    "SIMULATE",
    "simulate",
];

/// String array with all split command alias sorted in lexicographic order
pub const SPLIT_SORTED: &[&str] = &[
    "--SPLIT", "--split", "-S", "-SPLIT", "-s", "-split", "/S", "/SPLIT", "/s", "/split", "SPLIT",
    "split",
];

/// String array with all version command alias sorted in lexicographic order
#[cfg(feature = "i18n")]
pub const VERSION_SORTED: &[&str] = &[
    "--VERSION",
    "--version",
    "-V",
    "-VERSION",
    "-v",
    "-version",
    "/V",
    "/VERSION",
    "/v",
    "/version",
    "VERSION",
    "version",
];
