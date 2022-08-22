//! Command Line Interface, contains all output commands
use std::io::Write;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

/// Displays a colored "Copy" and the file path
#[inline]
pub fn copy_msg(file: &str) {
    message(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Green,
        crate::processor::consts::COMMAND_MSGS[1],
        file,
        true,
    );
}

/// Displays a colored "Create" and the folder path
#[inline]
pub fn create_msg(folder: &str) {
    message(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Green,
        crate::processor::consts::COMMAND_MSGS[0],
        folder,
        true,
    );
}

/// Displays a colored "ERROR", an error message in stderr and exit with the error code
pub fn error_msg(msg: &str, code: i32) -> i32 {
    message(
        &mut StandardStream::stderr(ColorChoice::Always),
        Color::Red,
        &("[".to_owned()
            + &chrono::Local::now().format("%Y-%m-%d %T").to_string()
            + "] "
            + crate::processor::consts::COMMAND_MSGS[3]),
        msg,
        false,
    );
    code
}

/// Displays a colored "Usage", the help message in stdout and exit with NO_ERROR code
pub fn help() -> i32 {
    message(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Yellow,
        crate::processor::consts::COMMAND_MSGS[10],
        crate::processor::consts::MSG_HELP,
        true,
    );
    crate::processor::consts::NO_ERROR
}

/// Displays a colored "Loading" and the file path
#[inline]
pub fn loading_msg(file: &str) {
    message(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Cyan,
        crate::processor::consts::COMMAND_MSGS[4],
        file,
        true,
    );
}

/// The kernel of the output messages
fn message(ss: &mut StandardStream, color: Color, colored_msg: &str, msg: &str, stdout: bool) {
    let mut stdout_locked = std::io::stdout().lock();
    let mut stderr_locked = std::io::stderr().lock();

    if stdout {
        ss.set_color(ColorSpec::new().set_fg(Some(color))).unwrap();
        stdout_locked
            .write_all(colored_msg.as_bytes())
            .expect(crate::processor::consts::ERROR_MSGS[12]);
        ss.reset().unwrap();

        #[cfg(windows)]
        let str = format!(" {}\n", msg.replace("\\\\?\\", ""));

        #[cfg(not(windows))]
        let str = format!(" {}\n", msg);

        stdout_locked
            .write_all(str.as_bytes())
            .expect(crate::processor::consts::ERROR_MSGS[12]);
        return;
    }

    ss.set_color(ColorSpec::new().set_fg(Some(color))).unwrap();
    stderr_locked
        .write_all(colored_msg.as_bytes())
        .expect(crate::processor::consts::ERROR_MSGS[12]);
    ss.reset().unwrap();

    #[cfg(windows)]
    let str = format!(" {}\n", msg.replace("\\\\?\\", ""));

    #[cfg(not(windows))]
    let str = format!(" {}\n", msg);

    stderr_locked
        .write_all(str.as_bytes())
        .expect(crate::processor::consts::ERROR_MSGS[12]);
}

/// Displays a colored "Ok" and the file path
#[inline]
pub fn ok_msg(file: &str) {
    message(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Blue,
        crate::processor::consts::COMMAND_MSGS[5],
        file,
        true,
    );
}

/// Displays a colored "Remove" and the file path
#[inline]
pub fn remove_msg(file: &str) {
    message(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Red,
        crate::processor::consts::COMMAND_MSGS[6],
        file,
        true,
    );
}

/// Displays the program name, version, URL and date/time (optional)
#[inline]
pub fn show_header(datetime: bool) {
    if datetime {
        print!("[{}] ", chrono::Local::now().format("%Y-%m-%d %T"));
    }

    println!(
        "sync version {} (https://github.com/mazoti/sync) {}\n",
        option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"),
        crate::processor::consts::COMMAND_MSGS[7]
    );
}

