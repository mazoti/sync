//! Command Line Interface, contains all output commands

use std::io::{Read, Write};

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

// ============================================= Public methods in ascending order ==============

/// Displays a colored "Copy" and the file path
#[inline(always)]
pub fn copy_msg(filepath: &str) {
    message(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Green,
        crate::processor::command_msgs(1),
        filepath,
        true,
    );
}

/// Displays a colored "(SIMULATION) Copying" and the file path
#[inline(always)]
pub fn copy_msg_simulation(filepath: &str) {
    message_simulation(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Green,
        crate::processor::command_msgs(1),
        filepath,
    );
}

/// Displays a colored "Create" and the folder path
#[inline(always)]
pub fn create_msg(folderpath: &str) {
    message(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Green,
        crate::processor::command_msgs(0),
        folderpath,
        true,
    );
}

/// Displays a colored "(SIMULATION) Create" and the folder path
#[inline(always)]
pub fn create_msg_simulation(folderpath: &str) {
    message_simulation(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Green,
        crate::processor::command_msgs(0),
        folderpath,
    );
}

/// Displays a colored "Duplicate" and the file path
#[inline(always)]
pub fn duplicate_msg(filepath: &str) {
    message(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Yellow,
        crate::processor::command_msgs(15),
        filepath,
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
#[inline]
pub fn help() -> i32 {
    message(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Yellow,
        crate::processor::command_msgs(10),
        crate::processor::msg_help(),
        true,
    );
    crate::processor::help_code()
}

/// Displays a colored "Loading" and the file path
#[inline(always)]
pub fn loading_msg(filepath: &str) {
    message(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Cyan,
        crate::processor::command_msgs(4),
        filepath,
        true,
    );
}

/// Displays a colored "Ok" and the file path
#[inline(always)]
pub fn ok_msg(filepath: &str) {
    message(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Blue,
        crate::processor::command_msgs(5),
        filepath,
        true,
    );
}

/// Displays a colored "One item" and the folder path
#[inline(always)]
pub fn one_item(folderpath: &str) {
    message(
        &mut StandardStream::stderr(ColorChoice::Always),
        Color::Red,
        crate::processor::command_msgs(14),
        folderpath,
        false,
    );
}

/// Displays a colored "Remove" and the file path
#[inline(always)]
pub fn remove_msg(filepath: &str) {
    message(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Red,
        crate::processor::command_msgs(6),
        filepath,
        true,
    );
}

/// Displays a colored "(SIMULATION) Remove" and the file path
#[inline(always)]
pub fn remove_msg_simulation(filepath: &str) {
    message_simulation(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Red,
        crate::processor::command_msgs(6),
        filepath,
    );
}

/// Displays the program name, version, URL and date/time (optional)
#[inline]
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
pub fn sync_msg(filepath: &str) {
    message(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Magenta,
        crate::processor::command_msgs(8),
        filepath,
        true,
    );
}

/// Displays a colored "(SIMULATION) Sync" and the file path
#[inline(always)]
pub fn sync_msg_simulation(filepath: &str) {
    message_simulation(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Magenta,
        crate::processor::command_msgs(8),
        filepath,
    );
}

/// Displays a colored "Update" and the file path
#[inline(always)]
pub fn update_msg(filepath: &str) {
    message(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Yellow,
        crate::processor::command_msgs(9),
        filepath,
        true,
    );
}

/// Displays a colored "(SIMULATION) Update" and the file path
#[inline(always)]
pub fn update_msg_simulation(filepath: &str) {
    message_simulation(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Yellow,
        crate::processor::command_msgs(9),
        filepath,
    );
}

/// Displays a colored "Warning", the file path and a message in stdout
#[inline(always)]
pub fn warning_msg(filepath: &str) {
    message(
        &mut StandardStream::stdout(ColorChoice::Always),
        Color::Yellow,
        crate::processor::error_msgs()[11],
        &(filepath.to_owned() + " " + crate::processor::error_msgs()[13]),
        true,
    );
}

// ============================================= Private methods in ascending order, connects to termcolor crate ==============

/// The kernel of the output messages
fn message(ss: &mut StandardStream, color: Color, colored_msg: &str, msg: &str, stdout: bool) {
    let str: String;

    let mut stdout_locked = std::io::stdout().lock();
    let mut stderr_locked = std::io::stderr().lock();

    if stdout {
        ss.set_color(ColorSpec::new().set_fg(Some(color))).unwrap();
        stdout_locked
            .write_all(colored_msg.as_bytes())
            .expect(crate::processor::error_msgs()[12]);
        ss.reset().unwrap();

        #[cfg(windows)]
        {
            str = format!(" {}\n", msg.replace("\\\\?\\", ""));
        }

        #[cfg(not(windows))]
        {
            str = format!(" {}\n", msg);
        }

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
    {
        str = format!(" {}\n", msg.replace("\\\\?\\", ""));
    }

    #[cfg(not(windows))]
    {
        str = format!(" {}\n", msg);
    }
    stderr_locked
        .write_all(str.as_bytes())
        .expect(crate::processor::error_msgs()[12]);
}

/// Adds a red "(SIMULATION) " before the command message
fn message_simulation(ss: &mut StandardStream, color: Color, colored_msg: &str, msg: &str) {
    {
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

        ss.reset().unwrap();

        // Unlocks stdout and stderr
    }

    message(ss, color, colored_msg, msg, true)
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
