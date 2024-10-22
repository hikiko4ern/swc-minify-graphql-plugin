use std::hint::black_box;

use graphql_minify::{minify, MinifyAllocator};

const SCHEMA: &str = include_str!("../test_data/github_schema.graphql");

pub fn main() {
    let _ = minify(black_box(SCHEMA), &mut MinifyAllocator::default());
}
