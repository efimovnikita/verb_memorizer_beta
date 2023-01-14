use crate::library::{get_verbs, is_two_forms_correct};
use clap::{Arg, Command};
use cursive::theme::{Color, PaletteColor, Theme};
use cursive::view::{Nameable, Resizable};
use cursive::views::{Dialog, EditView, LinearLayout, TextView};
use cursive::Cursive;
use library::{IrregularVerb, VerbQueue};
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

        let mut siv = cursive::default();

        let theme = custom_theme_from_cursive(&siv);
        siv.set_theme(theme);

        let user_data = library::VerbQueue::new(verbs);
        siv.set_user_data(user_data.clone());

        siv.add_layer(
            Dialog::around(
                LinearLayout::vertical()
                    .child(TextView::new(format!(
                        "Verb: {}",
                        user_data.queue.front().unwrap().infinitive
                    )))
                    .child(
                        EditView::new()
                            .on_submit(validate_and_show_next)
                            .with_name("edit"),
                    )
                    .fixed_width(40),
            )
            .title("Write second and third form")
            .button("Quit", |s| s.quit()),
        );

        siv.run();
    }

    if let Some(matches) = matches.subcommand_matches("verbs") {
        match get_verbs(matches) {
            Ok(_) => {}
            Err(error) => {
                eprintln!("{error}");
                std::process::exit(1)
            }
        };
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

        let validation_result = library::validate(past, filtered_verb, past_participle);

        let message: library::ResultMsg =
            library::ResultMsg::new(validation_result.0, validation_result.1);
        println!("{}", serde_json::to_string(&message).unwrap());
    }
}

fn validate_and_show_next(s: &mut Cursive, text: &str) {
    let mut verb = s
        .user_data::<VerbQueue>()
        .and_then(|data| data.queue.front().cloned())
        .unwrap_or_else(IrregularVerb::default);

    if let Err(error) = is_two_forms_correct(text) {
        s.add_layer(Dialog::info(format!(
            "{}.\nThe correct answer is: {} - {} - {}",
            error, verb.infinitive, verb.past, verb.past_participle,
        )));
        s.with_user_data(|data: &mut VerbQueue| data.errors += 1);
        return;
    }

    if let Ok(validated_input) = is_two_forms_correct(text) {
        let mut parts = validated_input.split_whitespace();
        let past = parts.next().unwrap_or("").to_lowercase();
        let past_participle = parts.next().unwrap_or("").to_lowercase();

        let validation_result = library::validate(past, &&verb, past_participle);

        if !validation_result.0 {
            s.add_layer(Dialog::info(format!(
                "Incorrect. The correct answer is: {}",
                validation_result.1
            )));
            s.with_user_data(|data: &mut VerbQueue| data.errors += 1);
            return;
        }

        // pop new verb
        s.with_user_data(|data: &mut VerbQueue| data.queue.pop_front());

        let mut empty_list: bool = false;
        s.with_user_data(|data: &mut VerbQueue| {
            if data.queue.is_empty() {
                empty_list = true;
            }
        });

        if empty_list {
            let total = s
                .user_data::<VerbQueue>()
                .map(|data| data.verbs_count)
                .unwrap_or(0);

            let errors = s
                .user_data::<VerbQueue>()
                .map(|data| data.errors)
                .unwrap_or(0);

            s.pop_layer();
            s.pop_layer();
            s.add_layer(
                Dialog::around(TextView::new(format!(
                    "Done!\nTotal number of verbs: {total}\nNumbers of errors: {errors}"
                )))
                .title("Finish")
                .button("Ok", |s| s.quit())
                .fixed_width(45),
            );
            return;
        }

        s.with_user_data(|data: &mut VerbQueue| verb = data.queue.front().unwrap().clone())
            .unwrap();

        s.pop_layer();
        s.add_layer(
            Dialog::around(
                LinearLayout::vertical()
                    .child(TextView::new(format!("Verb: {}", verb.infinitive)))
                    .child(
                        EditView::new()
                            .on_submit(validate_and_show_next)
                            .with_name("edit"),
                    )
                    .fixed_width(40),
            )
            .title("Write second and third form")
            .button("Quit", |s| s.quit()),
        );
    }
}

fn custom_theme_from_cursive(siv: &Cursive) -> Theme {
    // We'll return the current theme with a small modification.
    let mut theme = siv.current_theme().clone();

    theme.palette[PaletteColor::Background] = Color::TerminalDefault;

    theme
}
