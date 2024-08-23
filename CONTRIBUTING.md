# Contributing

- [Conventional Commits](#conventional-commits)
- [Setup](#setup)
- [Development](#development)
  - [Available commands](#available-commands)
- [Releasing new version](#releasing-new-version)

## Conventional Commits

This project uses [Conventional Commits](https://www.conventionalcommits.org/en) to automate versioning. If you're a new contributor, don't worry about this. When you open a PR, a maintainer will change the PR's title so it's in the style of conventional commits, but that's all.

## Setup

The extension requires globally installed:

- [Node.js][node.js] with [Corepack][corepack] enabled

  The version of Node.js used is specified in [`.nvmrc`](./.nvmrc). It is recommended to use version managers - e.g. [fnm][fnm].

- [Rust][rust]

  If [`rustup`][rustup] is used, it should automatically install everything you need when building. If not, you need to manually install the version and target specified in the [rust-toolchain.toml](./rust-toolchain.toml) file.

- [`cargo-run-bin`][cargo-run-bin]

  To install:
  ```sh
  cargo install --locked cargo-run-bin
  ```

## Development

1. install dependencies:

   ```sh
   pnpm i
   ```

2. run `pnpm dev` to build a plugin

3. use `lib/swc_plugin_minify_graphql.wasm` directly, `pnpm link` or any other way to load the plugin

### Available commands

Here are some helpful commands:

- `pnpm dev` - continuously builds the library when files are changed
- `pnpm test` - runs the tests once
- `pnpm test:watch` - runs tests in watch mode

## Releasing new version

At the moment releases are done manually. To publish a release, run:

1. `pnpm bump` - automatically bumps the version based on commits from the previous tag and generates [`CHANGELOG.md`](./CHANGELOG.md)
2. `pnpm publish` - publishes the package

<!-- links -->

[node.js]: https://nodejs.org
[corepack]: https://github.com/nodejs/corepack
[fnm]: https://github.com/Schniz/fnm
[rust]: https://www.rust-lang.org
[rustup]: https://www.rust-lang.org/tools/install
[cargo-run-bin]: https://crates.io/crates/cargo-run-bin
