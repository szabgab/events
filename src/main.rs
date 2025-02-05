use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fs;

use fs_extra::copy_items;
use fs_extra::dir;

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
    let options = dir::CopyOptions::new().overwrite(true);
    let mut from_paths = Vec::new();
    from_paths.push("static/js");
    copy_items(&from_paths, "_site", &options)?;

    let filename = "rust.yaml";
    let text = fs::read_to_string(filename).unwrap();

    let now: DateTime<FixedOffset> = Utc::now().fixed_offset();
    //println!("now:  {now}");

    let events: Vec<Event> = serde_yaml::from_str(&text).unwrap_or_else(|err| {
        eprintln!("Could not parse YAML file: {err}");
        std::process::exit(1);
    });

    let events = events
        .iter()
        .filter(|event| {
            let dt = DateTime::parse_from_str(&event.start, "%Y-%m-%dT%H:%M:%S%z").unwrap();
            dt.cmp(&now) != Ordering::Less
        })
        .collect::<Vec<&Event>>();

    generate_text(&events)?;
    generate_html(&events, now)?;

    Ok(())
}

fn generate_html(
    events: &[&Event],
    now: DateTime<FixedOffset>,
) -> Result<(), Box<dyn std::error::Error>> {
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
        "now": now.to_string(),
    });
    let output = template.render(&globals).unwrap();

    std::fs::write("_site/index.html", output)?;
    Ok(())
}

fn generate_text(events: &[&Event]) -> Result<(), Box<dyn std::error::Error>> {
    let template = include_str!("../templates/text.txt");
    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(template)
        .unwrap();

    let globals = liquid::object!({
        "events": events,
    });
    let output = template.render(&globals).unwrap();

    std::fs::write("_site/rust.txt", output)?;
    Ok(())
}
