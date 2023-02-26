mod aliases;
mod processor;

/// Function array to run according to the number of arguments entered
const FN_ARGS: [fn(&std::time::Instant); 4] =
    [one_argument, two_arguments, three_arguments, four_arguments];

/// Displays the error message (optional) and sends the error code to operating system
fn error(err: processor::SyncError) {
    #[cfg(feature = "i18n")]
    {
        #[cfg(debug_assertions)]
        println!("{err:?}");

        std::process::exit(processor::error_msg(
            &err.to_string(),
            err.code as i32,
            true,
        ));
    }

    #[cfg(not(feature = "i18n"))]
    std::process::exit(err.code as i32);
}

/// Displays elapsed time (optional) and sends a zero code (NO_ERROR) to operating system
fn no_error(_start: &std::time::Instant) {
    #[cfg(feature = "i18n")]
    println!("\n{} {:#?}", processor::elapse_msg(), _start.elapsed());

    std::process::exit(processor::ErrorCode::NoError as i32);
}

/// Checks if the argument is a command and runs the input function with to_process argument
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

/// Checks if the argument is a command and runs the input function with source and destination arguments
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

/// Process user inputs from command line
fn main() {
    let _start = std::time::Instant::now();
    let args = std::env::args().len();

    #[cfg(all(windows, feature = "colored"))]
    {
        enable_ansi_support().unwrap();
    }

    if args <= FN_ARGS.len() {
        return FN_ARGS[args - 1](&_start);
    }

    #[cfg(feature = "i18n")]
    std::process::exit(processor::help() as i32);

    #[cfg(not(feature = "i18n"))]
    std::process::exit(processor::ErrorCode::NoError as i32);
}

/// User entered "sync" or clicked on binary (no argument): could display help or process all .config
/// files in parallel if there is anyone in the same folder
fn one_argument(_start: &std::time::Instant) {
    #[cfg(feature = "i18n")]
    processor::show_header(true);

    if let Err(err) =
        processor::sync_folder(&std::env::current_dir().unwrap().display().to_string())
    {
        #[cfg(feature = "i18n")]
        if let crate::processor::ErrorCode::Help = err.code {
            crate::processor::help();
            std::process::exit(crate::processor::ErrorCode::Help as i32);
        }
        return error(err);
    }
    no_error(_start);
}

