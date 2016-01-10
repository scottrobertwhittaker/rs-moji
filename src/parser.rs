pub fn parse<'a>(tokens: Vec<char>) -> Vec<&'a str> {
    let program = tokens.iter().cloned().collect::<String>();
    println!("Parsing '{}'...", program);
    vec!["A", "S", "T"]
}
