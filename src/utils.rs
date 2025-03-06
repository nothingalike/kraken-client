//! Utility functions for the Kraken API client

use std::collections::HashMap;
use url::Url;

/// Build a URL with query parameters
pub fn build_url(base_url: &str, path: &str, params: Option<&HashMap<String, String>>) -> String {
    let mut url = format!("{}{}", base_url, path);
    
    if let Some(params) = params {
        if !params.is_empty() {
            let mut url_obj = Url::parse(&url).expect("Invalid URL");
            
            for (key, value) in params {
                url_obj.query_pairs_mut().append_pair(key, value);
            }
            
            url = url_obj.to_string();
        }
    }
    
    url
}

/// Convert a HashMap to a URL encoded string
pub fn hashmap_to_url_encoded(params: &HashMap<String, String>) -> String {
    params
        .iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<String>>()
        .join("&")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_build_url() {
        let base_url = "https://api.kraken.com";
        let path = "/0/public/Ticker";
        
        // Test without params
        let url = build_url(base_url, path, None);
        assert_eq!(url, "https://api.kraken.com/0/public/Ticker");
        
        // Test with params
        let mut params = HashMap::new();
        params.insert("pair".to_string(), "XBTUSD".to_string());
        
        let url = build_url(base_url, path, Some(&params));
        assert!(url.starts_with("https://api.kraken.com/0/public/Ticker?"));
        assert!(url.contains("pair=XBTUSD"));
    }
    
    #[test]
    fn test_hashmap_to_url_encoded() {
        let mut params = HashMap::new();
        params.insert("nonce".to_string(), "1614232229325".to_string());
        params.insert("pair".to_string(), "XBTUSD".to_string());
        
        let encoded = hashmap_to_url_encoded(&params);
        assert!(encoded.contains("nonce=1614232229325"));
        assert!(encoded.contains("pair=XBTUSD"));
        assert_eq!(encoded.split('&').count(), 2);
    }
}
