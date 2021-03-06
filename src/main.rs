#[macro_use]
extern crate dotenv_codegen;
extern crate reqwest;
extern crate scraper;
extern crate serde;
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct Log {
    #[serde(rename = "encounterID")]
    encounter_id: u32,
    #[serde(rename = "encounterName")]
    encounter_name: String,
    #[serde(rename = "percentile")]
    parse: f64,
    difficulty: i8,
}

#[derive(Default, Debug)]
struct Character {
    name: String,
    class: String,
    guild: String,
    realm: String,
    ilvl: f32,
    time: String,
    heroic: HashMap<u32, Log>,
    mythic: HashMap<u32, Log>,
}

impl Character {
    fn get_mythic(&self) -> String {
        let count: f64 = self.mythic.keys().len() as f64;

        if count == 0.0 {
            return String::new();
        }

        let mut sum: f64 = 0.0;
        for log in self.mythic.values() {
            sum += log.parse;
        }

        format!(
            "{kills}/10M Performance: {parses}",
            kills = count,
            parses = format!("{:.2}", sum / count)
        )
    }

    fn get_color(&self) -> u32 {
        match self.class.as_str() {
            "deathknight" => 12852794,
            "demon_hunter" => 10694857,
            "druid" => 16743434,
            "hunter" => 11195250,
            "mage" => 4179947,
            "monk" => 65432,
            "paladin" => 16026810,
            "priest" => 16777215,
            "rogue" => 16774248,
            "shaman" => 28893,
            "warlock" => 8882414,
            "warrior" => 13015917,
            _ => 0,
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let arg = &args[1];

    let wowprogress_url = format!("https://www.wowprogress.com/gearscore/us/char_rating/prev/1/lfg.1/raids_week./lang.en/class.{class}?sortby=ts", 
                                    class = arg);

    let mut characters: Vec<Character> = Vec::new();
    find_characters(&wowprogress_url, &mut characters);

    let api_key = dotenv!("WARCRAFTLOGS_API_KEY");
    let discord = dotenv!("DISCORD_WEBHOOK");
    let client = reqwest::blocking::Client::new();
    for char in &mut characters {
        if char.ilvl <= 210.0 {
            continue;
        }

        get_logs(char, api_key);
        if char.heroic.keys().len() == 0 {
            continue;
        }

        char.class = arg.to_string();
        post_discord(discord, &client, char);
    }

    print!("{}", discord);
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

fn get_ilvl(cell: &str) -> f32 {
    let float = cell.parse::<f32>();
    if float.is_err() {
        return 0.0;
    }
    float.unwrap()
}

fn make_character(entry: &table_extract::Row) -> Character {
    let mut character: Character = Default::default();
    for (index, cell) in entry.iter().enumerate() {
        match index {
            0 => character.name = get_character_name(cell),
            1 => character.guild = get_guild_name(cell),
            3 => character.realm = get_realm(cell),
            4 => character.ilvl = get_ilvl(cell),
            5 => character.time = get_posted_time(cell),
            _ => (),
        }
    }

    character
}

fn process_realm(realm: &str) -> String {
    realm.replace(' ', "-").replace('\'', "").to_lowercase()
}

fn get_logs(character: &mut Character, api_key: &str) {
    let warcraftlogs_url = format!("https://www.warcraftlogs.com:443/v1/rankings/character/{character_name}/{realm}/{region}?timeframe=historical&api_key={key}",
                                    character_name = character.name.to_lowercase(),
                                    realm = process_realm(&character.realm),
                                    region = "us",
                                    key = api_key);

    println!("{}", warcraftlogs_url);
    let logs: Vec<Log> = reqwest::blocking::get(&warcraftlogs_url)
        .unwrap()
        .json()
        .unwrap();

    for log in logs {
        if log.difficulty == 5 {
            character.mythic.insert(log.encounter_id, log);
        } else if log.difficulty == 4 {
            character.heroic.insert(log.encounter_id, log);
        }
    }
    println!("{:?}", character);
}

fn find_characters(url: &str, characters: &mut Vec<Character>) {
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    let table = table_extract::Table::find_first(&body).unwrap();
    for row in &table {
        characters.push(make_character(&row));
    }
}

fn post_discord(webhook: &str, client: &reqwest::blocking::Client, character: &Character) {
    let r = process_realm(&character.realm);
    let c = character.name.to_lowercase();

    let link = format!(
        "https://www.warcraftlogs.com/character/us/{realm}/{character}",
        realm = r,
        character = c
    );

    let json = json!({
        "embeds": [
            {
                "title": character.name,
                "description": format!("{class}\n{realm}\n{ilvl}\n{mythic}", class = character.class, realm = character.realm, ilvl = character.ilvl, mythic = character.get_mythic()),
                "url": link,
                "color": character.get_color(),
                "fields": [
                    {
                      "name": "Warcraft Logs",
                      "value": link
                    },
                    {
                      "name": "Raider.io",
                      "value": format!("https://raider.io/characters/us/{realm}/{class}", realm = r, class = c)
                    }
                ]
            }
        ]
    }
    );
    client.post(webhook).json(&json).send().unwrap();
}
