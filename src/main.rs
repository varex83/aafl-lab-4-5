use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use regex::Regex;

fn main() -> io::Result<()> {
    let input_path = std::env::args().nth(1).expect("missing input file argument");
    let output_path = std::env::args().nth(2).expect("missing output file argument");

    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);

    let mut output_file = File::create(output_path)?;

    let regex = Regex::new(r"\s*([0-9]*\.?[0-9]{4})\s*[;:?]\s*([0-9]*\.?[0-9]{3})\s*[;:?]\s*([1-9][0-9]*)\s*[;:?]\s*([A-Za-z0-9а-яА-Я\-_']+)\s*[;:?]\s*([A-Za-z0-9]{12})\s*[;:?]\s*([0-9]*\.?[0-9]+)\s*").unwrap();

    for line in reader.lines() {
        let line = line?;
        if let Some(caps) = regex.captures(&line) {
            // Extracting fields from the captures
            let pos_num = &caps[3];
            let product_name = &caps[4];
            let invoice_num = &caps[5];
            let cost = &caps[6];

            writeln!(output_file, "{}; {}; {}; {}", pos_num, product_name, invoice_num, cost)?;
        }
    }

    Ok(())
}
