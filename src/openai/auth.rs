use std::env;

pub fn get_openai_key() -> String {
    match env::var("OPENAI_KEY") {
        Ok(val) => val,
        Err(e) => panic!("OPENAI_KEY not set: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_openai_key() {
        let key = get_openai_key();
        assert!(!key.is_empty());
    }
}
