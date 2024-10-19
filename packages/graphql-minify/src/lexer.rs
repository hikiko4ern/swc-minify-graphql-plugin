use bumpalo::{collections::String as BumpaloString, Bump};
use logos::{Lexer, Logos, Span};

use super::block_string::{dedent_block_lines_mut, print_block_string, BlockStringToken};
use crate::block_string::BlockStringLines;

#[derive(Debug, PartialEq, Clone, Default)]
/// An enumeration of errors that can occur during the lexing process.
pub enum LexingError {
    #[default]
    UnknownToken,
    /// First value is the index of the first character of the unterminated string
    UnterminatedString(Span),
}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"([\s,]+|#[^\r\n]*)+")]
#[logos(error = LexingError)]
pub(crate) enum Token<'a> {
    #[token("{")]
    BraceOpen,

    #[token("}")]
    BraceClose,

    #[token("(")]
    ParenOpen,

    #[token(")")]
    ParenClose,

    #[token("[")]
    BracketOpen,

    #[token("]")]
    BracketClose,

    #[token(":")]
    Colon,

    #[token("=")]
    Equals,

    #[token("!")]
    Exclamation,

    #[token("?")]
    Question,

    #[token("&")]
    Ampersand,

    #[token("|")]
    Pipe,

    #[token("...")]
    Ellipsis,

    #[regex(r#"""""#)]
    BlockStringDelimiter,

    #[regex(r#""([^"\\]*(\\.[^"\\]*)*)""#, |lexer| match lexer.slice() {
      s if s.contains(['\n', '\r']) => Err(LexingError::UnterminatedString(lexer.span())),
      s => Ok(s),
    })]
    String(&'a str),

    #[regex("-?[0-9]+")]
    Int(&'a str),

    #[regex("-?[0-9]+\\.[0-9]+(e-?[0-9]+)?")]
    Float(&'a str),

    #[regex("true|false")]
    Bool(&'a str),

    #[regex("@[a-zA-Z_][a-zA-Z0-9_]*")]
    Directive(&'a str),

    #[regex("\\$[a-zA-Z_][a-zA-Z0-9_]*")]
    Variable(&'a str),

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier(&'a str),
}

pub(crate) fn parse_block_string<'a, 'bump>(
    lexer: &mut Lexer<'a, Token<'a>>,
    bump: &'bump mut Bump,
) -> BumpaloString<'bump> {
    let remainder = lexer.remainder();

    let mut block_string_lines = BlockStringLines::with_capacity_in(5, bump);

    {
        let mut block_lexer = BlockStringToken::lexer(remainder);
        let mut current_line = BumpaloString::new_in(bump);
        let mut max_line_length = 0;

        while let Some(Ok(token)) = block_lexer.next() {
            match token {
                BlockStringToken::NewLine => {
                    max_line_length = max_line_length.max(current_line.len());
                    block_string_lines.push(current_line);
                    current_line = BumpaloString::with_capacity_in(max_line_length, bump);
                }
                BlockStringToken::Text
                | BlockStringToken::Quote
                | BlockStringToken::EscapeSeq
                | BlockStringToken::EscapedTripleQuote => {
                    current_line.push_str(block_lexer.slice());
                }
                BlockStringToken::TripleQuote => break,
            }
        }

        if !current_line.is_empty() {
            block_string_lines.push(current_line);
        }

        lexer.bump(remainder.len() - block_lexer.remainder().len());
    }

    dedent_block_lines_mut(&mut block_string_lines);
    print_block_string(bump, &block_string_lines)
}
