use serde_json::{self, Result};

pub fn decode(text: String) -> Vec<(String, String)> {
    serde_json::from_str(&text).expect("Failed to decode json!")
}

pub fn encode(data: Vec<(String, String)>) -> String {
    serde_json::to_string(&data).expect("Failed to encode json!")
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_decode() {
        let text = r#"
        [
            ["bilibili", "123456"],
            ["ilibilib", "654321"]
        ]
        "#.to_string();
        assert_eq!(decode(text), vec![
            (String::from("bilibili"), String::from("123456")),
            (String::from("ilibilib"), String::from("654321"))
        ])
    }

    #[test]
    fn test_encode() {
        let text = r#"[["bilibili","123456"],["ilibilib","654321"]]"#.to_string();
        assert_eq!(encode(vec![
            (String::from("bilibili"), String::from("123456")),
            (String::from("ilibilib"), String::from("654321"))
        ]), text)
    }

}