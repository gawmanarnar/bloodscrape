extern crate reqwest;
extern crate scraper;

fn main() {
    find("https://www.wowprogress.com/gearscore/us/char_rating/prev/1/lfg.1/raids_week./lang.en/class.paladin?sortby=ts");
}

fn get_character_name(cell: &std::string::String) -> String {
    let fragment = scraper::Html::parse_fragment(cell);
    let selector = scraper::Selector::parse("a").unwrap();

    let atag = fragment.select(&selector).next().unwrap();
    return String::from(atag.text().next().unwrap());
}

fn find(url: &str) {
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    let table = table_extract::Table::find_first(&body).unwrap();
    for row in &table {
        let mut index = 0;
        for cell in row.iter() {
            match index {
                0 => println!("{:#?}", get_character_name(cell)),
                _ => continue,
            }
            index += 1;
        }
    }
}
