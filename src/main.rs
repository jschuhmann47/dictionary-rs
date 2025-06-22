use std::{collections::HashSet, env};

use reqwest::StatusCode;

use crate::definitions::WordDefinition;

const BASE_API_URL: &str = "https://dictionary-api.eliaschen.dev";

mod definitions;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() <= 1 {
        println!("Missing arguments!");
        return;
    }
    let query = args[1..].join("%20");
    let url = format!("{}/api/dictionary/en/{}", BASE_API_URL.to_string(), query);
    let client = reqwest::blocking::Client::builder()
        .build()
        .expect("failed to build client");
    let list_of_definitions = match get_definitions(&client, &url){
        Ok(def) => def,
        Err(e) => {
            println!("Error occurred: {:?}", e);
            return;
        }
    };

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
    let word_response = match client.get(url).send() {
        Ok(res) => 
        match res.status() {
            StatusCode::OK =>  res
                .json::<WordDefinition>()
                .map_err(|_| "failed to deserialize".to_string())?,
            StatusCode::NOT_FOUND => return Err("not found".to_string()),
            s => return Err(format!("unhandled status code {}", s))
        }
        Err(_) => return Err("failed to GET".to_string()),
    };
    Ok(word_response.get_definitions())
}

fn remove_duplicates(list: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut new_list = list.clone();
    new_list.retain(|c| seen.insert(c.to_string()));
    new_list
}
