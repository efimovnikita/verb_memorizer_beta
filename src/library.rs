use regex::Regex;
use serde::Serialize;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn validate(past: String, verb: &&IrregularVerb, past_participle: String) -> (bool, String) {
    if past == verb.past && past_participle == verb.past_participle {
        (true, "".to_string())
    } else {
        (
            false,
            format!(
                "{} - {} - {}",
                verb.infinitive, verb.past, verb.past_participle
            ),
        )
    }
}

pub fn get_verbs(matches: &clap::ArgMatches) -> Result<(), String> {
    // Get the value of the "FILE" argument
    let file_path = matches.get_one::<String>("FILE").unwrap();
    let mut verbs: Vec<IrregularVerb> = Vec::new();
    match read_irregular_verbs(file_path) {
        Ok(vector) => verbs.extend(vector),
        Err(error) => {
            format!("Error while extracting list of verbs from file: {error}");
        }
    };
    for verb in verbs {
        println!("{}", verb.infinitive);
    }

    Ok(())
}

pub(crate) fn is_path_exists(path: &str) -> Result<String, String> {
    if Path::new(&path).exists() {
        Ok(path.to_string())
    } else {
        Err(format!("Invalid path: {path}"))
    }
}

pub(crate) fn replace_commas_with_whitespaces(s: &str) -> String {
    s.chars().map(|c| if c == ',' { ' ' } else { c }).collect()
}

pub(crate) fn remove_extra_whitespaces(s: &str) -> String {
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

pub(crate) fn is_two_forms_correct(input: &str) -> Result<String, String> {
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

#[derive(Serialize, Debug)]
pub(crate) struct ResultMsg {
    pub(crate) is_success: bool,
    pub(crate) msg: String,
}

impl ResultMsg {
    pub(crate) fn new(is_success: bool, msg: String) -> ResultMsg {
        ResultMsg { is_success, msg }
    }
}

pub struct IrregularVerb {
    pub infinitive: String,
    pub past: String,
    pub past_participle: String,
}

impl IrregularVerb {
    pub(crate) fn new(infinitive: String, past: String, past_participle: String) -> IrregularVerb {
        IrregularVerb {
            infinitive,
            past,
            past_participle,
        }
    }
}

pub(crate) fn read_irregular_verbs(filename: &str) -> Result<Vec<IrregularVerb>, String> {
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
