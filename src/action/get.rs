use crate::action::ActionDef;
use crate::error::{ActionError, ConfigEditError};
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Retrieve a value at the given path.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Parser)]
pub struct GetAction {
	/// The JSON Pointer (RFC6901) within the file to the value being retrieved.
	///
	/// Omit the key to fetch the entire document
	pub(crate) key: Option<String>,
}

impl ActionDef for GetAction {
	fn apply(&mut self, value: Value) -> Result<Value, ConfigEditError> {
		let path = self.key.clone().unwrap_or_default();
		value
			.pointer(&path)
			.cloned()
			.ok_or_else(|| ActionError::PointerNotFound(path).into())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use serde_json::json;
	use test_case::test_case;

	// Success test cases
	#[test_case(None, json!({"a": 1, "b": 2}), json!({"a": 1, "b": 2}) ; "get entire document")]
	#[test_case(Some("/a".to_string()), json!({"a": 1, "b": 2}), json!(1) ; "get simple path")]
	#[test_case(Some("/b".to_string()), json!({"a": 1, "b": 2}), json!(2) ; "get another simple path")]
	#[test_case(Some("/nested/value".to_string()), json!({"nested": {"value": "test"}}), json!("test") ; "get nested path")]
	#[test_case(Some("/nested/obj".to_string()), json!({"nested": {"obj": {"key": "value"}}}), json!({"key": "value"}) ; "get nested object")]
	#[test_case(Some("/arr/0".to_string()), json!({"arr": [1, 2, 3]}), json!(1) ; "get array element by index")]
	#[test_case(Some("/arr/1".to_string()), json!({"arr": [1, 2, 3]}), json!(2) ; "get another array element")]
	#[test_case(Some("/arr/2".to_string()), json!({"arr": [1, 2, 3]}), json!(3) ; "get last array element")]
	#[test_case(Some("/deep/nested/array/0".to_string()), json!({"deep": {"nested": {"array": [42, 43, 44]}}}), json!(42) ; "get deeply nested array element")]
	#[test_case(Some("/complex".to_string()), json!({"complex": {"arr": [1, {"obj": true}], "val": null}}), json!({"arr": [1, {"obj": true}], "val": null}) ; "get complex nested structure")]
	fn test_get_action_success(key: Option<String>, input: Value, expected: Value) {
		let mut action = GetAction { key };
		let result = action.apply(input).unwrap();
		assert_eq!(result, expected);
	}

	// Error test cases
	#[test_case(Some("/nonexistent".to_string()), json!({"a": 1}) ; "nonexistent path")]
	#[test_case(Some("/nested/nonexistent".to_string()), json!({"nested": {}}) ; "nonexistent nested path")]
	#[test_case(Some("/arr/99".to_string()), json!({"arr": [1, 2, 3]}) ; "array index out of bounds")]
	#[test_case(Some("/a/b".to_string()), json!({"a": "not an object"}) ; "path through non-object")]
	#[test_case(Some("/a/0".to_string()), json!({"a": "not an array"}) ; "array index on non-array")]
	#[test_case(Some("/deeply/nested/path/that/does/not/exist".to_string()), json!({"deeply": {"nested": {}}}) ; "deeply nested nonexistent path")]
	fn test_get_action_error(key: Option<String>, input: Value) {
		let mut action = GetAction { key };
		let result = action.apply(input);
		assert!(result.is_err());
		if let Err(err) = result {
			match err {
				ConfigEditError::Action(ActionError::PointerNotFound(_)) => {}
				_ => panic!("Expected PointerNotFound error, got: {:?}", err),
			}
		}
	}
}
