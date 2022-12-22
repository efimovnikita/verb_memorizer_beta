use clap::{Command, Arg};
use colored::*;
use rand::{seq::SliceRandom, thread_rng};
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

fn is_path_exists(p: &str) -> Result<String, String> {
    if Path::new(&p).exists() {
        Ok(p.to_string())
    } else {
        Err(format!("Invalid path: {}", p))
    }
}

fn main() {
    // Parse the command line arguments
    let matches = Command::new("memorize-app")
        .version("1.0")
        .author("Maskedball <maskedballmail@gmail.com>")
        .about("App for memorizing irregular verbs forms.")
        .subcommand(
            Command::new("memorize")
                .arg(
                    Arg::new("FILE")
                        .help("The file to memorize")
                        .required(true)
                        .value_parser(is_path_exists)
                )
        )
        .get_matches();

    // Check if the "memorize" subcommand was used
    if let Some(matches) = matches.subcommand_matches("memorize") {
        // Get the value of the "FILE" argument
        let file_path = matches.get_one::<String>("FILE").unwrap();

        let verbs = read_irregular_verbs(file_path);
        if verbs.is_err() {
            eprintln!("Error while extracting list of verbs from file");
            return;
        }

        let mut unwrapped_verbs = verbs.unwrap();

        // Shuffle vector
        let mut rng = thread_rng();
        unwrapped_verbs.shuffle(&mut rng);

        let mut correct_answers = 0;
        let mut total_answers = 0;

        for verb in &unwrapped_verbs {
            println!(
                "{}",
                format!("Infinitive form: {}", verb.infinitive)
                    .underline()
                    .yellow()
                    .bold()
            );

            println!("Past and past participle forms (separated by a space):");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Error reading input");
            let input = input.trim();

            let mut parts = input.split_whitespace();
            let past = parts.next().unwrap();
            let past_participle = parts.next().unwrap();

            if past == verb.past && past_participle == verb.past_participle {
                correct_answers += 1;
                println!("{}", "Correct!".green().bold());
            } else {
                println!(
                    "{}",
                    format!(
                        "Incorrect. The correct answer is: {} - {} - {}",
                        verb.infinitive, verb.past, verb.past_participle
                    )
                    .red()
                    .bold()
                );
            }
            total_answers += 1;

            println!()
        }

        println!("Correct answers: {}/{}", correct_answers, total_answers);
    }
}

struct IrregularVerb {
    infinitive: String,
    past: String,
    past_participle: String,
}

impl IrregularVerb {
    fn new(infinitive: String, past: String, past_participle: String) -> IrregularVerb {
        IrregularVerb {
            infinitive,
            past,
            past_participle,
        }
    }
}

fn read_irregular_verbs(filename: &str) -> Result<Vec<IrregularVerb>, String> {
    let file = match fs::File::open(filename) {
        Ok(file) => file,
        Err(error) => return Err(format!("Error opening file: {}", error)),
    };
    let reader = io::BufReader::new(file);

    let mut verbs = Vec::new();

    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => return Err(format!("Error reading line: {}", error)),
        };
        let mut parts = line.split(",");

        let infinitive = match parts.next() {
            Some(infinitive) => infinitive.trim().to_owned(),
            None => return Err("Error parsing infinitive form".to_owned()),
        };
        let past = match parts.next() {
            Some(past) => past.trim().to_owned(),
            None => return Err("Error parsing past form".to_owned()),
        };
        let past_participle = match parts.next() {
            Some(past_participle) => past_participle.trim().to_owned(),
            None => return Err("Error parsing past participle form".to_owned()),
        };

        verbs.push(IrregularVerb::new(infinitive, past, past_participle));
    }

    Ok(verbs)
}
