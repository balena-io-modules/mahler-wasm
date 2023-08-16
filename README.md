# mahler-wasm

WASM compiled utilities for [Mahler](https://github.com/balena-io-modules/mahler).

This module uses [wasm-pack](https://rustwasm.github.io/wasm-pack/) to expose some essential 
functions from Rust to make them accessible in Node.js.

## Provided functions

- `diff`. Create a [JSON Patch](https://datatracker.ietf.org/doc/html/rfc6902/) to transform between two JSON serializable objects. This function just exposes the functionality from the [json-patch crate](https://crates.io/crates/json-patch).
- `patch`. Apply a [JSON Patch](https://datatracker.ietf.org/doc/html/rfc6902/) to a JSON serializable object. This function just exposes the functionality from the [json-patch crate](https://crates.io/crates/json-patch).

Example

```typescript
import { diff, patch } from "mahler-wasm";

const left = {
  "title": "Goodbye!",
  "author" : {
    "givenName" : "John",
    "familyName" : "Doe"
  },
  "tags":[ "example", "sample" ],
  "content": "This will be unchanged"
};

const right = {
  "title": "Hello!",
  "author" : {
    "givenName" : "John"
  },
  "tags": [ "example" ],
  "content": "This will be unchanged",
  "phoneNumber": "+01-123-456-7890"
};

const p = diff(left, right);
console.log(p);
// [
//   { "op": "remove", "path": "/author/familyName" },
//   { "op": "remove", "path": "/tags/1" },
//   { "op": "replace", "path": "/title", "value": "Hello!" },
//   { "op": "add", "path": "/phoneNumber", "value": "+01-123-456-7890" },
// ]
const doc = patch(left, p);
console.log(doc);
// {
//   "title": "Hello!",
//   "author" : {
//     "givenName" : "John"
//   },
//   "tags": [ "example" ],
//   "content": "This will be unchanged",
//   "phoneNumber": "+01-123-456-7890"
// }
```

## Installing

This module does not have any dependencies for installing, to install, you can just use 

```
npm install mahler-wasm
```

## Building mahler-wasm

To build the module, these are the dependencies

- [Rust](https://www.rust-lang.org) 
- The [wasm32-wasi](https://bytecodealliance.github.io/cargo-wasi/steps.html) target
- [wasm-pack](https://rustwasm.github.io/wasm-pack/)

Before building, you need to install the repository dependencies

```
# Pull the repository if you haven't done it yet
git clone https://github.com/balena-io-modules/mahler-wasm.git && cd mahler-wasm

# Install dependencies
npm install
```

Assuming you have all dependencies installed, you can build using

```
npm run build
```

And use the following to run tests

```
npm run test
```

## License

Licensed under

* [Apache License, Version 2.0](LICENSE)
