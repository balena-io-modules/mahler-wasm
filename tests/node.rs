#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use gloo_utils::format::JsValueSerdeExt;
use mahler_wasm::{diff, patch, Patch};
use serde_json::{from_value, json, Value};
use wasm_bindgen::JsValue;

#[wasm_bindgen_test]
fn lib_test() {
    let left = json!({
      "title": "Goodbye!",
      "author" : {
        "givenName" : "John",
        "familyName" : "Doe"
      },
      "tags":[ "example", "sample" ],
      "content": "This will be unchanged"
    });
    let left = JsValue::from_serde(&left).unwrap();

    let right = json!({
      "title": "Hello!",
      "author" : {
        "givenName" : "John"
      },
      "tags": [ "example" ],
      "content": "This will be unchanged",
      "phoneNumber": "+01-123-456-7890"
    });
    let right = JsValue::from_serde(&right).unwrap();

    let p = diff(&left, &right)
        .or_else(|_| Err("Failed to find patch!"))
        .unwrap();
    assert_eq!(
        p,
        from_value::<Patch>(json!([
          { "op": "remove", "path": "/author/familyName" },
          { "op": "remove", "path": "/tags/1" },
          { "op": "replace", "path": "/title", "value": "Hello!" },
          { "op": "add", "path": "/phoneNumber", "value": "+01-123-456-7890" },
        ]))
        .unwrap()
    );

    let obj = patch(&left, p)
        .or_else(|_| Err("Failed to apply patch!"))
        .unwrap();

    assert_eq!(
        obj.into_serde::<Value>().unwrap(),
        json!({
          "title": "Hello!",
          "author" : {
            "givenName" : "John"
          },
          "tags": [ "example" ],
          "content": "This will be unchanged",
          "phoneNumber": "+01-123-456-7890"
        })
    );
}
