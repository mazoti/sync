fn main() {
    let mut a = [
	    "--JOIN", "--join", "-J", "-JOIN", "-j", "-join", "/J", "/JOIN", "/j", "/join", "JOIN",
    "join",
	];
    a.sort();

    println!("{:?}", a);
}