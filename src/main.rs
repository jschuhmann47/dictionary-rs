use std::{collections::HashSet, env};

use scraper::{Html, Selector};

const BASE_API_URL: &str = "https://api.dictionaryapi.dev";

struct DefinitionResponse {
    
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() <= 1 {
        println!("Missing arguments!");
        return;
    }
    let query = args[1..].join("%20");
    let url = format!("{}/api/v2/entries/en/{}",BASE_API_URL.to_string(), query);
    let client = reqwest::blocking::Client::builder()
        .build()
        .expect("failed to build client");
    let list_of_definitions = get_definitions(&client, &url).expect("failed to get definitions");

    if list_of_definitions.is_empty() {
        println!("Expression not found");
        return;
    }
    println!("Definitions found for \"{}\":", args[1..].join(" "));
    for (i, definition) in remove_duplicates(list_of_definitions).iter().enumerate() {
        let mut chars_to_extract = 0;
        if definition.ends_with(":") {
            chars_to_extract = 1;
        }
        println!(
            "  {}: {}",
            i + 1,
            &definition[0..definition.len() - chars_to_extract]
        );
    }
}

fn get_definitions(client: &reqwest::blocking::Client, url: &str) -> Result<Vec<String>, String> {
    let res = DefinitionResponse{};
    client.get(url).json(json)
}

fn remove_duplicates(list: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut new_list = list.clone();
    new_list.retain(|c| seen.insert(c.to_string()));
    new_list
}
