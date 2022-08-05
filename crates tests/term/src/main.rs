fn main() {
    let mut t = term::stdout().unwrap();

    t.fg(term::color::RED).unwrap();
    writeln!(t, "red").unwrap();

    t.fg(term::color::GREEN).unwrap();
    writeln!(t, "green").unwrap();

    t.fg(term::color::BLUE).unwrap();
    writeln!(t, "blue").unwrap();

    t.fg(term::color::YELLOW).unwrap();
    writeln!(t, "yellow").unwrap();

    t.reset().unwrap();
}
