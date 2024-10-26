# Changelog

## [0.1.2] - 2024-10-26

### Bug Fixes

- include `graphql-minify` license in pack

### Features

- print the parsing error span relative to the file

### Performance

- re-use single allocator for `graphql_minify::minify`

## [0.1.1] - 2024-10-22

### Bug Fixes

- fix `license` to `Unlicense`

## [0.1.0] - 2024-10-22

### Features

- minimize string and template literals
- report error span
- **(graphql-minify)** do not print space before ellipsis

### Miscellaneous Tasks

- **(bench)** add minify benchmarks

### Performance

- **(graphql-minify)** do not re-split lines in `print_block_string`
- **(graphql-minify)** use `bumpalo` for allocations
- **(graphql-minify)** do not store pointers in `Token`
- **(graphql-minify)** get rid of multiple memory allocations in `print_block_string`
- **(graphql-minify)** use `memchr` for string validation
