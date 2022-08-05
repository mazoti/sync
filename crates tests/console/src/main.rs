use console::Style;


fn main() {
let red = Style::new().red();
println!("{}", red.apply_to("red"));

let green = Style::new().green();
println!("{}", green.apply_to("green"));

let blue = Style::new().blue();
println!("{}", blue.apply_to("blue"));

let yellow = Style::new().yellow();
println!("{}", yellow.apply_to("yellow"));

}
