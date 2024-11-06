use afl::fuzz;
use graphql_minify::{minify, MinifyAllocator};

fn main() {
    fuzz!(|data: &str| {
        let _ = minify(data, &mut MinifyAllocator::default());
    });
}
