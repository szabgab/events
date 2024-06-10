fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all("_site").unwrap();

    let html = "Hello";
    let template = include_str!("../templates/page.html");
    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(template)
        .unwrap();

    let globals = liquid::object!({
        "content": &html,
        "title": "Rust Maven demo",
    });
    let output = template.render(&globals).unwrap();

    std::fs::write("_site/index.html", output)?;
    Ok(())
}
