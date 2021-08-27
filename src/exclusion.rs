//! Line exclusions
//!
//! Defines categories of lines that should be excluded from the cleaned output

use exclusion_functions::{is_empty, only, starts_with};
use Exclusion::{Empty, Only, StartsWith};

/// Exclusion categories
pub enum Exclusion {
    /// Empty lines
    Empty,
    /// Lines that start with the specified string
    ///
    /// This is commonly used to determine code comments
    ///
    /// # Example
    /// ```bash
    /// // This is JS a comment
    /// //! A rust module comment
    /// >> Maybe a comment in some lang
    /// ```
    StartsWith(String),
    /// Lines that only contain the provided characters
    ///
    /// This is commonly used to determine lines that are only braces/brackets
    ///
    /// # Example
    /// ```bash
    /// {
    /// });
    /// (    
    /// ```
    Only(Vec<char>),
}

impl Exclusion {
    /**
    Check whether the input line matches this exclusion

    Returns `true` if the input `line` matches this exclusion variant

    # Example

    ```
    # use fclean::exclusion::Exclusion;
    let empty = Exclusion::Empty;
    assert_eq!(empty.check(""), true);
    ```
    */
    pub fn check(&self, line: &str) -> bool {
        match self {
            Empty => is_empty(line),
            StartsWith(prefix) => starts_with(line, prefix),
            Only(chars) => only(line, chars),
        }
    }
}

mod exclusion_functions {
    pub fn is_empty(line: &str) -> bool {
        line.trim().is_empty()
    }

    pub fn starts_with(line: &str, prefix: &str) -> bool {
        line.trim().starts_with(&prefix)
    }

    pub fn only(line: &str, chars: &[char]) -> bool {
        line.replace(chars, "").trim().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_empty_given_empty_string_returns_true() {
        let empty_strings = vec!["", "   "];

        for s in empty_strings {
            assert!(is_empty(s))
        }
    }

    #[test]
    fn is_empty_given_nonempty_string_returns_false() {
        let empty_strings = vec!["A", " String  "];

        for s in empty_strings {
            assert!(!is_empty(s))
        }
    }

    #[test]
    fn starts_with_given_string_with_prefix_returns_true() {
        let prefixed_strings = vec![
            ("|| weird comment", "||"),
            ("# python comment", "#"),
            ("// JS comment", "//"),
        ];

        for (comment, prefix) in prefixed_strings {
            assert!(starts_with(comment, prefix))
        }
    }

    #[test]
    fn starts_with_given_string_without_prefix_returns_false() {
        let prefixed_strings = vec![
            ("weird comment", "||"),
            ("python comment", "#"),
            ("JS comment", "//"),
        ];

        for (comment, prefix) in prefixed_strings {
            assert!(!starts_with(comment, prefix))
        }
    }

    #[test]
    fn only_given_string_with_only_char_returns_true() {
        let chars = vec!['{', '}', '(', ')', ';'];
        let lines = vec!["  {  ", "   }", "(", ");", "({", "  });", "{{", "}}"];

        for line in lines {
            assert!(only(line, &chars));
        }
    }

    #[test]
    fn only_given_string_with_other_chars_returns_false() {
        let chars = vec!['{', '}', '(', ')'];
        let lines = vec![
            "if {  ",
            "(int",
            "ant)",
            "=> ({",
            " return x; })",
            "{{val",
            "str}}",
        ];

        for line in lines {
            assert!(!only(line, &chars));
        }
    }
}
