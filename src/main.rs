use clap::Parser;
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
    file: String,
    top: usize,
    total_row_characters: usize,
    total_words: usize,
    total_letters: usize,
    total_unique_words: usize,
    total_unique_letters: usize,
    words: Vec<Frequency>,
    letters: Vec<Frequency>,
}

impl Report {
    pub fn print_json(&self) {
        println!(
            "{}",
            serde_json::json!({
                "report": {
                    "file": self.file,
                    "top": self.top,
                    "total_row": self.total_row_characters,
                    "total_sanitized_words": self.total_words,
                    "total_sanitized_letters": self.total_letters,
                    "total_sanitized_unique_words": self.total_unique_words,
                    "total_sanitized_unique_letters": self.total_unique_letters,
                    "words": self.words,
                    "letters":self.letters
                },
            })
        );
    }

    pub fn print(&self) {
        println!("--- Text Analysis Report ---");

        println!();

        println!("Input File: {}", self.file);
        println!("Total Row Characters: {}", self.total_row_characters);
        println!("Total Words: {}", self.total_words);
        println!("Total Letters: {}", self.total_letters);
        println!("Total Unique Words: {}", self.total_unique_words);
        println!("Total Unique Letters: {}", self.total_unique_letters);

        println!();

        println!("--- Words (Top: {}) ---", self.top);
        for (index, word) in self.words.iter().enumerate() {
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
        for (index, letter) in self.letters.iter().enumerate() {
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

    let text = sanitize_text(file_contents.clone());

    let mut letters: HashMap<String, i32> = HashMap::new();
    let mut words: HashMap<String, i32> = HashMap::new();

    for word in text.split(" ") {
        if word.is_empty() {
            continue;
        }

        for letter in word.chars() {
            let letter_string = letter.to_string();

            let mut letter_count: i32 = letters.remove(&letter_string).unwrap_or(0);
            letter_count += 1;

            letters.insert(letter_string, letter_count);
        }

        let mut word_count: i32 = words.remove(word).unwrap_or(0);
        word_count += 1;

        words.insert(word.to_string(), word_count);
    }

    let total_words_count = text.split(" ").count();
    let total_letters_count = text.split("").count();

    let total_unique_words_count = words.len();
    let total_unique_letters_count = letters.len();

    let mut words_list: Vec<(String, i32)> = words.into_iter().collect();
    words_list.sort_by(|a, b| b.1.cmp(&a.1));

    let mut letters_list: Vec<(String, i32)> = letters.into_iter().collect();
    letters_list.sort_by(|a, b| b.1.cmp(&a.1));

    if args.top != 0 {
        words_list.truncate(args.top);
        letters_list.truncate(args.top);
    }
    let words_freq_list: Vec<Frequency> = words_list
        .iter()
        .map(|x| Frequency::new(x.0.clone(), x.1, total_words_count))
        .collect();
    let letters_freq_list: Vec<Frequency> = letters_list
        .iter()
        .map(|x| Frequency::new(x.0.clone(), x.1, total_letters_count))
        .collect();

    let text_analysis_report = Report {
        file: args.file.to_str().unwrap().to_string(),
        top: args.top,
        total_row_characters: file_contents.split("").count(),
        total_words: total_words_count,
        total_letters: total_letters_count,
        total_unique_words: total_unique_words_count,
        total_unique_letters: total_unique_letters_count,
        words: words_freq_list,
        letters: letters_freq_list,
    };

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
