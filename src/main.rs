use tabela_enferujeda_rpg::rune;

// fn lab00() {
//     let runes = rune::all_runes().join(", ");
//     println!("{}", runes);
//     let rune_name = rune::all_runes().first().unwrap().to_string();
//     let _decode = rune::rune_decoder(rune_name.clone());
//     println!("decoded");
//     let file_name = rune::swap_pdf_txt(rune_name.clone());
//     rune::decod_save(file_name, _decode);
// }

fn main() {
    let ctx = rune::decod_read("master.txt".to_string());

    let res = rune::rune_content_do_dice(ctx);

    res.iter().for_each(|dt| {
        print!(
            "title: {}, dice {}",
            rune::dice_title(dt),
            rune::dice_dice(dt)
        )
    })
}
