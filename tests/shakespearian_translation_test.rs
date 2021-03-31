#[cfg(test)]
mod tests {
    use pokemon::translation::client::{ShakespeareApiClient};

    #[tokio::test]
    async fn test_can_translate_string() {
        let client = ShakespeareApiClient::new();
        let result = client.translate_string(String::from("hello, world!")).await;
        let translation = result.unwrap();
        assert_eq!("Valorous morrow to thee,  sir,  ordinary!", translation)
    }
}