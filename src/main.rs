extern crate reqwest;
extern crate scraper;
extern crate table_extract;

fn main() {
    find("https://www.wowprogress.com/gearscore/us/char_rating/prev/1/lfg.1/raids_week./lang.en/class.paladin?sortby=ts");
}

fn get_character(row: &table_extract::Row) -> String {
    let fragment = scraper::Html::parse_fragment(row.get("Character").unwrap());
    let selector = scraper::Selector::parse("a").unwrap();

    let atag = fragment.select(&selector).next().unwrap();
    return String::from(atag.text().next().unwrap());
}

fn find(url: &str) {
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    let table = table_extract::Table::find_first(&body).unwrap();
    for row in &table {
        println!("{}", get_character(&row));
    }
}
