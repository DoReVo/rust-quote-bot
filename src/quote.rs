use core::panic;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct QuoteResponse {
    pub a: String,
    pub h: String,
    pub q: String,
}

pub async fn get_quote() -> Result<QuoteResponse, Box<dyn std::error::Error>> {
    let response = reqwest::get("https://zenquotes.io/api/random").await;

    let req_response = match response {
        Ok(data) => data,
        Err(error) => panic!("Error: {:?}", error),
    };

    let response_data = match req_response.json::<Vec<QuoteResponse>>().await {
        Ok(data) => data,
        Err(error) => panic!("Error {:?}", error),
    };

    // Take the first item
    let first_quote = match response_data.first() {
        Some(quote) => quote.clone(),
        None => panic!("Error"),
    };

    return Ok(first_quote);
}
