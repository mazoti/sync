//! User interface: contains all output commands

/// Displays "Usage", the help message in stdout and exit with HELP code
#[inline(always)]
pub fn help(
    command: &str,
    message: &str,
    return_code: crate::processor::ErrorCode,
) -> crate::processor::ErrorCode {
    #[cfg(feature = "colored")]
    print!("\x1B[33m{command}\x1b[0m{message}");

    #[cfg(not(feature = "colored"))]
    print!("{command}{message}");

    return_code
}

/// Displays the program name, version, URL and datetime (optional)
#[inline]
pub fn show_header(datetime: bool) {
    if datetime {
        print!("[{}] ", crate::processor::datetime());
    }

    println!(
        "sync version {} (https://github.com/mazoti/sync) {}\n",
        option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"),
        crate::processor::start_msg(),
    );
}

//====================================== Message methods in ascending order ======================================

/// Displays "Copying" and the file path
#[cfg(feature = "i18n")]
#[inline(always)]
pub fn copy_msg(command: &str, message: &str) {
    #[cfg(all(windows, feature = "colored"))]
    println!(
        "\x1B[92m{command:>14} \x1B[0m{}",
        message.replace("\\\\?\\", "")
    );

    #[cfg(all(not(windows), feature = "colored"))]
    println!("\x1B[92m{command:>14} \x1B[0m{message}");

    #[cfg(all(windows, not(feature = "colored")))]
    println!("{command:>14} {}", message.replace("\\\\?\\", ""));

    #[cfg(all(not(windows), not(feature = "colored")))]
    println!("{command:>14} {message}");
}

/// Displays "Creating" and the folder path
#[cfg(feature = "i18n")]
#[inline(always)]
pub fn create_msg(command: &str, message: &str) {
    #[cfg(all(windows, feature = "colored"))]
    println!(
        "\x1B[96m{command:>14} \x1B[0m{}",
        message.replace("\\\\?\\", "")
    );

    #[cfg(all(not(windows), feature = "colored"))]
    println!("\x1B[96m{command:>14} \x1B[0m{message}");

    #[cfg(all(windows, not(feature = "colored")))]
    println!("{command:>14} {}", message.replace("\\\\?\\", ""));

    #[cfg(all(not(windows), not(feature = "colored")))]
    println!("{command:>14} {message}");
}

/// Displays "DUPLICATED" with all duplicated file paths
#[inline]
pub fn duplicate_msgs(command: &str, files: Vec<&str>) {
    for message in files {
        #[cfg(feature = "colored")]
        println!("\x1B[91m{command:>14} \x1B[0m{message}");

        #[cfg(not(feature = "colored"))]
        println!("{command:>14} {message}");
    }
    println!();
}

/// Displays "Empty", the file or folder path and a message in stdout
#[cfg(feature = "i18n")]
#[inline(always)]
pub fn empty_msg(command: &str, message: &str) {
    #[cfg(feature = "colored")]
    eprintln!("\x1B[93m{command:>14} \x1b[0m{message}");

    #[cfg(not(feature = "colored"))]
    eprintln!("{command:>14} {message}");
}

/// Displays "ERROR", an error message in stderr and exits with the error code.
/// If user_input is "true", waits an "enter" from the user keyboard
#[inline]
pub fn error_msg(command: &str, message: &str, code: i32, user_input: bool) -> i32 {
    use std::io::Read;

    #[cfg(feature = "colored")]
    eprintln!("\x1B[91m{command:>14} \x1B[0m{message}");

    #[cfg(not(feature = "colored"))]
    eprintln!("{command:>14} {message}");

    // Waits user press "Enter"
    if user_input {
        let mut buffer = [0; 1];
        std::io::stdin().read_exact(&mut buffer).unwrap();
    }
    code
}

