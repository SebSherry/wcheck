extern crate clap;
extern crate lazy_static;
extern crate regex;

use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;

use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{Error, Write};
use std::path::{Path, PathBuf};
use std::process::exit;

mod word;
use word::Word;

const BRITISH_ENGLISH_DICTIONARY_FILE: &'static str = "/usr/share/dict/british-english";
const RESERVED_WORDS_FILE: &'static str = "/usr/share/dict/wcheck-reserved-words";
const BASELINE_FILE: &'static str = ".wcheck-baseline";

fn read_dictionary_file(dictionary: &mut Vec<String>, filename: &str) -> Result<(), Error> {
    let mut file_contents: Vec<String> = fs::read_to_string(filename)?
        .lines()
        .map(|w| w.to_lowercase())
        .collect();

    if dictionary.is_empty() {
        file_contents.clone_into(dictionary);
    } else {
        dictionary.append(&mut file_contents);
        dictionary.sort();
    }

    Ok(())
}

fn read_words_from_file(filename: &PathBuf) -> Result<Vec<Word>, Error> {
    lazy_static! {
        static ref WORD_MATCH_RE: Regex = Regex::new("[a-zA-Z][a-zA-Z_\']*[a-zA-Z]").unwrap();
    }
    let mut word_list: Vec<Word> = Vec::new();

    for (idx, line) in fs::read_to_string(filename)?.lines().enumerate() {
        let line_words: Vec<&str> = line
            .split_whitespace()
            .filter(|w| WORD_MATCH_RE.is_match(w))
            .collect();

        // Remove punctuation
        word_list.append(
            &mut line_words
                .into_iter()
                .map(|w| {
                    let word = WORD_MATCH_RE
                        .find(w)
                        .expect("Missing word after match")
                        .as_str()
                        .to_string();

                    Word {
                        word,
                        file: filename.to_path_buf(),
                        line_nr: (idx + 1) as u32,
                    }
                })
                .collect(),
        );
    }

    Ok(word_list)
}

/// Generates a baseline file for all found spelling mistakes
/// If a baseline file exists, append the new spelling mistakes
fn generate_baseline(misspelled_words: &Vec<Word>) -> Result<(), Error> {
    let mut baseline: HashMap<String, Vec<String>> = HashMap::new();
    let mut baseline_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(BASELINE_FILE)?;

    for word in misspelled_words {
        let filename = word.get_relative_file_path().to_string_lossy().to_string();

        if baseline.contains_key(&filename) {
            let mut words_for_file: Vec<String> = baseline
                .get(&filename)
                .expect("Get baseline words for a file")
                .to_owned();
            if !words_for_file.contains(&word.word) {
                words_for_file.push(word.word.clone());

                baseline.insert(filename, words_for_file.clone());

                writeln!(baseline_file, "{}", word.generate_baseline_entry())?;
            }
        } else {
            let words_for_file = vec![word.word.clone()];
            baseline.insert(filename, words_for_file);
            writeln!(baseline_file, "{}", word.generate_baseline_entry())?;
        }
    }

    Ok(())
}

fn read_baseline_file() -> Result<HashMap<String, Vec<String>>, Error> {
    let mut baseline: HashMap<String, Vec<String>> = HashMap::new();

    if Path::new(BASELINE_FILE).exists() {
        for line in fs::read_to_string(BASELINE_FILE)?.lines() {
            let line_split = line.split(": ").collect::<Vec<&str>>();
            let filename = line_split.get(0).unwrap().to_string();
            let word = line_split.get(1).unwrap().to_string();

            if baseline.contains_key(&filename) {
                let mut words_for_file: Vec<String> = baseline
                    .get(&filename)
                    .expect("Get baseline words for a file")
                    .to_owned();
                words_for_file.push(word);
                baseline.insert(filename, words_for_file.clone());
            } else {
                let words_for_file = vec![word];
                baseline.insert(filename, words_for_file);
            }
        }
    }

    Ok(baseline)
}

fn check_spelling_for_file_contents(
    dictionary: &Vec<String>,
    baseline: &HashMap<String, Vec<String>>,
    file_words: &Vec<Word>,
) -> Vec<Word> {
    let mut all_misspelled_words = Vec::new();

    for word in file_words {
        if let Err(misspelled_words) = word.is_correct_spelling(&dictionary) {
            // Check if the word is in the baseline for this file, if so skip
            let file_path = word.get_relative_file_path().to_string_lossy().to_string();
            if !baseline.contains_key(&file_path)
                || !baseline.get(&file_path).unwrap().contains(&word.word)
            {
                let is_multiple_words = word.is_camel_case() || word.is_snake_case();
                for m_word in misspelled_words {
                    if is_multiple_words {
                        println!(
                            "Misspelled word in {}:{}: '\x1b[91m{}\x1b[0m' within '\x1b[93m{}\x1b[0m'",
                            file_path, word.line_nr, m_word, word.word
                        );
                    } else {
                        println!(
                            "Misspelled word in {}:{}: '\x1b[91m{}\x1b[0m'",
                            file_path, word.line_nr, m_word
                        );
                    }
                }

                all_misspelled_words.push(word.clone());
            }
        }
    }

    all_misspelled_words
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Generates a baseline file of spelling mistakes to be ignored in future checks
    #[arg(long = "baseline")]
    generate_baseline: bool,

    /// Files to be spell checked
    #[arg(required = true)]
    files: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let mut dictionary: Vec<String> = Vec::new();
    if let Err(e) = read_dictionary_file(&mut dictionary, BRITISH_ENGLISH_DICTIONARY_FILE) {
        eprintln!("Failed to read british words dictionary: {}", e);
        eprintln!("Is the dictionary installed?");
        exit(-1);
    }

    if let Err(e) = read_dictionary_file(&mut dictionary, RESERVED_WORDS_FILE) {
        eprintln!("Failed to read reserved words dictionary: {}", e);
        eprintln!("Be sure to place 'wcheck-reserved-words' into /usr/share/dict");
        exit(-1);
    }

    let baseline: HashMap<String, Vec<String>>;
    match read_baseline_file() {
        Ok(b) => baseline = b,
        Err(e) => {
            eprintln!("Failed to read baseline file: {}", e);
            exit(-1);
        }
    }

    let mut all_misspelled_words: Vec<Word> = Vec::new();
    for file in args.files {
        match read_words_from_file(&file) {
            Ok(file_words) => {
                let mut misspelled_words =
                    check_spelling_for_file_contents(&dictionary, &baseline, &file_words);
                all_misspelled_words.append(&mut misspelled_words);
            }
            Err(e) => {
                eprintln!("Failed to read file {}: {}", file.display(), e);
                exit(-2);
            }
        }
    }

    if args.generate_baseline {
        println!("Generating baseline file");
        if let Err(e) = generate_baseline(&all_misspelled_words) {
            eprintln!("Failed to generate baseline file: {}", e);
            exit(-3);
        }
    }

    exit(all_misspelled_words.len() as i32);
}
