use std::error::Error;

use chrono::Datelike;
use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

use fs_extra::copy_items;
use fs_extra::dir;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use ical::generator::{Emitter, IcalCalendarBuilder};

#[derive(EnumIter, Deserialize, Serialize, Debug, PartialEq, Clone)]
enum Language {
    English,
    French,
    Hebrew,
}

#[derive(EnumIter, Deserialize, Serialize, Debug, PartialEq, Clone)]
enum Category {
    Perl,
    Python,
    Rust,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
struct Event {
    title: String,
    url: String,
    name: String,
    address: String,
    language: Language,
    start: String, // 2024-06-06T18:00:00+03:00
    category: Category,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all("_site").unwrap();
    let options = dir::CopyOptions::new().overwrite(true);
    let from_paths = vec!["static/js"];
    copy_items(&from_paths, "_site", &options)?;

    let now: DateTime<FixedOffset> = Utc::now().fixed_offset();
    //println!("now:  {now}");

    let mut events = read_events("rust.yaml", now);
    events.extend(read_events("python.yaml", now));

    let mut counts = HashMap::new();

    generate_page(now, &events, "all")?;
    counts.insert(String::from("All"), events.len());

    for category in Category::iter() {
        let cat_str = format!("{:?}", category);

        let cat_events = events
            .iter()
            .filter(|event| event.category == category)
            .cloned()
            .collect::<Vec<Event>>();
        counts.insert(cat_str.clone(), cat_events.len());

        generate_text(&events, format!("{}", cat_str.to_lowercase()).as_str())?;

        generate_html(
            &cat_events,
            now,
            format!("{}", cat_str.to_lowercase()).as_str(),
        )?;

        for language in Language::iter() {
            let language_str = format!("{:?}", language);

            let these_events = cat_events
                .iter()
                .filter(|event| event.language == language)
                .cloned()
                .collect::<Vec<Event>>();

            counts.insert(format!("{}-{}", cat_str, language_str), these_events.len());

            generate_html(
                &these_events,
                now,
                format!(
                    "{}-{}",
                    cat_str.to_lowercase(),
                    language_str.to_lowercase()
                )
                .as_str(),
            )?;
        }
    }

    let mut counts = counts.iter().collect::<Vec<_>>();
    counts.sort_by_key(|entry| entry.1);
    counts.reverse();
    generate_main_page(now, counts)?;

    Ok(())
}

fn generate_page(now: DateTime<FixedOffset>, events: &Vec<Event>, filename: &str) -> Result<(), Box<dyn Error>> {
    generate_ical(events, filename)?;
    generate_text(events, filename)?;
    generate_html(events, now, filename)?;
    generate_markdown(events, now, filename)?;
    Ok(())
}

fn read_events(filename: &str, now: DateTime<FixedOffset>) -> Vec<Event> {
    let text = fs::read_to_string(filename).unwrap();

    let events: Vec<Event> = serde_yaml::from_str(&text).unwrap_or_else(|err| {
        eprintln!("Could not parse YAML file: {err}");
        std::process::exit(1);
    });

    events
        .into_iter()
        .filter(|event| {
            let dt = DateTime::parse_from_str(&event.start, "%Y-%m-%dT%H:%M:%S%z").unwrap();
            dt.cmp(&now) != Ordering::Less
        })
        .collect::<Vec<Event>>()
}

fn generate_main_page(
    now: DateTime<FixedOffset>,
    counts: Vec<(&String, &usize)>,
) -> Result<(), Box<dyn Error>> {
    let template = include_str!("../templates/index.html");
    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(template)
        .unwrap();

    let globals = liquid::object!({
        "title": "Virtual Events",
        "now": now.to_string(),
        "counts": counts,
    });
    let output = template.render(&globals).unwrap();

    std::fs::write(format!("_site/index.html"), output)?;
    Ok(())
}

fn generate_ical(events: &[Event], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let cal = IcalCalendarBuilder::version("2.0")
        .gregorian()
        .prodid("-//ical-rs//github.com//")
        .build();

    for event in events {
        println!("{}", event.start);
        // let event = IcalEventBuilder::tzid("Europe/Berlin")
        // .uid("UID for identifying this event.")
        // .changed("20210115")
        // .one_day("20220101")
        // .set(ical_property!("SUMMARY", "New Year"))
        // .build();
        // cal.events.push(event);
    }

    let output = cal.generate();
    std::fs::write(format!("_site/{filename}.ical"), output)?;

    Ok(())
}

fn generate_markdown(
    events: &[Event],
    now: DateTime<FixedOffset>,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let title = format!("Online Rust events {}", now.month());
    let text = format!("I found the following Rust-related online events for the next 10 days.");

    let template = include_str!("../templates/list.md");
    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(template)
        .unwrap();

    let globals = liquid::object!({
        "title": title,
        "text": text,
        "events": events,
    });
    let output = template.render(&globals).unwrap();

    std::fs::write(format!("_site/{filename}.md"), output)?;
    Ok(())
}

fn generate_html(
    events: &[Event],
    now: DateTime<FixedOffset>,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let html = "";
    let template = include_str!("../templates/list.html");
    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(template)
        .unwrap();

    let globals = liquid::object!({
        "content": &html,
        "events": events,
        "title": "Virtual Events",
        "now": now.to_string(),
    });
    let output = template.render(&globals).unwrap();

    std::fs::write(format!("_site/{filename}.html"), output)?;
    Ok(())
}

fn generate_text(events: &[Event], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let template = include_str!("../templates/list.txt");
    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(template)
        .unwrap();

    let globals = liquid::object!({
        "events": events,
    });
    let output = template.render(&globals).unwrap();

    std::fs::write(format!("_site/{filename}.txt"), output)?;
    Ok(())
}
