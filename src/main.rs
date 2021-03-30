extern crate reqwest;
extern crate scraper;

fn main() {
    find("https://www.wowprogress.com/gearscore/us/char_rating/prev/1/lfg.1/raids_week./lang.en/class.paladin?sortby=ts");
}

fn get_atag(cell: &str) -> String {
    let fragment = scraper::Html::parse_fragment(cell);
    let selector = scraper::Selector::parse("a").unwrap();

    let option = fragment.select(&selector).next();
    if option.is_none() {
        return String::new();
    }

    let atag = option.unwrap();
    return String::from(atag.text().next().unwrap());
}

fn get_character_name(cell: &str) -> String {
    return get_atag(cell);
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

fn get_realm(cell: &str) -> String {
    return get_atag(cell);
}

fn get_posted_time(cell: &str) -> String {
    let fragment = scraper::Html::parse_fragment(cell);
    let selector = scraper::Selector::parse("span").unwrap();

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
        println!("");
        for cell in row.iter() {
            match index {
                0 => println!("Character: {:#?}", get_character_name(cell)),
                1 => println!("Guild Name: {:#?}", get_guild_name(cell)),
                3 => println!("Realm: {:#?}", get_realm(cell)),
                4 => println!("Item Level: {:#?}", cell),
                5 => println!("Posted: {:#?}", get_posted_time(cell)),
                _ => (),
            }
            index += 1;
        }
    }
}
