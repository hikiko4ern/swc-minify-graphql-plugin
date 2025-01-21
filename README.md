# swc-plugin-minify-graphql

[GraphQL] query and schema minimizer plugin for [SWC]

## Compatibility

Since WASM plugins are not backward compatible (see [swc-project/swc#5060][swc-wasm-compat-issue], [Selecting the version - SWC][selecting-swc-core]), use the table below to select the correct plugin version:

| plugin version | used `swc_core` version | potentially compatible `swc_core` versions |
| -------------: | ----------------------: | :----------------------------------------- |
|          `0.2` |                `10.6.1` | `>=10`                                     |
|          `0.1` |                 `1.0.2` | `>=0.98.0 <10`                             |

## Usage

Install the package:

```sh
npm i swc-plugin-minify-graphql
```

and add it to your SWC config.

### Basic

The plugin handles string literals and template literals marked with the GraphQL comment

```ts
const QUERY = /* GraphQL */ `
	query ($id: ID!) {
		image (id: $id) {
			...img
		}
	}

	fragment img on Image {
		id
		url
	}
`;
```

minifying them:

```ts
const QUERY = /* GraphQL */ `query($id:ID!){image(id:$id){...img}}fragment img on Image{id url}`;
```

Minification is supported not only for entire queries and schemas, but also for their individual parts (e.g., fragments):

```ts
const IMAGE_FIELDS = /* GraphQL */ `
	id
	url
`;
const IMAGE_FRAGMENT = /* GraphQL */ `
	fragment image on Image {
		id
		url
	}
`;

// becomes

const IMAGE_FIELDS = /* GraphQL */ `id url`;
const IMAGE_FRAGMENT = /* GraphQL */ `fragment image on Image{id url}`;
```

GraphQL comments are case-insensitive and can have any number of whitespace characters and asterisks at the beginning and end. `/* graphql */`, `/* GraphQL */`, `/** GraphQL */` and even `/* *** * gRaPhQl * *** */` will work.

### Template literals with expressions

Expressions within template literals are also supported:

```ts
const IMAGE = /* GraphQL */ `
	id
	url
`;

const ENTITY = /* GraphQL */ `
	id
	image {
		${IMAGE}
		previewUrl
	}
`;

// becomes

const IMAGE = /* GraphQL */ `id url`;

const ENTITY = /* GraphQL */ `id image{${IMAGE} previewUrl}`;
```

But with a single exception: expressions cannot break GraphQL tokens. The following code is invalid:

```ts
const LONG = 'Long';

const FIELD = /* GraphQL */ `
	id
	some${LONG}FieldName
`;

// becomes

const FIELD = /* GraphQL */ `id some ${LONG} FieldName`;

// instead of

const FIELD = /* GraphQL */ `id some${LONG}FieldName`;
```

While the minified code may be correct in some cases, this usage is not intended and can be broken at any time.

### `gql` tag support <!-- spell-checker: ignore gql -->

`gql`/`graphql` tagged template literals are not currently supported, and there are no plans to add support. You can use other plugins like [`graphql-tag-swc-plugin`] that support minification.

## Credits

- [`graphql-minify`](https://github.com/dan-lee/graphql-minify-rs): a re-implementation of [`stripIgnoredCharacters`](https://graphql-js.org/api/function/stripignoredcharacters/) from the [GraphQL.js reference implementation](https://github.com/graphql/graphql-js) in Rust

## License

Licensed under either of

- The Unlicense
  ([UNLICENSE](./UNLICENSE) or https://unlicense.org/UNLICENSE)
- MIT license
  ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

<!-- links -->

[GraphQL]: https://graphql.org
[SWC]: https://swc.rs
[swc-wasm-compat-issue]: https://github.com/swc-project/swc/issues/5060
[selecting-swc-core]: https://swc.rs/docs/plugin/selecting-swc-core
[`graphql-tag-swc-plugin`]: https://github.com/rishabh3112/graphql-tag-swc-plugin
