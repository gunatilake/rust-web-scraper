fn main() {
    let response = reqwest::blocking::get(
        "https://www.bayut.com/for-sale/apartments/dubai/",
    )
    .unwrap()
    .text()
    .unwrap();

    let document = scraper::Html::parse_document(&response);

    let title_selector = scraper::Selector::parse("article>div>a").unwrap();    

    let titles = document.select(&title_selector).map(|x| x.value().attr("href").unwrap());

    let website = "https://www.bayut.com";

    titles
        .zip(1..101)
        .for_each(|(item, number)| println!("{}. {}{}", number, website, item));
}