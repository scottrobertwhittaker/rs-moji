/// Returns a vector of characters in the specified `source_code`.
///
/// A 'real' lexer would return a vector of *tokens* for the `parser` to
/// transform into an abstract syntax tree.  But every token in Mojikun
/// consists of a single character, and no operations take arguments, so there
/// isn't really a meaningful distinction between the vector of tokens and the
/// abstract syntax tree.
pub fn lex(source_code: &str) -> Vec<char> {
    source_code.chars().collect()
}

#[cfg(test)]
mod tests {
    use super::lex;

    #[test]
    fn test_one_character() {
        let source = "👍";
        assert_eq!(vec!['👍'], lex(source));
    }

    #[test]
    fn test_two_characters() {
        let source = "👍👍";
        assert_eq!(vec!['👍', '👍'], lex(source));
    }
}
