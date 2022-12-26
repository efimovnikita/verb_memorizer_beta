use clap::{Arg, Command};
use colored::*;
use rand::{seq::SliceRandom, thread_rng};

mod library;

#[cfg(test)]
mod tests;

fn main() {
    // Parse the command line arguments
    let file_arg = Arg::new("FILE")
        .help("The file with verbs to memorize")
        .default_value("irregular_verbs.txt")
        .value_parser(library::is_path_exists);
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
                        .value_parser(library::is_two_forms_correct),
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

        let mut verbs: Vec<library::IrregularVerb> = Vec::new();

        match library::read_irregular_verbs(file_path) {
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
            std::io::stdin()
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

        let mut verbs: Vec<library::IrregularVerb> = Vec::new();

        match library::read_irregular_verbs(file_path) {
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

        let mut verbs: Vec<library::IrregularVerb> = Vec::new();

        match library::read_irregular_verbs(file_path) {
            Ok(vector) => verbs.extend(vector),
            Err(error) => {
                eprintln!("Error while extracting list of verbs from file: {error}");
                std::process::exit(1)
            }
        }

        let filtered_verbs: Vec<&library::IrregularVerb> = verbs
            .iter()
            .filter(|v| v.infinitive == verb.trim())
            .collect();
        if filtered_verbs.is_empty() {
            eprintln!("Can't find verb '{verb}' in file '{file_path}'");
            std::process::exit(1)
        }

        let filtered_verb = filtered_verbs.first().unwrap();

        if past == filtered_verb.past && past_participle == filtered_verb.past_participle {
            let msg: library::ResultMsg = library::ResultMsg::new(true, "".to_string());
            println!("{}", serde_json::to_string(&msg).unwrap());
        } else {
            let msg: library::ResultMsg = library::ResultMsg::new(
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
