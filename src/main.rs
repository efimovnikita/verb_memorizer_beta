use clap::{Arg, Command};
use colored::*;
use rand::{seq::SliceRandom, thread_rng};
use regex::Regex;
use serde::Serialize;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

#[cfg(test)]
mod tests;

fn is_path_exists(path: &str) -> Result<String, String> {
    if Path::new(&path).exists() {
        Ok(path.to_string())
    } else {
        Err(format!("Invalid path: {path}"))
    }
}

fn replace_commas_with_whitespaces(s: &str) -> String {
    s.chars().map(|c| if c == ',' { ' ' } else { c }).collect()
}

fn remove_extra_whitespaces(s: &str) -> String {
    let mut result = String::new();
    let mut previous_char_was_whitespace = false;

    for c in s.chars() {
        if c.is_whitespace() {
            if !previous_char_was_whitespace {
                result.push(c);
            }
            previous_char_was_whitespace = true;
        } else {
            result.push(c);
            previous_char_was_whitespace = false;
        }
    }

    result
}

fn is_two_forms_correct(input: &str) -> Result<String, String> {
    // Remove commas
    let input_without_commas = replace_commas_with_whitespaces(input);

    // Create a regular expression to match the pattern "at least one character, then at least one whitespace, then at least one character"
    let regex = Regex::new(r"^\w+\s+\w+$").unwrap();

    // Use the `is_match()` method to check if the input string `s` matches the regular expression
    if regex.is_match(&input_without_commas) {
        // If the input string `s` matches the pattern, return it as a `String` wrapped in `Ok`
        Ok(remove_extra_whitespaces(&input_without_commas))
    } else {
        // If the input string `s` does not match the pattern, return an error message wrapped in `Err`
        Err(format!(
            "The string '{input}' does not satisfy the pattern 'word word'"
        ))
    }
}

fn main() {
    // Parse the command line arguments
    let file_arg = Arg::new("FILE")
        .help("The file with verbs to memorize")
        .default_value("irregular_verbs.txt")
        .value_parser(is_path_exists);
    let matches = Command::new("memorize-app")
        .version("1.0")
        .author("Maskedball <maskedballmail@gmail.com>")
        .about("App for memorizing irregular verbs forms.")
        .subcommand_required(true)
        .subcommand(Command::new("memo").arg(&file_arg))
        .subcommand(Command::new("verbs").arg(&file_arg))
        .subcommand(
            Command::new("check")
                .arg(&file_arg)
                .arg(
                    Arg::new("FORMS")
                        .long("forms")
                        .short('f')
                        .help("Two forms of verb for check")
                        .required(true)
                        .value_parser(is_two_forms_correct),
                )
                .arg(
                    Arg::new("VERB")
                        .long("verb")
                        .short('v')
                        .help("Verb for check")
                        .required(true),
                ),
        )
        .get_matches();

    // Check if the "verbs" subcommand was used
    if let Some(matches) = matches.subcommand_matches("memo") {
        // Get the value of the "FILE" argument
        let file_path = matches.get_one::<String>("FILE").unwrap();

        let mut verbs: Vec<IrregularVerb> = Vec::new();

        match read_irregular_verbs(file_path) {
            Ok(vector) => verbs.extend(vector),
            Err(error) => {
                eprintln!("Error while extracting list of verbs from file: {error}");
                std::process::exit(1)
            }
        }

        // Shuffle vector
        let mut rng = thread_rng();
        verbs.shuffle(&mut rng);

        let mut correct_answers = 0;
        let mut total_answers = 0;

        for verb in &verbs {
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
            let past = parts.next().unwrap_or("").to_lowercase();
            let past_participle = parts.next().unwrap_or("").to_lowercase();

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

        println!("Correct answers: {correct_answers}/{total_answers}");
    }

    if let Some(matches) = matches.subcommand_matches("verbs") {
        // Get the value of the "FILE" argument
        let file_path = matches.get_one::<String>("FILE").unwrap();

        let mut verbs: Vec<IrregularVerb> = Vec::new();

        match read_irregular_verbs(file_path) {
            Ok(vector) => verbs.extend(vector),
            Err(error) => {
                eprintln!("Error while extracting list of verbs from file: {error}");
                std::process::exit(1)
            }
        }

        for verb in verbs {
            println!("{}", verb.infinitive);
        }
    }

    if let Some(matches) = matches.subcommand_matches("check") {
        // Get the value of the "FILE" argument
        let file_path = matches.get_one::<String>("FILE").unwrap();
        let verb = matches.get_one::<String>("VERB").unwrap();
        let forms = matches.get_one::<String>("FORMS").unwrap().trim();
        let mut parts = forms.split_whitespace();
        let past = parts.next().unwrap_or("").to_lowercase();
        let past_participle = parts.next().unwrap_or("").to_lowercase();

        let mut verbs: Vec<IrregularVerb> = Vec::new();

        match read_irregular_verbs(file_path) {
            Ok(vector) => verbs.extend(vector),
            Err(error) => {
                eprintln!("Error while extracting list of verbs from file: {error}");
                std::process::exit(1)
            }
        }

        let filtered_verbs: Vec<&IrregularVerb> = verbs
            .iter()
            .filter(|v| v.infinitive == verb.trim())
            .collect();
        if filtered_verbs.is_empty() {
            eprintln!("Can't find verb '{verb}' in file '{file_path}'");
            std::process::exit(1)
        }

        let filtered_verb = filtered_verbs.first().unwrap();

        if past == filtered_verb.past && past_participle == filtered_verb.past_participle {
            let msg: ResultMsg = ResultMsg::new(true, "".to_string());
            println!("{}", serde_json::to_string(&msg).unwrap());
        } else {
            let msg: ResultMsg = ResultMsg::new(
                false,
                format!(
                    "{} - {} - {}",
                    filtered_verb.infinitive, filtered_verb.past, filtered_verb.past_participle
                ),
            );
            println!("{}", serde_json::to_string(&msg).unwrap());
        }
    }
}

#[derive(Serialize, Debug)]
struct ResultMsg {
    is_success: bool,
    msg: String,
}

impl ResultMsg {
    fn new(is_success: bool, msg: String) -> ResultMsg {
        ResultMsg { is_success, msg }
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
        Err(error) => return Err(format!("Error opening file: {error}")),
    };
    let reader = io::BufReader::new(file);

    let mut verbs = Vec::new();

    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => return Err(format!("Error reading line: {error}")),
        };
        let mut parts = line.split(',');

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
