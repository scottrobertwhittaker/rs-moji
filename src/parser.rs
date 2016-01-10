/// A node in the abstract syntax tree of the Mojikun language.
#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    /// 👈: Move the pointer to the right
    PointLeft,

    /// 👉: Move the pointer to the left
    PointRight,

    /// 👍: Increment the memory cell under the pointer
    ThumbsUp,

    /// 👎: Decrement the memory cell under the pointer
    ThumbsDown,

    /// 💾: Input a character and store it in the cell at the pointer
    Save,

    /// 💻: Output the character signified by the cell at the pointer
    Display,

    /// 🔃: Jump past the matching 🔙 if the cell under the pointer is 0
    Loop,

    /// 🔙: Jump back to the matching 🔃 if the cell under the pointer is nonzero
    EndLoop,

    /// Represents a non-emoji character.
    None
}

/// Returns a vector of `Node`s corresponding to the specified `tokens`.
pub fn parse(tokens: Vec<char>) -> Vec<Node> {
    tokens.iter().map(|t| match *t {
        '👈' => Node::PointLeft,
        '👉' => Node::PointRight,
        '👍' => Node::ThumbsUp,
        '👎' => Node::ThumbsDown,
        '💾' => Node::Save,
        '💻' => Node::Display,
        '🔃' => Node::Loop,
        '🔙' => Node::EndLoop,
         _  => Node::None // TBD Is there something special about `None`?
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::Node;
    use super::parse;

    // TBD All these `Some(&Node::`s are pretty ugly. Will the `&`s be elided
    // if we `.unwrap`?

    #[test]
    fn test_point_right() {
      let tokens = vec!['👉'];
      assert_eq!(Some(&Node::PointRight), parse(tokens).first());
    }

    #[test]
    fn test_point_left() {
      let tokens = vec!['👈'];
      assert_eq!(Some(&Node::PointLeft), parse(tokens).first());
    }

    #[test]
    fn test_thumbs_up() {
      let tokens = vec!['👍'];
      assert_eq!(Some(&Node::ThumbsUp), parse(tokens).first());
    }

    #[test]
    fn test_thumbs_down() {
      let tokens = vec!['👎'];
      assert_eq!(Some(&Node::ThumbsDown), parse(tokens).first());
    }

    #[test]
    fn test_display() {
      let tokens = vec!['💻'];
      assert_eq!(Some(&Node::Display), parse(tokens).first());
    }

    #[test]
    fn test_save() {
      let tokens = vec!['💾'];
      assert_eq!(Some(&Node::Save), parse(tokens).first());
    }

    #[test]
    fn test_loop() {
      let tokens = vec!['🔃'];
      assert_eq!(Some(&Node::Loop), parse(tokens).first());
    }

    #[test]
    fn test_end_loop() {
      let tokens = vec!['🔙'];
      assert_eq!(Some(&Node::EndLoop), parse(tokens).first());
    }
}
