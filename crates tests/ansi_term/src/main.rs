use ansi_term::Colour::Red;
use ansi_term::Colour::Green;
use ansi_term::Colour::Blue;
use ansi_term::Colour::Yellow;

fn main() {
	println!("{}", Red.paint("red"));
	println!("{}", Green.paint("green"));
	println!("{}", Blue.paint("blue"));
	println!("{}", Yellow.paint("yellow"));
}
