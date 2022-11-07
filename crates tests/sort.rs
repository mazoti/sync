fn main() {
    let mut a = ["split", "SPLIT", "/split", "/s", "/SPLIT", "/S", "-split", "-s", "-SPLIT", "-S", "--split", "--SPLIT"];
    a.sort();

    println!("{:?}", a);
}