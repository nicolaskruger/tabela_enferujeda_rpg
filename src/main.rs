use tabela_enferujeda_rpg::rune;

fn main() {
    let runes = rune::all_runes().join(", ");
    println!("{}", runes);
}
