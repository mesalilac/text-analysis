use clap::Parser;
use colored::Colorize;
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::exit;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long)]
    file: PathBuf,

    #[arg(
        short,
        long,
        default_value_t = 10,
        help = "Display top N letters and words, 0 for all"
    )]
    top: usize,

    #[arg(short, long, default_value_t = false)]
    json: bool,

    #[arg(short, long)]
    no_color: bool,
}

#[derive(Serialize, Debug)]
struct Frequency {
    value: String,
    count: i32,
    percentage: f32,
}

impl Frequency {
    fn new(value: String, count: i32, total_list: usize) -> Self {
        Self {
            value,
            count,
            percentage: (count as f32 / total_list as f32) * 100.0,
        }
    }
}

#[derive(Serialize, Debug)]
struct Report {
    file: PathBuf,
    text: String,
    top: usize,
    total_row_characters: usize,
    total_words: usize,
    total_letters: usize,
    words_hashmap: HashMap<String, i32>,
    letters_hashmap: HashMap<String, i32>,
}

impl Report {
    pub fn new(file: PathBuf, top: usize, file_contents: String) -> Self {
        let total_row_characters = file_contents.chars().count();
        let text = sanitize_text(file_contents);
        let total_words = text.split(" ").count();
        let total_letters = text.split("").count();

        Self {
            file,
            text,
            top,
            total_row_characters,
            total_words,
            total_letters,
            words_hashmap: HashMap::new(),
            letters_hashmap: HashMap::new(),
        }
    }

    fn hashmap_into_frequency(&self, hashmap: HashMap<String, i32>) -> Vec<Frequency> {
        let mut list: Vec<(String, i32)> = hashmap.clone().into_iter().collect();
        list.sort_by(|a, b| b.1.cmp(&a.1));

        if self.top != 0 {
            list.truncate(self.top);
        }

        list.iter()
            .map(|x| Frequency::new(x.0.clone(), x.1, self.total_letters))
            .collect()
    }

    pub fn print_json(&self) {
        let words_freq_list: Vec<Frequency> =
            self.hashmap_into_frequency(self.words_hashmap.clone());
        let letters_freq_list: Vec<Frequency> =
            self.hashmap_into_frequency(self.letters_hashmap.clone());

        println!(
            "{}",
            serde_json::json!({
                "report": {
                    "info": {
                        "file": self.file,
                        "top": self.top,
                        "total_row_characters": self.total_row_characters,
                        "total_words": self.total_words,
                        "total_letters": self.total_letters,
                        "total_unique_words": self.words_hashmap.len(),
                        "total_unique_letters": self.letters_hashmap.len(),
                    },
                    "words": words_freq_list,
                    "letters": letters_freq_list
                },
            })
        );
    }

    pub fn print(&self) {
        let words_freq_list: Vec<Frequency> =
            self.hashmap_into_frequency(self.words_hashmap.clone());
        let letters_freq_list: Vec<Frequency> =
            self.hashmap_into_frequency(self.letters_hashmap.clone());

        println!("--- Text Analysis Report ---");

        println!();

        println!("Input File: {}", self.file.to_str().unwrap());
        println!("Total Row Characters: {}", self.total_row_characters);
        println!("Total Words: {}", self.total_words);
        println!("Total Letters: {}", self.total_letters);
        println!("Total Unique Words: {}", self.words_hashmap.len());
        println!("Total Unique Letters: {}", self.letters_hashmap.len());

        println!();

        println!("--- Words (Top: {}) ---", self.top);
        for (index, word) in words_freq_list.iter().enumerate() {
            println!(
                "{}. {}: {} ({:.2}%)",
                index + 1,
                word.value,
                word.count,
                word.percentage
            );
        }

        println!();

        println!("--- Letters (Top: {}) ---", self.top);
        for (index, letter) in letters_freq_list.iter().enumerate() {
            println!(
                "{}. {}: {} ({:.2}%)",
                index + 1,
                letter.value,
                letter.count,
                letter.percentage
            );
        }

        println!();

        println!("--- Report End ---");
    }

    pub fn generate(&mut self) {
        for word in self.text.split(" ") {
            if word.is_empty() {
                continue;
            }

            for letter in word.chars() {
                let letter_string = letter.to_string();

                let mut letter_count: i32 =
                    self.letters_hashmap.remove(&letter_string).unwrap_or(0);
                letter_count += 1;

                self.letters_hashmap.insert(letter_string, letter_count);
            }

            let mut word_count: i32 = self.words_hashmap.remove(word).unwrap_or(0);
            word_count += 1;

            self.words_hashmap.insert(word.to_string(), word_count);
        }
    }
}

fn main() {
    let args = Cli::parse();

    if args.file.is_dir() {
        eprintln!("{} is a directory", args.file.to_str().unwrap());
        exit(1);
    }

    if !args.file.exists() {
        eprintln!("{} does not exist", args.file.to_str().unwrap());
        exit(1);
    }

    let Ok(file_contents) = std::fs::read_to_string(&args.file) else {
        eprintln!("{} could not be read", args.file.to_str().unwrap());
        exit(1);
    };

    if args.no_color || std::env::var_os("NO_COLOR").is_some() {
        colored::control::set_override(false);
    }

    let mut text_analysis_report = Report::new(args.file, args.top, file_contents);

    text_analysis_report.generate();

    if args.json {
        text_analysis_report.print_json();
    } else {
        text_analysis_report.print();
    }
}

fn sanitize_text(input_text: String) -> String {
    let mut text = input_text;

    text = text.to_lowercase();
    text = text.trim().to_string();

    text = text.replace("\n", " ");
    text = text.replace("\r", " ");
    text = text.replace("\t", " ");
    text = text.replace("`", "");
    text = text.replace("~", "");
    text = text.replace("!", "");
    text = text.replace("@", "");
    text = text.replace("#", "");
    text = text.replace("$", "");
    text = text.replace("%", "");
    text = text.replace("^", "");
    text = text.replace("&", "");
    text = text.replace("*", "");
    text = text.replace("(", "");
    text = text.replace(")", "");
    text = text.replace("-", "");
    text = text.replace("_", "");
    text = text.replace("=", "");
    text = text.replace("+", "");
    text = text.replace("[", "");
    text = text.replace("{", "");
    text = text.replace("]", "");
    text = text.replace("}", "");
    text = text.replace(";", "");
    text = text.replace(":", "");
    text = text.replace("'", "");
    text = text.replace('"', "");
    text = text.replace(",", "");
    text = text.replace("<", "");
    text = text.replace(".", "");
    text = text.replace(">", "");
    text = text.replace("/", "");
    text = text.replace("?", "");
    text = text.replace("\\", "");
    text = text.replace("|", "");
    text = text.replace("0", "");
    text = text.replace("1", "");
    text = text.replace("2", "");
    text = text.replace("3", "");
    text = text.replace("4", "");
    text = text.replace("5", "");
    text = text.replace("6", "");
    text = text.replace("7", "");
    text = text.replace("8", "");
    text = text.replace("9", "");

    text
}
