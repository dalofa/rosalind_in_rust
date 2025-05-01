use std::io;

fn main() {
    // define the string and let the user supply it
    let mut input: String = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read input");

    //let input= String::from("AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC");
    
    // count the bases and reassign them to the variables
    // This allows me to print them without the () to match the rosalind format
    let base_counts: (usize, usize, usize, usize) = count_bases(&input);
    let (a,c,g,t) = base_counts;

    // print
    println!("{a} {c} {g} {t}");

}

fn count_bases(dna:&String) -> (usize, usize, usize, usize) {
    let a: usize = dna.matches("A").count();
    let c: usize = dna.matches("C").count();
    let g: usize = dna.matches("G").count();
    let t: usize = dna.matches("T").count();
    
    return (a,c,g,t)
}