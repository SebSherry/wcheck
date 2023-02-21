extern crate clap;
extern crate lazy_static;
extern crate regex;

use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;

use std::fs::{self, OpenOptions};
use std::io::{Error, Write};
use std::path::PathBuf;
use std::process::exit;

mod word;
use word::Word;

const BRITISH_ENGLISH_DICTIONARY_FILE: &'static str = "/usr/share/dict/british-english";
const RESERVED_WORDS_FILE: &'static str = "/usr/share/dict/wcheck-reserved-words";

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
    let mut baseline_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(".wcheck-baseline")?;

    for word in misspelled_words {
        writeln!(baseline_file, "{}", word.generate_baseline_entry())?
    }

    Ok(())
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

    let mut all_misspelled_words: Vec<Word> = Vec::new();
    for file in args.files {
        match read_words_from_file(&file) {
            Ok(file_words) => {
                for word in file_words {
                    match word.is_correct_spelling(&dictionary) {
                        Ok(_) => {}
                        Err(misspelled_words) => {
                            let is_multiple_words = word.is_camel_case() || word.is_snake_case();
                            for m_word in misspelled_words {
                                if is_multiple_words {
                                    println!(
                                        "Misspelled word in {}:{}: '\x1b[91m{}\x1b[0m' within '\x1b[93m{}\x1b[0m'",
                                        file.display(), word.line_nr, m_word, word.word
                                    );
                                } else {
                                    println!(
                                        "Misspelled word in {}:{}: '\x1b[91m{}\x1b[0m'",
                                        file.display(),
                                        word.line_nr,
                                        m_word
                                    );
                                }
                                all_misspelled_words.push(word.clone());
                            }
                        }
                    }
                }
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
        }
    }

    exit(all_misspelled_words.len() as i32);
}
