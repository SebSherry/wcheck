use std::env::current_dir;
use std::path::PathBuf;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Clone)]
pub struct Word {
    pub word: String,
    pub file: PathBuf,
    pub line_nr: u32,
}

impl Word {
    pub fn is_correct_spelling(&self, dictionary: &Vec<String>) -> Result<(), Vec<String>> {
        // If this word is camelCase or snake_case variable handle those cases
        if self.is_camel_case() {
            return self.check_camel_case(dictionary);
        }

        if self.is_snake_case() {
            return self.check_snake_case(dictionary);
        }

        if self.is_dictionary_word(dictionary, &self.word) {
            Ok(())
        } else {
            Err(vec![self.word.clone()])
        }
    }

    pub fn is_camel_case(&self) -> bool {
        lazy_static! {
            static ref CAMEL_CASE: Regex = Regex::new("[a-zA-Z][a-z]+([A-Z]+[a-z]+)+").unwrap();
        }

        CAMEL_CASE.is_match(&self.word)
    }

    pub fn is_snake_case(&self) -> bool {
        lazy_static! {
            static ref SNAKE_CASE: Regex = Regex::new("[a-zA-Z]+_([a-zA-Z_])+").unwrap();
        }

        SNAKE_CASE.is_match(&self.word)
    }

    pub fn generate_baseline_entry(&self) -> String {
        // TODO: Handle this error
        let cwd = current_dir().expect("Get the current directory");
        let relative_file_path = match self.file.is_relative() {
            true => self.file.as_path(),
            false => self.file.strip_prefix(cwd).unwrap(),
        };

        format!("{}: {}", relative_file_path.display(), self.word)
    }

    fn is_dictionary_word(&self, dictionary: &Vec<String>, word: &String) -> bool {
        dictionary.binary_search(&word.to_lowercase()).is_ok()
    }

    fn check_camel_case(&self, dictionary: &Vec<String>) -> Result<(), Vec<String>> {
        lazy_static! {
            static ref CAMEL_CASE_SHORT_MATCH: Regex = Regex::new("[a-zA-Z][^A-Z]+").unwrap();
        }

        let mut idx = 0;
        let mut misspelled_words: Vec<String> = Vec::new();

        while idx < self.word.len() {
            match CAMEL_CASE_SHORT_MATCH.find_at(&self.word, idx) {
                Some(m) => {
                    let sub_word = m.as_str().to_string();
                    if !self.is_dictionary_word(dictionary, &sub_word) {
                        misspelled_words.push(sub_word);
                    }
                    idx = m.end();
                }
                // We shouldn't get here, but if we do we should stop processing
                None => break,
            }
        }

        if misspelled_words.is_empty() {
            Ok(())
        } else {
            Err(misspelled_words)
        }
    }

    fn check_snake_case(&self, dictionary: &Vec<String>) -> Result<(), Vec<String>> {
        let mut misspelled_words: Vec<String> = Vec::new();

        for sub_word in self.word.split("_") {
            let sub_word_string = sub_word.to_string();
            if !self.is_dictionary_word(dictionary, &sub_word_string) {
                misspelled_words.push(sub_word_string);
            }
        }

        if misspelled_words.is_empty() {
            Ok(())
        } else {
            Err(misspelled_words)
        }
    }
}

#[cfg(test)]
mod test;
