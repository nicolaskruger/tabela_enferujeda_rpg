use regex::Regex;
use std::fs;

const D_D_BOOK: &str = "./d&d_book/";

pub struct Dice {
    title: String,
    lines: Vec<String>,
    dice: String,
}

pub fn dice_title(dice: &Dice) -> String {
    dice.title.to_string()
}

pub fn dice_dice(dice: &Dice) -> String {
    dice.dice.to_string()
}

pub fn dice_lines(dice: &Dice) -> Vec<String> {
    dice.lines.clone()
}

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

pub fn swap_pdf_txt(file_name: String) -> String {
    file_name.replace(".pdf", ".txt")
}

pub fn decod_save(file_name: String, file_content: String) {
    fs::write(rune_full_path(file_name), file_content).unwrap();
}

pub fn decod_read(file_name: String) -> String {
    fs::read_to_string(rune_full_path(file_name)).unwrap()
}

fn dice_regex() -> Regex {
    Regex::new(r"^d\d+").unwrap()
}

pub fn rune_content_do_dice(content: String) -> Vec<Dice> {
    let store: Vec<&str> = content.split("\n").collect();
    let idxs: Vec<usize> = store
        .iter()
        .enumerate()
        .filter(|x| dice_regex().is_match(x.1))
        .map(|i| i.0)
        .collect();

    idxs.iter()
        .map(|idx| Dice {
            title: store[idx - 1].to_string(),
            dice: store[*idx].to_string(),
            lines: vec![],
        })
        .collect()
}
