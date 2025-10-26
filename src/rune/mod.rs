use regex::Regex;
use std::fs;

const D_D_BOOK: &str = "./d&d_book/";

fn pdf_regex() -> Regex {
    Regex::new(r"\.pdf$").unwrap()
}

pub fn all_runes() -> Vec<String> {
    fs::read_dir(D_D_BOOK)
        .into_iter()
        .flatten()
        .filter_map(|f| f.ok())
        .filter_map(|f| f.file_name().into_string().ok())
        .filter(|f| pdf_regex().is_match(f))
        .collect()
}

fn rune_full_path(rune_name: String) -> String {
    format!("{}{}", String::from(D_D_BOOK), rune_name)
}

pub fn rune_decoder(rune_name: String) -> String {
    pdf_extract::extract_text(rune_full_path(rune_name)).unwrap()
}
