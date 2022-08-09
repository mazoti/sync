mod processor;

/// Display the error message (optional) and send the error code to operating system
macro_rules! error {
    ($err:expr) => {
        #[cfg(debug_assertions)]
        println!("{:?}", $err);

        #[cfg(feature = "cli")]
        if let Some(msg) = &$err.message {
            std::process::exit(processor::cli::error_msg(msg, $err.code));
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

/// Process user input from command line
fn main() {
    let _start = std::time::Instant::now();

    if std::env::args().len() == 1 {
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

    if std::env::args().len() == 2 {
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

            if config == "--version"
                || config == "-version"
                || config == "version"
                || config == "-v"
            {
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

    if std::env::args().len() == 3 {
        let source = std::env::args().nth(1).unwrap();
        let destination = std::env::args().nth(2).unwrap();

        #[cfg(feature = "cli")]
        processor::cli::show_header(true);
        if let Err(err) = processor::sync::sync(&source, &destination) {
            error!(err);
        }
        no_error!(_start);
    }

    if std::env::args().len() == 4 {
        let source_folder = std::env::args().nth(1).unwrap();
        let dest_folder = std::env::args().nth(2).unwrap();
        let config = std::env::args().nth(3).unwrap();

        if source_folder == "--check"
            || source_folder == "-check"
            || source_folder == "check"
            || source_folder == "-c"
        {
            #[cfg(feature = "cli")]
            processor::cli::show_header(false);

            if let Err(err) = processor::check::check(&dest_folder, &config) {
                error!(err);
            }
            no_error!(_start);
        }

        if source_folder == "--force"
            || source_folder == "-force"
            || source_folder == "force"
            || source_folder == "-f"
        {
            #[cfg(feature = "cli")]
            processor::cli::show_header(false);

            processor::force(&dest_folder, &config);
            no_error!(_start);
        }

        #[cfg(feature = "cli")]
        if source_folder == "--simulate"
            || source_folder == "-simulate"
            || source_folder == "simulate"
            || source_folder == "-s"
        {
            processor::cli::show_header(false);

            if let Err(err) = processor::sync::simulate(&dest_folder, &config) {
                error!(err);
            }
            no_error!(_start);
        }

        #[cfg(feature = "cli")]
        processor::cli::show_header(true);

        if let Err(err) = processor::create(&source_folder, &dest_folder, &config) {
            error!(err);
        }
        no_error!(_start);
    }

    #[cfg(feature = "cli")]
    std::process::exit(processor::cli::help());

    #[cfg(not(feature = "cli"))]
    std::process::exit(processor::consts::NO_ERROR);
}
