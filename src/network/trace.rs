use ext_php_rs::prelude::*;
use ext_php_rs::types::Zval;
use serde_json::Value as JsonValue;
use reqwest::blocking::get;

use crate::zval::convert_json_to_zval;


#[php_function(ignore_module)]
pub fn trace(key: Option<String>) -> Result<Zval, PhpException> {
    let json_value = fetch_cloudflare_meta()?;

    if let Some(k) = key {
        match json_value.get(&k) {
            Some(value) => Ok(convert_json_to_zval(value.clone())),
            None => Err(PhpException::default(format!(
                "Error: Key '{}' not found in JSON response.",
                k
            ))),
        }
    } else {
        Ok(convert_json_to_zval(json_value))
    }
}

fn fetch_cloudflare_meta() -> Result<JsonValue, PhpException> {
    match fetch_url_content("https://speed.cloudflare.com/meta") {
        Ok(content) => serde_json::from_str::<JsonValue>(&content)
            .map_err(|_| PhpException::default("Failed to parse JSON response.".to_string())),
        Err(_) => Err(PhpException::default("Failed to send request to Cloudflare.".to_string())),
    }
}

fn fetch_url_content(url: &str) -> Result<String, u8> {
    match get(url) {
        Ok(res) if res.status().is_success() => res
            .text()
            .map_err(|_| 0), // Handle text conversion errors
        _ => Err(0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_url_content_success() {
        let url: &str = "https://httpbin.org/get";
        let result: Result<String, u8> = fetch_url_content(url);
        assert!(result.is_ok(), "Expected OK, got {:?}", result);
    }

    #[test]
    fn test_fetch_url_content_failure() {
        let url: &str = "https://nonexistent.url";
        let result: Result<String, u8> = fetch_url_content(url);
        assert!(result.is_err(), "Expected Err, got {:?}", result);
    }
}