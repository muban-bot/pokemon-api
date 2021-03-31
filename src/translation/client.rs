use serde_json::Value;
use std::error::Error;

#[derive(Clone)]
pub struct ShakespeareApiClient { pub endpoint: &'static str }

impl ShakespeareApiClient {
    pub fn new() -> ShakespeareApiClient {
        ShakespeareApiClient {
            endpoint: "https://api.funtranslations.com/translate/shakespeare.json"
        }
    }

    pub async fn translate_string(&self, victim: String) -> Result<String, Box<dyn Error>> {
        let resp = reqwest::get(format!("{}?text={}", self.endpoint, victim))
            .await?
            .text()
            .await;

        let result: Value = serde_json::from_str(resp.unwrap().as_str())?;

        let translation = (&result["contents"]["translated"]).as_str();

        match translation {
            None => Err("bad request".into()),
            Some(val) => Ok(val.to_string())
        }
    }
}