use regex::Regex;
use std::fs;

fn pdf_regex() -> Regex {
    Regex::new(r"\.pdf$").unwrap()
}

pub fn all_runes() -> Vec<String> {
    fs::read_dir("./d&d_book/")
        .into_iter()
        .flatten()
        .filter_map(|f| f.ok())
        .filter_map(|f| f.file_name().into_string().ok())
        .filter(|f| pdf_regex().is_match(f))
        .collect()
}
