use std::fs;    // needed to read files

fn main() {
    // parse fasta file into a meaninful format, perhaps a hash map
    let results = fasta_to_gc(&String::from("rosalind_gc.txt"));
    // 

    // find the beggest of the sequences
    let mut max_gc: f64 = 0.0;
    let mut max_header: String = String::new();

    for (header, gc) in results {
        if gc>max_gc {
            max_gc = gc;
            max_header = header;
        }

    }

    // format correctly for Rosalind answers
    max_header.remove(0);

    println!("{max_header}\n{max_gc}")
}

fn get_gc_from_string(seq: &String) -> f64 {
    let gc_count = seq.chars()
    .filter(|c| *c == 'C'|| *c == 'G')
    .count();

    let total_len = seq.len();

    let gc_perc = gc_count as f64 / total_len as f64 * 100.0;

    return gc_perc
    
}


fn fasta_to_gc(file_path:&String) -> Vec<(String, f64)> {
    // read in the fasta file line by line
    let text_cont = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");


    // iterate over the lines
    let mut seq: String = String::new();
    let mut header: String = String::new();
    let mut results = Vec::new(); 
    //let mut gc_cont: i32 
    for line in text_cont.lines() {

        // collect header  
        if line.starts_with(">") {
            if header.trim().is_empty() {
                header = line.trim().to_string();
            } else {
                // Set the header
                results.push((header,get_gc_from_string(&seq)));
                header = line.trim().to_string(); // Update the header variable (rather than using let, which creates a new scoped variable)
                seq.clear(); // clear seq from the appended input from before 
            }
        } else {
            seq.push_str(line.trim());
            
        }
        
    }
    results.push((header,get_gc_from_string(&seq))); // Get the last sequence as well
    

    return results

    }



