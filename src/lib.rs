/// Returns a greeting message
pub fn greeting() -> String {
    String::from("Hello Warp, I am David Parker.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greeting() {
        assert_eq!(greeting(), "Hello Warp, I am David Parker.");
    }
}
