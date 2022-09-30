use std::collections::HashMap;
use serde::Deserialize;
use serde::Serialize;


#[derive(Debug, Serialize, Deserialize)]
struct Data {
    rates: HashMap<String, f32>,
}

    #[tokio::main]
    async fn main() -> Result<(), reqwest::Error> {
        let echo_json: Data = reqwest::Client::new()
            .get("https://open.er-api.com/v6/latest/GBP")
            .send()
            .await?
            .json()
            .await?;
    
        println!("1 GBP is worth: {:#?} USD", &echo_json.rates["USD"]);
        Ok(())
    }