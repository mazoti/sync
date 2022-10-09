mod processor;

const FN_ARGS: [fn(); 4] = [one_argument, two_arguments, three_arguments, four_arguments];

/// Display the error message (optional) and send the error code to operating system
macro_rules! error {
    ($err:expr) => {
        #[cfg(debug_assertions)]
        println!("{:?}", $err);

        #[cfg(feature = "cli")]
        if let Some(msg) = &$err.message {
            std::process::exit(processor::cli::error_msg(msg, $err.code, true));
        }

        #[cfg(not(feature = "cli"))]
        std::process::exit($err.code);
    };
}

/// Display elapsed time (optional) and send a zero code (NO_ERROR) to operating system
macro_rules! no_error {
    ($start:expr) => {
        #[cfg(feature = "cli")]
        println!(
            "\n{} {:#?}",
            crate::processor::consts::COMMAND_MSGS[2],
            $start.elapsed()
        );
        std::process::exit(processor::consts::NO_ERROR);
    };
}

fn four_arguments() {
    #[cfg(feature = "cli")]
    let _start = std::time::Instant::now();

    let command = std::env::args().nth(1).unwrap();
    let source_folder = std::env::args().nth(2).unwrap();
    let dest_folder = std::env::args().nth(3).unwrap();

    if command == "--check" || command == "-check" || command == "check" || command == "-c" {
        #[cfg(feature = "cli")]
        processor::cli::show_header(false);

        if let Err(err) = processor::check::check(&source_folder, &dest_folder) {
            error!(err);
        }
        no_error!(_start);
    }

    if command == "--force" || command == "-force" || command == "force" || command == "-f" {
        #[cfg(feature = "cli")]
        processor::cli::show_header(false);

        processor::force(&source_folder, &dest_folder);
        no_error!(_start);
    }

    #[cfg(feature = "cli")]
    if command == "--simulate" || command == "-simulate" || command == "simulate" || command == "-s"
    {
        processor::cli::show_header(false);

        if let Err(err) = processor::sync::simulate(&source_folder, &dest_folder) {
            error!(err);
        }
        no_error!(_start);
    }

    #[cfg(feature = "cli")]
    processor::cli::show_header(true);

    if let Err(err) = processor::create(&command, &source_folder, &dest_folder) {
        error!(err);
    }
    no_error!(_start);
}

fn three_arguments() {
    #[cfg(feature = "cli")]
    let _start = std::time::Instant::now();

    let source = std::env::args().nth(1).unwrap();
    let destination = std::env::args().nth(2).unwrap();

    #[cfg(feature = "cli")]
    processor::cli::show_header(true);
    if let Err(err) = processor::sync::sync(&source, &destination) {
        error!(err);
    }
    no_error!(_start);
}

fn two_arguments() {
    #[cfg(feature = "cli")]
    let _start = std::time::Instant::now();

    let config = std::env::args().nth(1).unwrap();

    #[cfg(feature = "cli")]
    {
        if config == "--help"
            || config == "-h"
            || config == "/?"
            || config == "/help"
            || config == "-help"
            || config == "help"
        {
            std::process::exit(processor::cli::help());
        }

        if config == "--version" || config == "-version" || config == "version" || config == "-v" {
            println!("{}", option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"));
            std::process::exit(processor::consts::NO_ERROR);
        }

        processor::cli::show_header(true);
    }

    if let Err(err) = processor::sync::file(&config) {
        error!(err);
    }
    no_error!(_start);
}

fn one_argument() {
    #[cfg(feature = "cli")]
    let _start = std::time::Instant::now();

    #[cfg(feature = "cli")]
    processor::cli::show_header(true);

    let current_path = std::env::current_dir().unwrap().display().to_string();
    if let Err(err) = processor::sync::folder(&current_path) {
        #[cfg(feature = "cli")]
        if err.code == processor::consts::HELP {
            std::process::exit(processor::cli::help());
        }
        error!(err);
    }
    no_error!(_start);
}

/// Process user input from command line
fn main() {
    let args = std::env::args().len();

    if args < 6 {
        return FN_ARGS[args - 1]();
    }

    #[cfg(feature = "cli")]
    std::process::exit(processor::cli::help());

    #[cfg(not(feature = "cli"))]
    std::process::exit(processor::consts::NO_ERROR);
}
