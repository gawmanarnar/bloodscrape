extern crate reqwest;
extern crate table_extract;
extern crate scraper;

fn main() {
    find("https://www.wowprogress.com/gearscore/us/char_rating/prev/1/lfg.1/raids_week./lang.en/class.paladin?sortby=ts");
}

fn find(url: &str) {
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    let table = table_extract::Table::find_first(&body).unwrap();
    for row in &table {
        let fragment = scraper::Html::parse_fragment(row.get("Character").unwrap());
        let selector = scraper::Selector::parse("a").unwrap();

        let a = fragment.select(&selector).next().unwrap();
        println!("{}", a.text().next().unwrap());
    }
}
