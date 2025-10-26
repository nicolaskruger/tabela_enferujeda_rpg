use tabela_enferujeda_rpg::rune;

fn main() {
    let runes = rune::all_runes().join(", ");
    println!("{}", runes);
    let rune_name = rune::all_runes().first().unwrap().to_string();
    let decode = rune::rune_decoder(rune_name);
    println!("{}", decode)
}
