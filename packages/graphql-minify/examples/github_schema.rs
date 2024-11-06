use std::hint::black_box;

use graphql_minify::{minify, MinifyAllocator, MinifyError};

const SCHEMA: &str = include_str!("../test_data/valid/github_schema.graphql");

pub fn main() -> Result<(), MinifyError> {
    minify(black_box(SCHEMA), &mut MinifyAllocator::default()).map(|_| ())
}
