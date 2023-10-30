use reqwest::blocking::Client;
use scraper::{Html, Selector};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Buat instance client
    let client = Client::new();

    // Lakukan permintaan GET ke URL
    let response = client.get("https://versus.com/id/phone").send()?;
    let body = response.text()?;

    // Parsing dokumen HTML
    let document = Html::parse_document(&body);

    // Definisikan selector untuk elemen yang ingin diekstraksi
    let selector = Selector::parse(".product-card__title").unwrap();

    // Loop melalui elemen yang sesuai dengan selector dan cetak teksnya
    for element in document.select(&selector) {
        println!("{}", element.text().collect::<String>());
    }

    Ok(())
}
