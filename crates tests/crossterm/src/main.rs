use std::io::stdout;

use crossterm::{style::{Color, Print, ResetColor, SetForegroundColor}, ExecutableCommand};

fn main() {
    stdout()
        .execute(SetForegroundColor(Color::Red)).unwrap()
        .execute(Print("red\n")).unwrap();

    stdout()
        .execute(SetForegroundColor(Color::Green)).unwrap()
        .execute(Print("green\n")).unwrap();

    stdout()
        .execute(SetForegroundColor(Color::Blue)).unwrap()
        .execute(Print("blue\n")).unwrap();

    stdout()
        .execute(SetForegroundColor(Color::Yellow)).unwrap()
        .execute(Print("yellow\n")).unwrap()
        .execute(ResetColor).unwrap();
}
