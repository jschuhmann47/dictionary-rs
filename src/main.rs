use std::env;

use scraper::{Html, Selector};

const DEFINITIONS_SELECTOR: &str = "div.def.ddef_d.db";
const SUGGESTIONS_SELECTOR: &str = "li.lbt.lp-5.lpl-20 > a";

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() <= 1 {
        println!("Missing arguments!");
        return;
    }
    let query = args[1..].join("-");
    let url = format!(
        "https://dictionary.cambridge.org/dictionary/english/{}",
        query
    );
    let list_of_definitions = get_selectors_from_url(&url, DEFINITIONS_SELECTOR);

    if list_of_definitions.is_empty() {
        println!("Not found");
        let url = format!(
            "https://dictionary.cambridge.org/spellcheck/english/?q={}",
            query
        );
        let list_of_suggestions = get_selectors_from_url(&url, SUGGESTIONS_SELECTOR);
        println!("Perhaps you wanted to say:");
        for (i, suggestion) in list_of_suggestions.iter().enumerate() {
            println!("  {}: {}", i + 1, suggestion);
        }
        return;
    }
    println!("Definitions found for \"{}\":", args[1..].join(" "));
    for (i, definition) in list_of_definitions.iter().enumerate() {
        // the len - 2 is to truncate the ending that comes like "foo: " to just "foo"
        println!("  {}: {}", i + 1, &definition[0..definition.len() - 2]);
    }
}

fn get_url_as_html(url: &str) -> Html {
    let res = reqwest::blocking::get(url)
        .expect("failed to get url")
        .text()
        .expect("failed to get text");
    scraper::Html::parse_document(&res)
}

fn get_selectors(body: &Html, selector: Selector) -> Vec<String> {
    body.select(&selector)
        .map(|d| d.text().collect())
        .collect::<Vec<String>>()
}

fn get_selectors_from_url(url: &str, selector: &str) -> Vec<String> {
    let doc_body = get_url_as_html(&url);
    let suggestions = scraper::Selector::parse(selector).expect("failed to create selector");
    get_selectors(&doc_body, suggestions)
}