/// User entered "sync" and one argument (command): could be HELP_SORTED,
/// VERSION_SORTED, SIMULATE_SORTED, CHECK_SORTED, FORCE_SORTED, JOIN_SORTED or a .config file
fn two_arguments(_start: &std::time::Instant) {
    let config = std::env::args().nth(1).unwrap();
    let current_path = std::env::current_dir().unwrap().display().to_string();

    #[cfg(feature = "i18n")]
    {
        if aliases::HELP_SORTED.binary_search(&config.as_str()).is_ok() {
            std::process::exit(processor::help() as i32);
        }

        if aliases::VERSION_SORTED
            .binary_search(&config.as_str())
            .is_ok()
        {
            println!("{}", option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"));
            std::process::exit(processor::ErrorCode::NoError as i32);
        }

        processor::show_header(true);

        execute_folder(
            aliases::SIMULATE_SORTED,
            config.as_str(),
            &current_path,
            _start,
            processor::simulate_folder,
        );
    }

    execute_folder(
        aliases::CHECK_SORTED,
        config.as_str(),
        &current_path,
        _start,
        processor::check_folder,
    );

    execute_folder(
        aliases::FORCE_SORTED,
        config.as_str(),
        &current_path,
        _start,
        processor::force_folder,
    );

    execute_folder(
        aliases::JOIN_SORTED,
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

/// User entered "sync" and two arguments (a command and a folder): could be DUPLICATE_SORTED,
/// EMPTY_SORTED, SIMULATE_SORTED, CHECK_SORTED, FORCE_SORTED, HASH_SORTED, JOIN_SORTED or "sync source destination",
/// where source and destination could be files or folders
fn three_arguments(_start: &std::time::Instant) {
    let source = std::env::args().nth(1).unwrap();
    let destination = std::env::args().nth(2).unwrap();

    #[cfg(feature = "i18n")]
    {
        processor::show_header(true);

        execute_folder(
            aliases::DUPLICATE_SORTED,
            source.as_str(),
            &destination,
            _start,
            processor::duplicate,
        );

        execute_folder(
            aliases::EMPTY_SORTED,
            source.as_str(),
            &destination,
            _start,
            processor::empty,
        );

        execute_folder(
            aliases::SIMULATE_SORTED,
            source.as_str(),
            &destination,
            _start,
            processor::simulate_file,
        );
    }

    execute_folder(
        aliases::CHECK_SORTED,
        source.as_str(),
        &destination,
        _start,
        processor::check_file,
    );

    execute_folder(
        aliases::FORCE_SORTED,
        source.as_str(),
        &destination,
        _start,
        processor::force_file,
    );

    execute_folder(
        aliases::HASH_SORTED,
        source.as_str(),
        &destination,
        _start,
        processor::hash_file,
    );

    execute_folder(
        aliases::JOIN_SORTED,
        source.as_str(),
        &destination,
        _start,
        processor::join_folder,
    );

    if let Err(err) = processor::sync(&source, &destination) {
        return error(err);
    }
    no_error(_start);
}

/// User entered "sync" and three arguments (command, source and destination): could be SIMULATE_SORTED,
/// CHECK_SORTED, FORCE_SORTED, HASH_SORTED, MOVE_SORTED, SPLIT_SORTED or user is creating a .config file
fn four_arguments(_start: &std::time::Instant) {
    let command = std::env::args().nth(1).unwrap();
    let source_folder = std::env::args().nth(2).unwrap();
    let dest_folder = std::env::args().nth(3).unwrap();

    #[cfg(feature = "i18n")]
    {
        processor::show_header(false);

        execute_file(
            aliases::SIMULATE_SORTED,
            command.as_str(),
            &source_folder,
            &dest_folder,
            _start,
            processor::simulate,
        );

        processor::show_header(true);
    }

    execute_file(
        aliases::CHECK_SORTED,
        command.as_str(),
        &source_folder,
        &dest_folder,
        _start,
        processor::check,
    );

    execute_file(
        aliases::FORCE_SORTED,
        command.as_str(),
        &source_folder,
        &dest_folder,
        _start,
        processor::force,
    );

    execute_file(
        aliases::HASH_SORTED,
        command.as_str(),
        &source_folder,
        &dest_folder,
        _start,
        processor::hash_folder,
    );

    execute_file(
        aliases::MOVE_SORTED,
        command.as_str(),
        &source_folder,
        &dest_folder,
        _start,
        processor::mv,
    );

    execute_file(
        aliases::SPLIT_SORTED,
        command.as_str(),
        &source_folder,
        &dest_folder,
        _start,
        processor::split,
    );

    if let Err(err) = processor::create(&command, &source_folder, &dest_folder) {
        return error(err);
    }
    no_error(_start);
}

/// Enable colored terminal output
#[cfg(all(windows, feature = "colored"))]
fn enable_ansi_support() -> Result<(), std::io::Error> {
    use std::{ffi::OsStr, iter::once, os::windows::ffi::OsStrExt};
    use windows_sys::Win32::{
        Foundation::INVALID_HANDLE_VALUE,
        Storage::FileSystem::{
            CreateFileW, FILE_GENERIC_READ, FILE_GENERIC_WRITE, FILE_SHARE_WRITE, OPEN_EXISTING,
        },
        System::Console::{GetConsoleMode, SetConsoleMode, ENABLE_VIRTUAL_TERMINAL_PROCESSING},
    };

    unsafe {
        let mut console_mode = 0;
        let console_out_name: Vec<u16> =
            OsStr::new("CONOUT$").encode_wide().chain(once(0)).collect();
        let console_handle = CreateFileW(
            console_out_name.as_ptr(),
            FILE_GENERIC_READ | FILE_GENERIC_WRITE,
            FILE_SHARE_WRITE,
            std::ptr::null(),
            OPEN_EXISTING,
            0,
            0,
        );

        if console_handle == INVALID_HANDLE_VALUE {
            return Err(std::io::Error::last_os_error());
        }

        if GetConsoleMode(console_handle, &mut console_mode) == 0 {
            return Err(std::io::Error::last_os_error());
        }

        if (console_mode & ENABLE_VIRTUAL_TERMINAL_PROCESSING == 0)
            && (SetConsoleMode(
                console_handle,
                console_mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING,
            ) == 0)
        {
            return Err(std::io::Error::last_os_error());
        }
    }

    Ok(())
}
