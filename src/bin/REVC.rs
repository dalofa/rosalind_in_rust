use std::io;

fn main() {
    // define the string and let the user supply it
    let mut input: String = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read input");

    // As all that is needed is replace I won't bother with a function
    let output = get_rev_comp(&input);
    

    // print
    println!("{output}")

}

fn get_rev_comp(dna:&String) -> String {
    
    // replace all characters and use capital vs non-capital to avoid double replacement
    let comp_dna: String = dna
    .replace("T","a")
    .replace("C","g")
    .replace("G","c")
    .replace("A","t")
    .to_uppercase();

    let rev_comp_dna: String = comp_dna.chars().rev().collect();

    return rev_comp_dna
}
