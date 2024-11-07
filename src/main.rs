use clap::Parser; // argparse
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;

// Define your inputs
#[derive(Parser, Debug)]
#[command(version = "1.0", about = "Given a set of sequencing reads in fastq, display the k-mer spectra")]

struct Args {
    // path to the fastq File
    #[arg(short, long)]
    fastq: String,
    #[arg(short, long)]
    kmer: u8,
}


fn main() -> io::Result<()> {
    let args = Args::parse();
    let fastq_filepath = &args.fastq;
    let k = args.kmer;

    // println!("Path to your fastq file is {fastq_filepath} and k-mer size is {k}"); - //works!
    let file = File::open(fastq_filepath)?;
    let reader = BufReader::new(file);

    let mut i = 0; // keep track of line number
    let mut d = HashMap::<String, i32>::new();

    for line in reader.lines() {
        let line = line?;
        // println!("{line}"); // works, lessgoo
        if i % 4 == 1 {
            let curr_line = &line.trim();
            // println!("{curr_line}"); // works!
            let curr_length = curr_line.len();
            let num_kmers = curr_length as u8 - k + 1; // works, phewww
            for i in 0..num_kmers {
                // rest of your code goes here
                let skipped_chars = curr_line.chars().skip(i.into());
                let selected_chars = skipped_chars.take(k.into());
                let kmer: String = selected_chars.collect();
                // println!("{kmer}"); // phewww works
                if d.contains_key(&kmer) == false {
                    d.insert(kmer, 1);
                }
                else {
                    let curr_freq = d.get(&kmer).unwrap();
                    d.insert(kmer, curr_freq+1);
                }

            }
        }
        i += 1;
    }

//    for (key,value) in &d {
//        println!("{key}\t{value}"); - works!
//    }

    let mut d2 = HashMap::<i32,i32>::new();

    for (_, value) in &d {
        if d2.contains_key(value) == false {
            d2.insert(*value, 1);
        }
        else {
            let curr_freq = d2.get(value).unwrap();
            d2.insert(*value, curr_freq+1);
        }
    }

// print the final spectra

    println!("KmerFreq\tCount");

    for (key, value) in &d2 {
        println!("{key}\t{value}");
    }


    Ok(())
}
