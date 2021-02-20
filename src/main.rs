use std::io;
use std::str;
use std::process;

fn transcription(dna: String) -> String {
    let char_vec: Vec<char> = dna.chars().collect();
    let mut transcribed_vec: Vec<char> = Vec::new();
    for i in char_vec {

        match i {
            'A' => transcribed_vec.push('U'),
            'T' => transcribed_vec.push('A'),
            'C' => transcribed_vec.push('G'),
            'G' => transcribed_vec.push('C'),
            _ => {
                // println!("Incorrect char");
                break;
             }
        }
    }

    let transcribed_string: String = transcribed_vec.into_iter().collect();
    return transcribed_string;
}

fn find_start(messenger_rna: String) -> String {
    let start_codon = "AUG";
    let start_index = messenger_rna.find(start_codon).unwrap();
    let inter_rna: String = messenger_rna.chars().skip(start_index).collect();
    return inter_rna;
}

fn break_into_codons(inter_rna: String) -> Vec<String> {
    let sub_len = 3;
    let subs = inter_rna.as_bytes()
        .chunks(sub_len)
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();

    let mut string_vec: Vec<String> = Vec::new();
    for i in &subs {
        string_vec.push(i.to_string());
    }

    return string_vec;
}

fn find_stop(inter_codons: &[String]) -> usize {
    if inter_codons.iter().any(|i| i == "UAA") {
        println!("UAA stop codon found!");
        let stop_index = inter_codons.iter().position(|r| r == "UAA").unwrap();
        return stop_index;
    }
    else if inter_codons.iter().any(|i| i == "UAG") {
        println!("UAG stop codon found!");
        let stop_index = inter_codons.iter().position(|r| r == "UAG").unwrap();
        return stop_index;
    }
    else if inter_codons.iter().any(|i| i == "UGA") {
        println!("UGA stop codon found!");
        let stop_index = inter_codons.iter().position(|r| r == "UGA").unwrap();
        return stop_index;
    }
    else {
        println!("No stop codon found!");
        process::exit(1);
    }
}

fn translation(inter_codons: Vec<String>) {
    for i in inter_codons {
        println!("{}", i);
    }
}

fn main() {
    println!("Enter the DNA strand to be transcribed and translated: ");

    let mut strand = String::new();

    io::stdin()
        .read_line(&mut strand)
        .expect("Failed to read line");

    strand = strand.to_uppercase();

    let messenger_rna = transcription(strand);
    println!("The transcribed strand is: {}", messenger_rna);
    let inter_rna = find_start(messenger_rna);
    println!("{}", inter_rna);
    let mut inter_codons = break_into_codons(inter_rna);
    let mut stop_index = find_stop(&inter_codons);
    stop_index = stop_index + 1;
    println!("{}", stop_index);
    inter_codons.truncate(stop_index);
    translation(inter_codons);
}
