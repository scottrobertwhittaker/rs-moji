pub fn lex(source_code: &str) -> Vec<&str> {
    println!("Lexing '{}'...", source_code);
    vec!["one", "two", "three"]
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_one_character () {
    }
}
