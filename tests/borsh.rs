#![cfg(feature = "borsh")]

use serde_json::{json, Value};

#[test]
fn borsh_serialization_works() {
    let original = json!({
        "code": 200,
        "success": true,
        "payload": {
            "features": [
                "serde",
                "json"
            ],
            "homepage": null
        }
    });

    let serialized = borsh::to_vec(&original).unwrap();
    let deserialized: Value = borsh::from_slice(&serialized).unwrap();

    assert_eq!(deserialized, original);
}
