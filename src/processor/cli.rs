//! Command Line Interface, contains all output commands
use std::io::{Read, Write};

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

/// Displays a colored "Copy" and the file path
#[inline(always)]
pub fn copy_msg(file: &str) {
    message(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Green,
        crate::processor::command_msgs(1),
        file,
        true,
    );
}

/// Displays a colored "Create" and the folder path
#[inline(always)]
pub fn create_msg(folder: &str) {
    message(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Green,
        crate::processor::command_msgs(0),
        folder,
        true,
    );
}

#[inline(always)]
pub fn duplicate_msg(file: &str) {
    message(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Yellow,
        crate::processor::command_msgs(15),
        file,
        true,
    );
}

/// Displays a colored "Empty" and the file or folder path
#[inline(always)]
pub fn empty_msg(file_folder: &str) {
    message(
        &mut StandardStream::stderr(ColorChoice::Always),
        Color::Red,
        crate::processor::command_msgs(13),
        file_folder,
        false,
    );
}

/// Displays a colored "ERROR", an error message in stderr and exit with the error code.
/// If user_input is "true", waits an "enter" from user keyboard
pub fn error_msg(msg: &str, code: i32, user_input: bool) -> i32 {
    message(
        &mut StandardStream::stderr(ColorChoice::Always),
        Color::Red,
        &("[".to_owned()
            + &crate::processor::datetime()
            + "] "
            + crate::processor::command_msgs(3)),
        msg,
        false,
    );

    // Waits user press "Enter"
    if user_input {
        let mut buffer = [0; 1];
        std::io::stdin().read_exact(&mut buffer).unwrap();
    }
    code
}

/// Displays a colored "Usage", the help message in stdout and exit with NO_ERROR code
#[inline(always)]
pub fn help() -> i32 {
    message(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Yellow,
        crate::processor::command_msgs(10),
        crate::processor::msg_help(),
        true,
    );
    crate::processor::no_error()
}

/// Displays a colored "Loading" and the file path
#[inline(always)]
pub fn loading_msg(file: &str) {
    message(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Cyan,
        crate::processor::command_msgs(4),
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
            .expect(crate::processor::error_msgs()[12]);
        ss.reset().unwrap();

        #[cfg(windows)]
        let str = format!(" {}\n", msg.replace("\\\\?\\", ""));

        #[cfg(not(windows))]
        let str = format!(" {}\n", msg);

        stdout_locked
            .write_all(str.as_bytes())
            .expect(crate::processor::error_msgs()[12]);
        return;
    }

    ss.set_color(ColorSpec::new().set_fg(Some(color))).unwrap();
    stderr_locked
        .write_all(colored_msg.as_bytes())
        .expect(crate::processor::error_msgs()[12]);
    ss.reset().unwrap();

    #[cfg(windows)]
    let str = format!(" {}\n", msg.replace("\\\\?\\", ""));

    #[cfg(not(windows))]
    let str = format!(" {}\n", msg);

    stderr_locked
        .write_all(str.as_bytes())
        .expect(crate::processor::error_msgs()[12]);
}

/// Displays a colored "Ok" and the file path
#[inline(always)]
pub fn ok_msg(file: &str) {
    message(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Blue,
        crate::processor::command_msgs(5),
        file,
        true,
    );
}

/// Displays a colored "Empty" and the file or folder path
#[inline(always)]
pub fn one_item(folder: &str) {
    message(
        &mut StandardStream::stderr(ColorChoice::Always),
        Color::Red,
        crate::processor::command_msgs(14),
        folder,
        false,
    );
}

/// Displays a colored "Remove" and the file path
#[inline(always)]
pub fn remove_msg(file: &str) {
    message(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Red,
        crate::processor::command_msgs(6),
        file,
        true,
    );
}

/// Displays the program name, version, URL and date/time (optional)
#[inline(always)]
pub fn show_header(datetime: bool) {
    if datetime {
        print!("[{}] ", crate::processor::datetime());
    }

    println!(
        "sync version {} (https://github.com/mazoti/sync) {}\n",
        option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"),
        crate::processor::command_msgs(7),
    );
}

