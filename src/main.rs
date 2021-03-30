extern crate reqwest;
extern crate scraper;

fn main() {
    find("https://www.wowprogress.com/gearscore/us/char_rating/prev/1/lfg.1/raids_week./lang.en/class.paladin?sortby=ts");
}

fn get_character_name(cell: &str) -> String {
    let fragment = scraper::Html::parse_fragment(cell);
    let selector = scraper::Selector::parse("a").unwrap();

    let atag = fragment.select(&selector).next().unwrap();
    return String::from(atag.text().next().unwrap());
}

fn get_guild_name(cell: &str) -> String {
    let fragment = scraper::Html::parse_fragment(cell);
    let selector = scraper::Selector::parse("nobr").unwrap();

    let nobr_option = fragment.select(&selector).next();
    if nobr_option.is_none() {
        return String::new();
    }

    let nobr = nobr_option.unwrap();
    return String::from(nobr.text().next().unwrap());
}

fn find(url: &str) {
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    let table = table_extract::Table::find_first(&body).unwrap();
    for row in &table {
        let mut index = 0;
        for cell in row.iter() {
            //println!("{:#?}", cell);
            match index {
                0 => println!("{:#?}", get_character_name(cell)),
                1 => println!("{:#?}", get_guild_name(cell)),
                _ => continue,
            }
            index += 1;
        }
    }
}
