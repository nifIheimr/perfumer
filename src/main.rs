use csv;
use tokio::{self, time::error::Error};

use perfumer::{client::druni::{self as druni, druni_scrape}, model::druni_product::Product as Product};

#[tokio::main]
async fn main() -> Result<(), Error> {
    create_csv("druni".to_string(), druni_scrape().await);

    Ok(())
}

//async fn get_druni_page() ->


fn create_csv(name : String, products : Vec<Product>) {
    let file_name = format!("{}.csv", name);
    let path = std::path::Path::new(&file_name);
    let mut writer = csv::Writer::from_path(path).unwrap();

    writer
        .write_record(&["url", "brand", "perfume", "price"])
        .unwrap();

    for product in products {
        let url = product.url.unwrap();
        let brand = product.brand.unwrap();
        let perfume = product.perfume.unwrap();
        let price = product.price.unwrap();

        writer.write_record(&[url, brand, perfume, price]).unwrap();
    }

    writer.flush().unwrap();
}