# graphql-minify

This is a slightly modified and optimized version of [dan-lee/graphql-minify-rs](https://github.com/dan-lee/graphql-minify-rs). The original README is below, and the original license is in [LICENSE](./LICENSE).

---

<a href="https://docs.rs/graphql-minify/latest/graphql_minify"><img alt="Docs" src="https://img.shields.io/docsrs/graphql-minify/latest"></a>
<a href="https://lib.rs/crates/graphql-minify"><img alt="Crates.io" src="https://img.shields.io/crates/v/graphql-minify"></a>
<a href="https://graphql-minify.daniellehr.de"><img alt="Demo" src="https://img.shields.io/badge/demo-wasm-blue"></a>

This is a re-implementation of [`stripIgnoredCharacters`](https://graphql-js.org/api/function/stripignoredcharacters/) from the [GraphQL.js reference implementation](https://github.com/graphql/graphql-js) in Rust. It uses [Logos](https://github.com/maciejhirsz/logos) for tokenization.

All relevant tests are ported from the reference implementation and run against the Rust implementation.

Beware: It does _not test for validity_ of the GraphQL document, its sole purpose is to minify the document as much as possible.

[**⚡️ Demo built with WASM**](http://graphql-minify.daniellehr.de)

<details>
<summary>Details</summary>

> Strips characters that are not significant to the validity or execution of a GraphQL document:
>
> - UnicodeBOM
> - WhiteSpace
> - LineTerminator
> - Comment
> - Comma
> - BlockString indentation
>
> Note: It is required to have a delimiter character between neighboring non-punctuator tokens and this function always uses single space as delimiter.
>
> It is guaranteed that both input and output documents if parsed would result in the exact same AST except for nodes location.

</details>

## Usage

```rust
use graphql_minify::minify;

fn main() {
  let minified = minify("query { user { id name } }");

  assert_eq!(minified.unwrap(), "query{user{id name}}");
}
```
