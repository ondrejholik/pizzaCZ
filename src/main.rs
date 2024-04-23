use reqwest;
use scraper::{Html, Selector};

fn main() {
    let url = "https://www.pizza.cz/denni-menu-pizza-west/"; // Replace this with the URL you want to scrape
    match fetch_and_parse(url) {
        Ok(_) => println!(),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn fetch_and_parse(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Fetch the URL content using reqwest
    let html = reqwest::blocking::get(url)?.text()?;

    // Parse the HTML document with scraper
    let document = Html::parse_document(&html);
    let selector = Selector::parse("main#site-content article#post-680.post-680.page.type-page.status-publish.hentry div.post-inner.thin div.entry-content p").unwrap();

    // Iterate over elements matching our selector
    for element in document.select(&selector) {
        let text = element.text().collect::<Vec<_>>().join(" ");
        println!("{}\n\n", text.trim());
    }

    Ok(())
}

