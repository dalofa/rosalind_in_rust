use std::io;

fn main() {
    // define the string and let the user supply it
    let mut input: String = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read input");

    // As all that is needed is replace I won't bother with a function
    let output: String = input.replace("T", "U");

    // print
    println!("{output}")

}