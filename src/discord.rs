use std::{collections::HashMap, env};

pub async fn send_to_webhook(quote: String) -> Result<(), Box<dyn std::error::Error>> {
    let webhook_url = env::var("DISCORD_WEBHOOK_URL").expect("Error! Discord webhook not set!");
    // print!("Discord URL {:?}", webhook_url);

    let client = reqwest::Client::new();

    let mut discord_data = HashMap::new();
    
    discord_data.insert("content", quote);

    print!("Data to send to discord {:?}", discord_data);

    let res = client.post(webhook_url).json(&discord_data).send().await?;

    println!("Discord response {:?}", res.text().await?);

    Ok(())
}
