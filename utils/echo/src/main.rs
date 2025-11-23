use std::env::args;

fn main() {
    let mut output: Vec<String> = Vec::new();
    let mut newline: bool = true;
    for arg in args().skip(1) {
        if arg == "-n" {
            newline = false;
        } else {
            output.push(arg);
        }
    }

    print!("{}", output.join(" "));
    if newline {
        println!();
    }
}
