use gloo_utils::format::JsValueSerdeExt;
use json_patch::{diff as json_diff, patch as json_patch, PatchOperation as Operation};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tsify::Tsify;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type Operation =
    | {op: "add", path: string, value: any}
    | {op: "remove", path: string}
    | {op: "replace", path: string, value: any}
    | {op: "move", from: string, path: string}
    | {op: "copy", from: string, path: string}
    | {op: "test", path: string, value: any};
"#;

#[derive(Tsify, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
// We take advantage that the PatchOperation enum in json-patch
// is json serializable to use it as part of the Patch type.
// If it wasn't we would have to re-define the Operation enum and transform
// the json-patch operations into our own.
pub struct Patch(Vec<Operation>);

#[wasm_bindgen]
/// Create a JSON patch to transform between two JSON serializable
/// objects
///
/// A [JSON Patch](https://datatracker.ietf.org/doc/html/rfc6902/) document
/// is a JSON document that represents an array of objects.
/// Each object represents a single operation to be applied to the target JSON document.
///
/// This function uses the Rust [json-patch crate](https://crates.io/crates/json-patch),
/// compiled to WASM, to efficiently calculate the differences between two objects.
///
/// The resulting object can be used with the `patch` function to apply on
/// a new object.
///
/// # Arguments
///
/// * `left` - Source object. It needs to be a JSON serializable object
/// * `right` - Target object. It needs to be a JSON serializable object
///
/// # Examples
/// ```typescript
/// import { diff, patch } from "mahler-wasm";
///
/// const left = {
///   "title": "Goodbye!",
///   "author" : {
///     "givenName" : "John",
///     "familyName" : "Doe"
///   },
///   "tags":[ "example", "sample" ],
///   "content": "This will be unchanged"
/// };
///
/// const right = {
///   "title": "Hello!",
///   "author" : {
///     "givenName" : "John"
///   },
///   "tags": [ "example" ],
///   "content": "This will be unchanged",
///   "phoneNumber": "+01-123-456-7890"
/// };
///
/// const p = diff(left, right);
/// console.log(p);
/// // [
/// //   { "op": "remove", "path": "/author/familyName" },
/// //   { "op": "remove", "path": "/tags/1" },
/// //   { "op": "replace", "path": "/title", "value": "Hello!" },
/// //   { "op": "add", "path": "/phoneNumber", "value": "+01-123-456-7890" },
/// // ]
/// const doc = patch(left, p);
/// console.log(doc);
/// // {
/// //   "title": "Hello!",
/// //   "author" : {
/// //     "givenName" : "John"
/// //   },
/// //   "tags": [ "example" ],
/// //   "content": "This will be unchanged",
/// //   "phoneNumber": "+01-123-456-7890"
/// // }
/// ```
///
pub fn diff(left: &JsValue, right: &JsValue) -> Result<Patch, JsError> {
    let left: Value = left.into_serde()?;
    let right: Value = right.into_serde()?;

    let json_patch::Patch(ops) = json_diff(&left, &right);

    Ok(Patch(ops))
}

#[wasm_bindgen]
/// Apply a JSON patch to a JSON serializable object
///
/// JSON patch 6902
/// A [JSON Patch](https://datatracker.ietf.org/doc/html/rfc6902/) document
/// is a JSON document that represents an array of objects.
/// Each object represents a single operation to be applied to the target JSON document.
///
/// This function uses the Rust [json-patch crate](https://crates.io/crates/json-patch),
/// compiled to WASM, to efficiently apply a patch to an object.
///
/// # Arguments
/// * `obj` - Source object. It needs to be a JSON serializable object
/// * `patch` - Patch to apply.
///
pub fn patch(obj: &JsValue, patch: Patch) -> Result<JsValue, JsError> {
    let mut obj: Value = obj.into_serde()?;

    // Get ops from input
    let Patch(ops) = patch;

    // Apply the patch
    let patch = json_patch::Patch(ops);
    json_patch(&mut obj, &patch)?;

    JsValue::from_serde(&obj).map_err(JsError::from)
}
