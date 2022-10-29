mod processor;

const FN_ARGS: [fn(&std::time::Instant); 4] =
    [one_argument, two_arguments, three_arguments, four_arguments];

const CHECK_SORTED: &[&str] = &[
    "--CHECK", "--check", "-C", "-CHECK", "-c", "-check", "/C", "/CHECK", "/c", "/check", "CHECK",
    "check",
];

const FORCE_SORTED: &[&str] = &[
    "--FORCE", "--force", "-F", "-FORCE", "-f", "-force", "/F", "/FORCE", "/f", "/force", "FORCE",
    "force",
];

#[cfg(feature = "cli")]
const HELP_SORTED: &[&str] = &[
    "--HELP", "--help", "-?", "-H", "-h", "-help", "/?", "/H", "/HELP", "/h", "/help", "HELP",
    "help",
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
fn error(err: processor::error::SyncError) {
    #[cfg(debug_assertions)]
    println!("{:?}", err);

    #[cfg(feature = "cli")]
    if let Some(msg) = err.message {
        std::process::exit(processor::cli::error_msg(&msg, err.code, true));
    }

    #[cfg(not(feature = "cli"))]
    std::process::exit(err.code);
}

/// Display elapsed time (optional) and send a zero code (NO_ERROR) to operating system
fn no_error(_start: &std::time::Instant) {
    #[cfg(feature = "cli")]
    println!(
        "\n{} {:#?}",
        crate::processor::consts::COMMAND_MSGS[2],
        _start.elapsed()
    );

    std::process::exit(processor::consts::NO_ERROR);
}

fn execute_folder(
    command: &[&str],
    argument: &str,
    to_process: &str,
    _start: &std::time::Instant,
    process: fn(&str) -> Result<(), crate::processor::error::SyncError>,
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
    process: fn(&str, &str) -> Result<(), crate::processor::error::SyncError>,
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
    processor::cli::show_header(false);

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

        processor::cli::show_header(true);
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
    processor::cli::show_header(true);

    execute_folder(
        CHECK_SORTED,
        source.as_str(),
        &destination,
        _start,
        processor::check_file,
    );
    execute_folder(
        FORCE_SORTED,
        source.as_str(),
        &destination,
        _start,
        processor::force_file,
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
            std::process::exit(processor::cli::help());
        }

        if VERSION_SORTED.binary_search(&config.as_str()).is_ok() {
            println!("{}", option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"));
            std::process::exit(processor::consts::NO_ERROR);
        }

        processor::cli::show_header(true);

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

    if let Err(err) = processor::sync_file(&config) {
        return error(err);
    }
    no_error(_start);
}

fn one_argument(_start: &std::time::Instant) {
    #[cfg(feature = "cli")]
    processor::cli::show_header(true);

    let current_path = std::env::current_dir().unwrap().display().to_string();
    if let Err(err) = processor::sync_folder(&current_path) {
        #[cfg(feature = "cli")]
        if err.code == processor::consts::HELP {
            std::process::exit(processor::cli::help());
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
    std::process::exit(processor::cli::help());

    #[cfg(not(feature = "cli"))]
    std::process::exit(processor::consts::NO_ERROR);
}