/// Displays a colored "Sync" and the file path
#[inline(always)]
pub fn sync_msg(file: &str) {
    message(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Magenta,
        crate::processor::command_msgs(8),
        file,
        true,
    );
}

/// Displays a colored "Update" and the file path
#[inline(always)]
pub fn update_msg(file: &str) {
    message(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Yellow,
        crate::processor::command_msgs(9),
        file,
        true,
    );
}

/// Displays a colored "Warning", the file path and a message in stdout
#[inline(always)]
pub fn warning_msg(file: &str) {
    message(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Yellow,
        crate::processor::error_msgs()[11],
        &(file.to_owned() + " " + crate::processor::error_msgs()[13]),
        true,
    );
}

/// Displays a colored "(SIMULATION) Copying" and the file path
#[inline(always)]
pub fn copy_msg_simulation(file: &str) {
    message_simulation(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Green,
        crate::processor::command_msgs(1),
        file,
    );
}

/// Displays a colored "(SIMULATION) Update" and the file path
#[inline(always)]
pub fn update_msg_simulation(file: &str) {
    message_simulation(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Yellow,
        crate::processor::command_msgs(9),
        file,
    );
}

/// Displays a colored "(SIMULATION) Create" and the folder path
#[inline(always)]
pub fn create_msg_simulation(folder: &str) {
    message_simulation(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Green,
        crate::processor::command_msgs(0),
        folder,
    );
}

/// Displays a colored "(SIMULATION) Remove" and the file path
#[inline(always)]
pub fn remove_msg_simulation(file: &str) {
    message_simulation(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Red,
        crate::processor::command_msgs(6),
        file,
    );
}

/// Displays a colored "(SIMULATION) Sync" and the file path
#[inline(always)]
pub fn sync_msg_simulation(file: &str) {
    message_simulation(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Magenta,
        crate::processor::command_msgs(8),
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
        .write_all(crate::processor::command_msgs(12).as_bytes())
        .expect(crate::processor::error_msgs()[12]);

    stdout_locked
        .write_all(" ".as_bytes())
        .expect(crate::processor::error_msgs()[12]);

    ss.set_color(ColorSpec::new().set_fg(Some(color))).unwrap();
    stdout_locked
        .write_all(colored_msg.as_bytes())
        .expect(crate::processor::error_msgs()[12]);

    ss.reset().unwrap();

    #[cfg(windows)]
    let str = format!(" {}\n", msg.replace("\\\\?\\", ""));

    #[cfg(not(windows))]
    let str = format!(" {}\n", msg);

    stdout_locked
        .write_all(str.as_bytes())
        .expect(crate::processor::error_msgs()[12]);
}

//====================================== Unit Tests ======================================

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "cli")]
    fn cli_tests() {
        crate::processor::warning_msg("a/file/path/file.ext");
        crate::processor::ok_msg("a/file/path/file.ext");
        crate::processor::update_msg("a/file/path/file.ext");
        crate::processor::create_msg("a/file/path/file.ext");
        crate::processor::remove_msg("a/file/path/file.ext");
        crate::processor::sync_msg("a/file/path/file.ext");
        crate::processor::loading_msg("a/file/path/file.ext");
        crate::processor::copy_msg("a/file/path/file.ext");
        crate::processor::error_msg("Error message", 1234, false);
        crate::processor::help();
        crate::processor::show_header(true);
        crate::processor::show_header(false);

        crate::processor::copy_msg_simulation("a/file/path/file.ext");
        crate::processor::update_msg_simulation("a/file/path/file.ext");
        crate::processor::create_msg_simulation("a/file/path/file.ext");
        crate::processor::remove_msg_simulation("a/file/path/file.ext");
        crate::processor::sync_msg_simulation("a/file/path/file.ext");
    }
}
