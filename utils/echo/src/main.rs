use std::env::args;

fn main() {
    let mut output: Vec<String> = Vec::new();
    for arg in args().skip(1) {
        output.push(arg);
    }

    if !output.is_empty() {
        println!("{}", output.join(" "));
    }
}
