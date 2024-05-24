use reqwest::Error;
use crate::model::druni_product::Product as Product;

pub async fn get_page(page : usize) -> Result<String, Error> {
    let response = reqwest::get(format!("https://www.druni.es/perfumes/premium?p={}", page.to_string()))
    .await?
    .text()
    .await?;

    Ok(response)
}

pub async fn druni_scrape() -> Vec<Product> {
    let mut products : Vec<Product> = Vec::new();
    let mut pages: Vec<usize> = (0..20).collect();
    while let Some(page) = pages.pop() {
        if let Ok(result) = get_page(page).await {
            let document = scraper::Html::parse_document(&result);

            let html_product_selector = scraper::Selector::parse("div.product-item-info").unwrap();
            let html_products = document.select(&html_product_selector);

            for html_product in html_products {
                let url = html_product
                    .select(&scraper::Selector::parse("a").unwrap())
                    .next()
                    .and_then(|a| a.value().attr("href"))
                    .map(str::to_owned);
        
                let brand = html_product
                    .select(&scraper::Selector::parse("p.product-brand").unwrap())
                    .next()
                    .map(|p| p.text().collect::<String>());
        
                let perfume = html_product
                    .select(&scraper::Selector::parse("a.product-item-link").unwrap())
                    .next()
                    .map(|a| a.text().collect::<String>());
        
                let price = html_product
                    .select(&scraper::Selector::parse("span.price").unwrap())
                    .next()
                    .map(|price| price.text().collect::<String>());
        
                let product = Product {
                    url,
                    brand,
                    perfume,
                    price
                };
        
                products.push(product);

                
            }
        }
    }
    products
}