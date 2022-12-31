mod processor;

const FN_ARGS: [fn(&std::time::Instant); 4] =
    [one_argument, two_arguments, three_arguments, four_arguments];

const CHECK_SORTED: &[&str] = &[
    "--CHECK", "--check", "-C", "-CHECK", "-c", "-check", "/C", "/CHECK", "/c", "/check", "CHECK",
    "check",
];

#[cfg(feature = "cli")]
const DUPLICATE_SORTED: &[&str] = &[
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

#[cfg(feature = "cli")]
const EMPTY_SORTED: &[&str] = &[
    "--EMPTY", "--empty", "-E", "-EMPTY", "-e", "-empty", "/E", "/EMPTY", "/e", "/empty", "EMPTY",
    "empty",
];

const FORCE_SORTED: &[&str] = &[
    "--FORCE", "--force", "-F", "-FORCE", "-f", "-force", "/F", "/FORCE", "/f", "/force", "FORCE",
    "force",
];

const HASH_SORTED: &[&str] = &[
    "--HASH", "--hash", "-HASH", "-hash", "/HASH", "/hash", "HASH", "hash",
];

#[cfg(feature = "cli")]
const HELP_SORTED: &[&str] = &[
    "--HELP", "--help", "-?", "-H", "-h", "-help", "/?", "/H", "/HELP", "/h", "/help", "HELP",
    "help",
];

const JOIN_SORTED: &[&str] = &[
    "--JOIN", "--join", "-J", "-JOIN", "-j", "-join", "/J", "/JOIN", "/j", "/join", "JOIN", "join",
];

const MOVE_SORTED: &[&str] = &[
    "--MOVE", "--move", "-M", "-MOVE", "-m", "-move", "/M", "/MOVE", "/m", "/move", "MOVE", "move",
];

#[cfg(feature = "cli")]
const SIMULATE_SORTED: &[&str] = &[
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

const SPLIT_SORTED: &[&str] = &[
    "--SPLIT", "--split", "-S", "-SPLIT", "-s", "-split", "/S", "/SPLIT", "/s", "/split", "SPLIT",
    "split",
];

#[cfg(feature = "cli")]
const VERSION_SORTED: &[&str] = &[
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

/// Display the error message (optional) and send the error code to operating system
fn error(err: processor::SyncError) {
    #[cfg(debug_assertions)]
    println!("{:?}", err);

    #[cfg(feature = "cli")]
    std::process::exit(processor::error_msg(&err.to_string(), err.code, true));

    #[cfg(not(feature = "cli"))]
    std::process::exit(err.code);
}

/// Display elapsed time (optional) and send a zero code (NO_ERROR) to operating system
fn no_error(_start: &std::time::Instant) {
    #[cfg(feature = "cli")]
    println!("\n{} {:#?}", processor::command_msgs(2), _start.elapsed());

    std::process::exit(processor::no_error());
}

fn execute_folder(
    command: &[&str],
    argument: &str,
    to_process: &str,
    _start: &std::time::Instant,
    process: fn(&str) -> Result<(), processor::SyncError>,
) {
    if command.binary_search(&argument).is_ok() {
        if let Err(err) = process(to_process) {
            return error(err);
        }
        no_error(_start);
    }
}

fn execute_file(
    command: &[&str],
    argument: &str,
    source: &str,
    destination: &str,
    _start: &std::time::Instant,
    process: fn(&str, &str) -> Result<(), processor::SyncError>,
) {
    if command.binary_search(&argument).is_ok() {
        if let Err(err) = process(source, destination) {
            return error(err);
        }
        no_error(_start);
    }
}

fn four_arguments(_start: &std::time::Instant) {
    let command = std::env::args().nth(1).unwrap();
    let source_folder = std::env::args().nth(2).unwrap();
    let dest_folder = std::env::args().nth(3).unwrap();

    #[cfg(feature = "cli")]
    processor::show_header(false);

    execute_file(
        CHECK_SORTED,
        command.as_str(),
        &source_folder,
        &dest_folder,
        _start,
        processor::check,
    );

    execute_file(
        FORCE_SORTED,
        command.as_str(),
        &source_folder,
        &dest_folder,
        _start,
        processor::force,
    );

    execute_file(
        HASH_SORTED,
        command.as_str(),
        &source_folder,
        &dest_folder,
        _start,
        processor::hash_folder,
    );

    execute_file(
        MOVE_SORTED,
        command.as_str(),
        &source_folder,
        &dest_folder,
        _start,
        processor::mv,
    );

    execute_file(
        SPLIT_SORTED,
        command.as_str(),
        &source_folder,
        &dest_folder,
        _start,
        processor::split,
    );

    #[cfg(feature = "cli")]
    {
        execute_file(
            SIMULATE_SORTED,
            command.as_str(),
            &source_folder,
            &dest_folder,
            _start,
            processor::simulate,
        );

        processor::show_header(true);
    }

    if let Err(err) = processor::create(&command, &source_folder, &dest_folder) {
        return error(err);
    }
    no_error(_start);
}

fn three_arguments(_start: &std::time::Instant) {
    let source = std::env::args().nth(1).unwrap();
    let destination = std::env::args().nth(2).unwrap();

    #[cfg(feature = "cli")]
    processor::show_header(true);

    execute_folder(
        CHECK_SORTED,
        source.as_str(),
        &destination,
        _start,
        processor::check_file,
    );

    #[cfg(feature = "cli")]
    {
        execute_folder(
            DUPLICATE_SORTED,
            source.as_str(),
            &destination,
            _start,
            processor::duplicate,
        );

        execute_folder(
            EMPTY_SORTED,
            source.as_str(),
            &destination,
            _start,
            processor::empty,
        );
    }

    execute_folder(
        FORCE_SORTED,
        source.as_str(),
        &destination,
        _start,
        processor::force_file,
    );

    #[cfg(feature = "cli")]
    execute_folder(
        HASH_SORTED,
        source.as_str(),
        &destination,
        _start,
        processor::hash_file,
    );

    execute_folder(
        JOIN_SORTED,
        source.as_str(),
        &destination,
        _start,
        processor::join_folder,
    );

    #[cfg(feature = "cli")]
    execute_folder(
        SIMULATE_SORTED,
        source.as_str(),
        &destination,
        _start,
        processor::simulate_file,
    );

    if let Err(err) = processor::sync(&source, &destination) {
        return error(err);
    }
    no_error(_start);
}

fn two_arguments(_start: &std::time::Instant) {
    let config = std::env::args().nth(1).unwrap();
    let current_path = std::env::current_dir().unwrap().display().to_string();

    #[cfg(feature = "cli")]
    {
        if HELP_SORTED.binary_search(&config.as_str()).is_ok() {
            std::process::exit(processor::help());
        }

        if VERSION_SORTED.binary_search(&config.as_str()).is_ok() {
            println!("{}", option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"));
            std::process::exit(processor::no_error());
        }

        processor::show_header(true);

        execute_folder(
            SIMULATE_SORTED,
            config.as_str(),
            &current_path,
            _start,
            processor::simulate_folder,
        );
    }

    execute_folder(
        CHECK_SORTED,
        config.as_str(),
        &current_path,
        _start,
        processor::check_folder,
    );

    execute_folder(
        FORCE_SORTED,
        config.as_str(),
        &current_path,
        _start,
        processor::force_folder,
    );

    execute_folder(
        JOIN_SORTED,
        config.as_str(),
        &current_path,
        _start,
        processor::join_folder,
    );

    if let Err(err) = processor::sync_file(&config) {
        return error(err);
    }
    no_error(_start);
}

/// User only enter "sync", could display help or process all configs
fn one_argument(_start: &std::time::Instant) {
    #[cfg(feature = "cli")]
    processor::show_header(true);

    let current_path = std::env::current_dir().unwrap().display().to_string();
    if let Err(err) = processor::sync_folder(&current_path) {
        #[cfg(feature = "cli")]
        if err.code == processor::help_code() {
            std::process::exit(processor::help());
        }
        return error(err);
    }
    no_error(_start);
}

/// Process user input from command line
fn main() {
    let _start = std::time::Instant::now();
    let args = std::env::args().len();

    if args <= FN_ARGS.len() {
        return FN_ARGS[args - 1](&_start);
    }

    #[cfg(feature = "cli")]
    std::process::exit(processor::help());

    #[cfg(not(feature = "cli"))]
    std::process::exit(processor::no_error());
}
