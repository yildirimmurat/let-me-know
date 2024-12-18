use rusqlite::{params, Connection, Result};
use reqwest::Client;
use serde_json::{Value, from_str};
use crate::user_db::User;

#[derive(Debug)]
pub struct Product {
    pub name: String,
    pub api_url: String,
    pub search_url: String,
    pub tracking: String,
}
pub fn insert_product(conn: &Connection, name: &str, api_url: &str, search_url: &str, tracking: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO products (name, api_url, search_url, tracking) VALUES (?1, ?2, ?3, ?4)",
        params![name, api_url, search_url, tracking],
    )?;

    Ok(())
}

pub fn get_all_products(conn: &Connection) -> Result<Vec<Product>> {
    let mut stmt = conn.prepare("SELECT name, api_url, search_url, tracking FROM users")?;
    let products_iter = stmt.query_map([], |row| {
        Ok(Product {
            name: row.get(0)?,
            api_url: row.get(1)?,
            search_url: row.get(2)?,
            tracking: row.get(3)?,
        })
    })?;

    let mut products = Vec::new();
    for product in products_iter {
        products.push(product?);
    }

    Ok(products)
}

pub async fn fetch_active_products(conn: &Connection, client: &Client) -> Result<Vec<Product>> {
    let mut stmt = conn.prepare("SELECT id, name, api_url, search_url, tracking FROM products")?;
    let product_iter = stmt.query_map([], |row| {
        Ok(Product {
            name: row.get(1)?,
            api_url: row.get(2)?,
            search_url: row.get(3)?,
            tracking: row.get(4)?,
        })
    })?;

    let mut active_products = Vec::new();

    for product in product_iter {
        let product = product?;

        let response = client.get(&product.api_url).send().await.unwrap();
        let json: Value = response.json().await.unwrap();

        if let Some(products_field) = json.get("products") {
            if let products_array = products_field.as_array() {
                let tracking_json = serde_json::to_string(&products_array).unwrap();
                let deserialized_array: Vec<Value> = from_str(&tracking_json).unwrap();

                // Check if the array is empty
                if !deserialized_array.is_empty() {
                    active_products.push(product);
                }
            }
        } else {
            // If "products" field is neither array nor object, treat it as invalid or empty
            eprintln!("Unexpected structure for products field");
        }
    }

    Ok(active_products)
}
