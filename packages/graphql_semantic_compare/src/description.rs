use std::fmt::Display;

use textwrap::dedent;

pub(crate) struct Description {
    result: *const str,
    _dedented: String,
}

impl Description {
    pub(crate) const TRIPLE_QUOTES: &str = r#"""""#;

    pub(crate) fn new_cleaned(src: &str) -> Self {
        let src = if src.starts_with(Self::TRIPLE_QUOTES) {
            &src[3..src.len() - 3]
        } else if src.starts_with('"') {
            &src[1..src.len() - 1]
        } else {
            src
        };

        let dedented = dedent(src);
        let trimmed = dedented.trim_matches([' ', '\r', '\n']) as *const _;

        Self {
            _dedented: dedented,
            result: trimmed,
        }
    }

    fn as_str(&self) -> &str {
        unsafe { &*self.result }
    }
}

impl PartialEq for Description {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Display for Description {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
