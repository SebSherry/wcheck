extern crate lazy_static;
extern crate regex;

use lazy_static::lazy_static;
use regex::Regex;

use std::env;
use std::fs;
use std::io::Error;
use std::process::exit;

mod word;

use word::Word;

fn read_dictionary() -> Result<Vec<String>, Error> {
    Ok(fs::read_to_string("/usr/share/dict/british-english")?
        .lines()
        .map(ToOwned::to_owned)
        .collect())
}

fn read_words_from_file(filename: &String) -> Result<Vec<Word>, Error> {
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
                        .expect("Missing word from match")
                        .as_str()
                        .to_string();

                    Word {
                        word,
                        line_nr: (idx + 1) as u32,
                    }
                })
                .collect(),
        );
    }

    Ok(word_list)
}

fn main() {
    let dictionary;
    match read_dictionary() {
        Ok(d) => dictionary = d,
        Err(e) => {
            eprintln!("Failed to read dictionary: {}", e);
            exit(-1);
        }
    };

    let mut misspelled_word_count: i32 = 0;
    for file in env::args().skip(1) {
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
                                        file, word.line_nr, m_word, word.word
                                    );
                                } else {
                                    println!(
                                        "Misspelled word in {}:{}: '\x1b[91m{}\x1b[0m'",
                                        file, word.line_nr, m_word
                                    );
                                }
                                misspelled_word_count += 1;
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read file {}: {}", file, e);
                exit(-2);
            }
        }
    }

    exit(misspelled_word_count);
}
