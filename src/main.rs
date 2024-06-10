use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct Event {
    title: String,
    url: String,
    name: String,
    address: String,
    language: String, // English
    start: String,    // 2024-06-06T18:00:00+03:00
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all("_site").unwrap();

    let filename = "rust.yaml";
    let text = fs::read_to_string(filename).unwrap();

    let events: Vec<Event> = serde_yaml::from_str(&text).unwrap_or_else(|err| {
        eprintln!("Could not parse YAML file: {err}");
        std::process::exit(1);
    });

    let html = "";
    let template = include_str!("../templates/page.html");
    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(template)
        .unwrap();

    let globals = liquid::object!({
        "content": &html,
        "events": events,
        "title": "Virtual Rust Events",
    });
    let output = template.render(&globals).unwrap();

    std::fs::write("_site/index.html", output)?;
    Ok(())
}
