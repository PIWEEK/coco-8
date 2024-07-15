use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn message() -> String {
    return format!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_returns_hello() {
        assert_eq!(message(), "Hello, world!".to_string())
    }
}
