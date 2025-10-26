use tabela_enferujeda_rpg::rune;

fn main() {
    let runes = rune::all_runes().join(", ");
    println!("{}", runes);
    let rune_name = rune::all_runes().first().unwrap().to_string();
    let _decode = rune::rune_decoder(rune_name.clone());
    println!("decoded");
    let file_name = rune::swap_pdf_txt(rune_name.clone());
    rune::decod_save(file_name, _decode);
}
