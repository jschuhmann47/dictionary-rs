use std::env;

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
    let resp = reqwest::blocking::get(url)
        .expect("failed to get url")
        .text()
        .expect("failed to get text");
    let doc_body = scraper::Html::parse_document(&resp);
    let definitions = scraper::Selector::parse("div.def.ddef_d.db").expect("failed to create selector");
    let list_of_definitions = doc_body
        .select(&definitions)
        .map(|d| d.text().collect())
        .collect::<Vec<String>>();
    if list_of_definitions.is_empty() {
        println!("Not found");
        return;
    }
    println!("Definitions found for \"{}\":", args[1..].join(" "));
    for (i, definition) in list_of_definitions.iter().enumerate() {
        println!("  {}: {}", i+1, &definition[0..definition.len() - 2]);
    }
}