/// Displays "Loading" and the file path
#[inline(always)]
pub fn loading_msg(command: &str, message: &str) {
    #[cfg(all(windows, feature = "colored"))]
    println!(
        "\x1B[96m{command:>14} \x1B[0m{}",
        message.replace("\\\\?\\", "")
    );

    #[cfg(all(not(windows), feature = "colored"))]
    println!("\x1B[96m{command:>14} \x1B[0m{message}");

    #[cfg(all(windows, not(feature = "colored")))]
    println!("{command:>14} {}", message.replace("\\\\?\\", ""));

    #[cfg(all(not(windows), not(feature = "colored")))]
    println!("{command:>14} {message}");
}

/// Displays "Ok" and the file or folder path
#[cfg(feature = "i18n")]
#[inline(always)]
pub fn ok_msg(command: &str, message: &str) {
    #[cfg(all(windows, feature = "colored"))]
    println!(
        "\x1B[94m{command:>14} \x1B[0m{}",
        message.replace("\\\\?\\", "")
    );

    #[cfg(all(not(windows), feature = "colored"))]
    println!("\x1B[94m{command:>14} \x1B[0m{message}");

    #[cfg(all(windows, not(feature = "colored")))]
    println!("{command:>14} {}", message.replace("\\\\?\\", ""));

    #[cfg(all(not(windows), not(feature = "colored")))]
    println!("{command:>14} {message}")
}

/// Displays "(ONE ITEM)" and the folder path
#[cfg(feature = "i18n")]
#[inline(always)]
pub fn one_item_msg(command: &str, message: &str) {
    #[cfg(feature = "colored")]
    eprintln!("\x1B[93m{command:>14} \x1b[0m{message}");

    #[cfg(not(feature = "colored"))]
    eprintln!("{command:>14} {message}");
}

/// Displays "Removing" and the file or folder path
#[cfg(feature = "i18n")]
#[inline(always)]
pub fn remove_msg(command: &str, message: &str) {
    #[cfg(all(windows, feature = "colored"))]
    println!(
        "\x1B[91m{command:>14} \x1B[0m{}",
        message.replace("\\\\?\\", "")
    );

    #[cfg(all(not(windows), feature = "colored"))]
    println!("\x1B[91m{command:>14} \x1B[0m{message}");

    #[cfg(all(windows, not(feature = "colored")))]
    println!("{command:>14} {}", message.replace("\\\\?\\", ""));

    #[cfg(all(not(windows), not(feature = "colored")))]
    println!("{command:>14} {message}");
}

/// Displays "Sync" and the file path
#[inline(always)]
pub fn sync_msg(command: &str, message: &str) {
    #[cfg(all(windows, feature = "colored"))]
    println!(
        "\x1B[95m{command:>14} \x1B[0m{}",
        message.replace("\\\\?\\", "")
    );

    #[cfg(all(not(windows), feature = "colored"))]
    println!("\x1B[95m{command:>14} \x1B[0m{message}");

    #[cfg(all(windows, not(feature = "colored")))]
    println!("{command:>14} {}", message.replace("\\\\?\\", ""));

    #[cfg(all(not(windows), not(feature = "colored")))]
    println!("{command:>14} {message}");
}

/// Displays "Updating" and the file or folder path
#[cfg(feature = "i18n")]
#[inline(always)]
pub fn update_msg(command: &str, message: &str) {
    #[cfg(all(windows, feature = "colored"))]
    println!(
        "\x1B[93m{command:>14} \x1B[0m{}",
        message.replace("\\\\?\\", "")
    );

    #[cfg(all(not(windows), feature = "colored"))]
    println!("\x1B[93m{command:>14} \x1B[0m{message}");

    #[cfg(all(windows, not(feature = "colored")))]
    println!("{command:>14} {}", message.replace("\\\\?\\", ""));

    #[cfg(all(not(windows), not(feature = "colored")))]
    println!("{command:>14} {message}");
}

//====================================== Simulation message methods in ascending order ======================================

