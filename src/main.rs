use std::io;

fn transcription(dna: String) -> Vec<char> {
    let char_vec: Vec<char> = dna.chars().collect();
    let mut transcribed_vec: Vec<char> = Vec::new();
    for i in char_vec {

        match i {
            'A' => transcribed_vec.push('U'),
            'T' => transcribed_vec.push('A'),
            'C' => transcribed_vec.push('G'),
            'G' => transcribed_vec.push('C'),
            _ => println!("{}", i) //just print wrong for now
        }
    }

    return transcribed_vec;
}

fn main() {
    println!("Enter the DNA strand to be transcribed and translated: ");

    let mut strand = String::new();

    io::stdin()
        .read_line(&mut strand)
        .expect("Failed to read line");

    strand = strand.to_uppercase();

    let messenger_rna = transcription(strand);
}
