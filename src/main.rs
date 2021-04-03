extern crate reqwest;
extern crate scraper;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let arg = &args[1];

    find(&format!("https://www.wowprogress.com/gearscore/us/char_rating/prev/1/lfg.1/raids_week./lang.en/class.{class}?sortby=ts", class = arg));
}

fn get_content(cell: &str, tag: &str) -> String {
    let fragment = scraper::Html::parse_fragment(cell);
    let selector = scraper::Selector::parse(tag).unwrap();

    let option = fragment.select(&selector).next();
    if option.is_none() {
        return String::new();
    }

    let atag = option.unwrap();
    return String::from(atag.text().next().unwrap());
}

fn get_character_name(cell: &str) -> String {
    get_content(cell, "a")
}

fn get_guild_name(cell: &str) -> String {
    get_content(cell, "nobr")
}

fn get_realm(cell: &str) -> String {
    get_content(cell, "a")
}

fn get_posted_time(cell: &str) -> String {
    get_content(cell, "span")
}

fn find(url: &str) {
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    let table = table_extract::Table::find_first(&body).unwrap();
    for row in &table {
        println!();
        for (index, cell) in row.iter().enumerate() {
            match index {
                0 => println!("Character: {:#?}", get_character_name(cell)),
                1 => println!("Guild Name: {:#?}", get_guild_name(cell)),
                3 => println!("Realm: {:#?}", get_realm(cell)),
                4 => println!("Item Level: {:#?}", cell),
                5 => println!("Posted: {:#?}", get_posted_time(cell)),
                _ => (),
            }
        }
    }
}