/// Displays "(SIMULATION) Copying" and the file path
#[cfg(feature = "i18n")]
#[inline(always)]
pub fn copy_msg_simulation(simulate: &str, command: &str, message: &str) {
    #[cfg(all(windows, feature = "colored"))]
    println!(
        "\x1B[91m{simulate:>14} \x1B[92m{command} \x1B[0m{}",
        message.replace("\\\\?\\", "")
    );

    #[cfg(all(not(windows), feature = "colored"))]
    println!("\x1B[91m{simulate:>14} \x1B[92m{command} \x1B[0m{message}");

    #[cfg(all(windows, not(feature = "colored")))]
    println!(
        "{simulate:>14} {command} {}",
        message.replace("\\\\?\\", "")
    );

    #[cfg(all(not(windows), not(feature = "colored")))]
    println!("{simulate:>14} {command} {message}");
}

/// Displays "(SIMULATION) Creating" and the folder path
#[cfg(feature = "i18n")]
#[inline(always)]
pub fn create_msg_simulation(simulate: &str, command: &str, message: &str) {
    #[cfg(all(windows, feature = "colored"))]
    println!(
        "\x1B[91m{simulate:>14} \x1B[96m{command} \x1B[0m{}",
        message.replace("\\\\?\\", "")
    );

    #[cfg(all(not(windows), feature = "colored"))]
    println!("\x1B[91m{simulate:>14} \x1B[96m{command} \x1B[0m{message}");

    #[cfg(all(windows, not(feature = "colored")))]
    println!(
        "{simulate:>14} {command} {}",
        message.replace("\\\\?\\", "")
    );

    #[cfg(all(not(windows), not(feature = "colored")))]
    println!("{simulate:>14} {command} {message}");
}

/// Displays "(SIMULATION) Removing" and the file or folder path
#[cfg(feature = "i18n")]
#[inline(always)]
pub fn remove_msg_simulation(simulate: &str, command: &str, message: &str) {
    #[cfg(all(windows, feature = "colored"))]
    println!(
        "\x1B[91m{simulate:>14} \x1B[91m{command} \x1B[0m{}",
        message.replace("\\\\?\\", "")
    );

    #[cfg(all(not(windows), feature = "colored"))]
    println!("\x1B[91m{simulate:>14} \x1B[91m{command} \x1B[0m{message}");

    #[cfg(all(windows, not(feature = "colored")))]
    println!(
        "{simulate:>14} {command} {}",
        message.replace("\\\\?\\", "")
    );

    #[cfg(all(not(windows), not(feature = "colored")))]
    println!("{simulate:>14} {command} {message}");
}

/// Displays "(SIMULATION) Sync" and the file or folder path
#[cfg(feature = "i18n")]
#[inline(always)]
pub fn sync_msg_simulation(simulate: &str, command: &str, message: &str) {
    #[cfg(all(windows, feature = "colored"))]
    println!(
        "\x1B[91m{simulate:>14} \x1B[95m{command} \x1B[0m{}",
        message.replace("\\\\?\\", "")
    );

    #[cfg(all(not(windows), feature = "colored"))]
    println!("\x1B[91m{simulate:>14} \x1B[95m{command} \x1B[0m{message}");

    #[cfg(all(windows, not(feature = "colored")))]
    println!(
        "{simulate:>14} {command} {}",
        message.replace("\\\\?\\", "")
    );

    #[cfg(all(not(windows), not(feature = "colored")))]
    println!("{simulate:>14} {command} {message}");
}

/// Displays "(SIMULATION) Updating" and the file or folder path
#[cfg(feature = "i18n")]
#[inline(always)]
pub fn update_msg_simulation(simulate: &str, command: &str, message: &str) {
    #[cfg(all(windows, feature = "colored"))]
    println!(
        "\x1B[91m{simulate:>14} \x1B[93m{command} \x1B[0m{}",
        message.replace("\\\\?\\", "")
    );

    #[cfg(all(not(windows), feature = "colored"))]
    println!("\x1B[91m{simulate:>14} \x1B[93m{command} \x1B[0m{message}");

    #[cfg(all(windows, not(feature = "colored")))]
    println!(
        "{simulate:>14} {command} {}",
        message.replace("\\\\?\\", "")
    );

    #[cfg(all(not(windows), not(feature = "colored")))]
    println!("{simulate:>14} {command} {message}");
}