/// Displays a colored "Sync" and the file path
#[inline]
pub fn sync_msg(file: &str) {
    message(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Magenta,
        crate::processor::consts::COMMAND_MSGS[8],
        file,
        true,
    );
}

/// Displays a colored "Update" and the file path
#[inline]
pub fn update_msg(file: &str) {
    message(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Yellow,
        crate::processor::consts::COMMAND_MSGS[9],
        file,
        true,
    );
}

/// Displays a colored "Warning", the file path and a message in stdout
#[inline]
pub fn warning_msg(file: &str) {
    message(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Yellow,
        crate::processor::consts::ERROR_MSGS[11],
        &(file.to_owned() + " " + crate::processor::consts::ERROR_MSGS[13]),
        true,
    );
}

/// Displays a colored "(SIMULATION) Copying" and the file path
#[inline]
pub fn copy_msg_simulation(file: &str) {
    message_simulation(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Green,
        crate::processor::consts::COMMAND_MSGS[1],
        file,
    );
}

/// Displays a colored "(SIMULATION) Update" and the file path
#[inline]
pub fn update_msg_simulation(file: &str) {
    message_simulation(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Yellow,
        crate::processor::consts::COMMAND_MSGS[9],
        file,
    );
}

/// Displays a colored "(SIMULATION) Create" and the folder path
#[inline]
pub fn create_msg_simulation(folder: &str) {
    message_simulation(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Green,
        crate::processor::consts::COMMAND_MSGS[0],
        folder,
    );
}

/// Displays a colored "(SIMULATION) Remove" and the file path
#[inline]
pub fn remove_msg_simulation(file: &str) {
    message_simulation(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Red,
        crate::processor::consts::COMMAND_MSGS[6],
        file,
    );
}

/// Displays a colored "(SIMULATION) Sync" and the file path
#[inline]
pub fn sync_msg_simulation(file: &str) {
    message_simulation(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Magenta,
        crate::processor::consts::COMMAND_MSGS[8],
        file,
    );
}

/// The kernel of the output messages simulation
fn message_simulation(ss: &mut StandardStream, color: Color, colored_msg: &str, msg: &str) {
    let mut stdout_locked = std::io::stdout().lock();
    let mut _stderr_locked = std::io::stderr().lock();

    ss.set_color(ColorSpec::new().set_fg(Some(Color::Red)))
        .unwrap();
    stdout_locked
        .write_all(crate::processor::consts::COMMAND_MSGS[12].as_bytes())
        .expect(crate::processor::consts::ERROR_MSGS[12]);

    ss.set_color(ColorSpec::new().set_fg(Some(color))).unwrap();
    stdout_locked
        .write_all(colored_msg.as_bytes())
        .expect(crate::processor::consts::ERROR_MSGS[12]);

    ss.reset().unwrap();

    #[cfg(windows)]
    let str = format!(" {}\n", msg.replace("\\\\?\\", ""));

    #[cfg(not(windows))]
    let str = format!(" {}\n", msg);

    stdout_locked
        .write_all(str.as_bytes())
        .expect(crate::processor::consts::ERROR_MSGS[12]);
}

//====================================== Unit Tests ======================================

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "cli")]
    fn cli_tests() {
        crate::processor::cli::warning_msg("a/file/path/file.ext");
        crate::processor::cli::ok_msg("a/file/path/file.ext");
        crate::processor::cli::update_msg("a/file/path/file.ext");
        crate::processor::cli::create_msg("a/file/path/file.ext");
        crate::processor::cli::remove_msg("a/file/path/file.ext");
        crate::processor::cli::sync_msg("a/file/path/file.ext");
        crate::processor::cli::loading_msg("a/file/path/file.ext");
        crate::processor::cli::copy_msg("a/file/path/file.ext");
        crate::processor::cli::error_msg("Error message", 1234);
        crate::processor::cli::help();
        crate::processor::cli::show_header(true);
        crate::processor::cli::show_header(false);
    }
}
