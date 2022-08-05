use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() {
	let mut stdout = StandardStream::stdout(ColorChoice::Always);
	
	stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
	writeln!(&mut stdout, "red").unwrap();
	
	stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
	writeln!(&mut stdout, "green").unwrap();

	stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue))).unwrap();
	writeln!(&mut stdout, "blue").unwrap();

	stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow))).unwrap();
	writeln!(&mut stdout, "yellow").unwrap();
	
	stdout.reset().unwrap();
	
}
