use std::ops::Deref;

use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub(crate) enum BlockStringToken {
    #[token("\"")]
    Quote,

    #[regex(r#"\n|\r\n|\r"#)]
    NewLine,

    #[regex(r#"\\""""#)]
    EscapedTripleQuote,

    #[regex(r#"\\"#)]
    EscapeSeq,

    #[regex(r#"[^"\r\n\\]+"#)]
    Text,

    #[token(r#"""""#)]
    TripleQuote,
}

#[derive(Default)]
pub(crate) struct BlockStringLines {
    lines: Vec<String>,
    total_len: usize,
}

impl BlockStringLines {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            lines: Vec::with_capacity(capacity),
            ..Default::default()
        }
    }

    pub fn push(&mut self, line: String) {
        self.total_len += line.len();
        self.lines.push(line);
    }
}

impl Deref for BlockStringLines {
    type Target = [String];

    fn deref(&self) -> &Self::Target {
        &self.lines
    }
}

pub(crate) fn print_block_string(lines: &BlockStringLines) -> String {
    let force_leading_new_line = lines.len() > 1
        && lines[1..].iter().all(|line| {
            line.is_empty()
                || line
                    .as_bytes()
                    .first()
                    .copied()
                    .is_some_and(is_graphql_whitespace)
        });

    let last_line = lines.last();
    let force_trailing_newline =
        last_line.is_some_and(|line| line.ends_with(['"', '\\']) && !line.ends_with(r#"\""""#));

    let mut result = String::with_capacity(lines.total_len + 7);

    result.push_str(r#"""""#);

    if force_leading_new_line {
        result.push('\n');
    }

    for line in lines.iter() {
        result.push_str(line.as_str());
        result.push('\n');
    }

    if !force_trailing_newline && result.ends_with('\n') {
        result.pop();
    }

    result.push_str(r#"""""#);
    result
}

pub(crate) fn dedent_block_lines_mut(lines: &mut BlockStringLines) {
    let mut common_indent = usize::MAX;
    let mut first_non_empty_line = None;
    let mut last_non_empty_line = None;

    for (i, line) in lines.iter().enumerate() {
        let indent = leading_whitespace(line);

        if indent < line.len() {
            first_non_empty_line.get_or_insert(i);
            last_non_empty_line = Some(i);

            if i != 0 && indent < common_indent {
                common_indent = indent;
            }
        }
    }

    let lines = &mut lines.lines;

    match (first_non_empty_line, last_non_empty_line) {
        (Some(start), Some(end)) => {
            for line in lines.iter_mut().skip(1) {
                if line.len() > common_indent {
                    *line = line.split_off(common_indent);
                } else {
                    line.clear();
                }
            }

            lines.drain(..start);
            lines.drain((end + 1 - start)..);
        }
        _ => lines.clear(),
    }
}

fn is_graphql_whitespace(b: u8) -> bool {
    b == b' ' || b == b'\t'
}

fn leading_whitespace(s: &str) -> usize {
    s.as_bytes()
        .iter()
        .position(|&b| !is_graphql_whitespace(b))
        .unwrap_or(s.len())
}

#[cfg(test)]
mod test_dedent {
    use super::{dedent_block_lines_mut, BlockStringLines};

    fn get_dedented_vec(lines: &[&str]) -> Vec<String> {
        let mut bsl = BlockStringLines::with_capacity(lines.len());
        for line in lines {
            bsl.push(String::from(*line));
        }
        dedent_block_lines_mut(&mut bsl);
        bsl.lines
    }

    #[test]
    fn does_not_dedent_first_line() {
        assert_eq!(get_dedented_vec(&["  a"]), &["  a"]);
        assert_eq!(get_dedented_vec(&[" a", "  b"]), &[" a", "b"]);
    }

    #[test]
    fn removes_minimal_indentation_length() {
        assert_eq!(get_dedented_vec(&["", " a", "  b"]), &["a", " b"]);
        assert_eq!(get_dedented_vec(&["", "  a", " b"]), &[" a", "b"]);
        assert_eq!(
            get_dedented_vec(&["", "  a", " b", "c"]),
            &["  a", " b", "c"]
        );
    }

    #[test]
    fn dedent_both_tab_and_space_as_single_character() {
        assert_eq!(
            get_dedented_vec(&["", "\ta", "          b"]),
            &["a", "         b"]
        );
        assert_eq!(
            get_dedented_vec(&["", "\t a", "          b"]),
            &["a", "        b"]
        );
        assert_eq!(
            get_dedented_vec(&["", " \t a", "          b"]),
            &["a", "       b"]
        );
    }

    #[test]
    fn dedent_do_not_take_empty_lines_into_account() {
        assert_eq!(get_dedented_vec(&["a", "", " b"]), &["a", "", "b"]);
        assert_eq!(get_dedented_vec(&["a", " ", "  b"]), &["a", "", "b"]);
    }

    #[test]
    fn removes_uniform_indentation_from_a_string() {
        let lines = vec![
            "",
            "    Hello,",
            "      World!",
            "",
            "    Yours,",
            "      GraphQL.",
        ];
        assert_eq!(
            get_dedented_vec(&lines),
            &["Hello,", "  World!", "", "Yours,", "  GraphQL.",]
        );
    }

    #[test]
    fn removes_empty_leading_and_trailing_lines() {
        let lines = vec![
            "",
            "",
            "    Hello,",
            "      World!",
            "",
            "    Yours,",
            "      GraphQL.",
            "",
            "",
        ];
        assert_eq!(
            get_dedented_vec(&lines),
            &["Hello,", "  World!", "", "Yours,", "  GraphQL.",]
        );
    }

    #[test]
    fn removes_blank_leading_and_trailing_lines() {
        let lines = vec![
            "  ",
            "        ",
            "    Hello,",
            "      World!",
            "",
            "    Yours,",
            "      GraphQL.",
            "        ",
            "  ",
        ];
        assert_eq!(
            get_dedented_vec(&lines),
            &["Hello,", "  World!", "", "Yours,", "  GraphQL.",]
        );
    }

    #[test]
    fn retains_indentation_from_first_line() {
        let lines = vec![
            "    Hello,",
            "      World!",
            "",
            "    Yours,",
            "      GraphQL.",
        ];
        assert_eq!(
            get_dedented_vec(&lines),
            &["    Hello,", "  World!", "", "Yours,", "  GraphQL.",]
        );
    }

    #[test]
    fn does_not_alter_trailing_spaces() {
        let lines = vec![
            "               ",
            "    Hello,     ",
            "      World!   ",
            "               ",
            "    Yours,     ",
            "      GraphQL. ",
            "               ",
        ];
        assert_eq!(
            get_dedented_vec(&lines),
            &[
                "Hello,     ",
                "  World!   ",
                "           ",
                "Yours,     ",
                "  GraphQL. ",
            ]
        );
    }
}

#[cfg(test)]
mod test_print {
    fn print_block_string<I: AsRef<str>>(input: I) -> String {
        use super::BlockStringLines;

        let mut lines = BlockStringLines::default();

        for line in input.as_ref().lines() {
            lines.push(line.replace(r#"""""#, r#"\""""#));
        }

        super::print_block_string(&lines)
    }

    #[test]
    fn does_not_escape_characters() {
        let str = r" \ / \b \f \n \r \t";
        assert_eq!(print_block_string(str), r#"""" \ / \b \f \n \r \t""""#);
    }

    #[test]
    fn by_default_print_block_strings_as_single_line() {
        let str = r"one liner";
        assert_eq!(print_block_string(str), r#""""one liner""""#);
    }

    #[test]
    fn by_default_print_block_strings_ending_with_triple_quotation_as_multi_line() {
        let str = r#"triple quotation """"#;
        assert_eq!(print_block_string(str), r#""""triple quotation \"""""""#);
    }

    #[test]
    fn correctly_prints_single_line_with_leading_space() {
        let str = "    space-led string";
        assert_eq!(print_block_string(str), r#""""    space-led string""""#);
    }

    #[test]
    fn correctly_prints_single_line_with_leading_space_and_trailing_quotation() {
        let str = "    space-led value \"quoted string\"";
        assert_eq!(
            print_block_string(str),
            r#""""    space-led value "quoted string"
""""#
        );
    }

    #[test]
    fn correctly_prints_single_line_with_trailing_backslash() {
        let str = "backslash \\";
        assert_eq!(
            print_block_string(str),
            r#""""backslash \
""""#
        );
    }

    #[test]
    fn correctly_prints_multi_line_with_internal_indent() {
        let str = "no indent\n with indent";
        assert_eq!(
            print_block_string(str),
            r#""""
no indent
 with indent""""#
        );
    }

    #[test]
    fn correctly_prints_string_with_a_first_line_indentation() {
        let str = ["    first  ", "  line     ", "indentation", "     string"].join("\n");

        assert_eq!(
            print_block_string(&str),
            [
                r#""""    first  "#,
                "  line     ",
                "indentation",
                r#"     string""""#
            ]
            .join("\n")
        );
    }
}
