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
    let definitions =
        scraper::Selector::parse("div.def.ddef_d.db").expect("failed to create selector");
    let list_of_definitions = doc_body
        .select(&definitions)
        .map(|d| d.text().collect())
        .collect::<Vec<String>>();
    if list_of_definitions.is_empty() {
        println!("Not found");
        // https://dictionary.cambridge.org/spellcheck/english/?q=helo
        let suggestions =
            scraper::Selector::parse("li.lbt.lp-5.lpl-20 > a").expect("failed to create selector");
        let list_of_suggestions = doc_body
            .select(&suggestions)
            .map(|d| d.inner_html())
            .collect::<Vec<String>>();
            println!("Perhaps you wanted to say:");
            for (i, suggestion) in list_of_suggestions.iter().enumerate() {
                println!("  {}: {}", i + 1, suggestion);
            } 
        return;
    }
    println!("Definitions found for \"{}\":", args[1..].join(" "));
    for (i, definition) in list_of_definitions.iter().enumerate() {
        println!("  {}: {}", i + 1, &definition[0..definition.len() - 2]);
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    #[test]
    fn it_works() {
        let mut file = File::open("./src/test.html").expect("failed to open file");
        let mut data = String::with_capacity(10000);
        file.read_to_string(&mut data).expect("failed to read to string");
        let suggestions = scraper::Selector::parse("li.lbt.lp-5.lpl-20 > a").expect("failed to create selector");
        let doc = scraper::Html::parse_document(&data);
        let res = doc.select(&suggestions)
        .map(|d| d.inner_html())
        .collect::<Vec<String>>();
        assert_eq!(res.get(0).map(|f| f.clone()), Some("hello".to_string()));
    }
}