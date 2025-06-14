use crate::action::action_def::ActionDef;
use crate::error::{ActionError, ConfigEditError};
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Set a value at the given path. Will overwrite any existing values
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Parser)]
pub struct SetAction {
	/// The JSON Pointer (RFC6901) within the file to the value being set
	pub key: String,
	/// The value to be set. Specify as a stringified json value, where invalid json will be treated as a string
	pub value: String,
}

impl ActionDef for SetAction {
	fn apply(&mut self, mut target: Value) -> Result<Value, ConfigEditError> {
		let value = match serde_json::from_str(&self.value) {
			Ok(v) => v,
			Err(_) => Value::String(self.value.clone()),
		};

		if let Some(inner) = target.pointer_mut(&self.key) {
			*inner = value;
			Ok(target)
		} else {
			Err(ActionError::PointerNotFound(self.key.clone()).into())
		}
	}
}

/// Append a value to an existing array at the given path. Will create the array if the leaf node of the path does not exist.
/// If a non-leaf node of the path does not exist, this action is a no-op
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Parser)]
pub struct AppendAction {
	/// The JSON Pointer (RFC6901) within the file to the value being set
	pub key: String,
	/// The value to be set. Specify as a stringified json value, where invalid json will be treated as a string
	pub value: String,
}

impl ActionDef for AppendAction {
	fn apply(&mut self, mut target: Value) -> Result<Value, ConfigEditError> {
		let value = match serde_json::from_str(&self.value) {
			Ok(v) => v,
			Err(_) => Value::String(self.value.clone()),
		};

		if let Some(inner) = target.pointer_mut(&self.key) {
			match inner {
				Value::Array(arr) => {
					arr.push(value);
					Ok(target)
				}
				Value::Null => {
					*inner = Value::Array(vec![value]);
					Ok(target)
				}
				_ => Err(ActionError::ApplyError(format!(
					"Cannot append to non-array value at path: {}",
					self.key
				))
				.into()),
			}
		} else {
			Err(ActionError::PointerNotFound(self.key.clone()).into())
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use serde_json::json;
	use test_case::test_case;

	// SetAction tests
	#[test_case("/a", "42", json!({"a": 1}), json!({"a": 42}) ; "set simple path with number")]
	#[test_case("/a", "\"hello\"", json!({"a": 1}), json!({"a": "hello"}) ; "set simple path with string")]
	#[test_case("/a", "true", json!({"a": 1}), json!({"a": true}) ; "set simple path with boolean")]
	#[test_case("/nested/value", "42", json!({"nested": {"value": "test"}}), json!({"nested": {"value": 42}}) ; "set nested path")]
	#[test_case("/a", "{\"key\": \"value\"}", json!({"a": 1}), json!({"a": {"key": "value"}}) ; "set with object value")]
	#[test_case("/a", "[1, 2, 3]", json!({"a": 1}), json!({"a": [1, 2, 3]}) ; "set with array value")]
	#[test_case("/a", "invalid json", json!({"a": 1}), json!({"a": "invalid json"}) ; "set with invalid json as string")]
	fn test_set_action_success(key: &str, value: &str, input: Value, expected: Value) {
		let mut action = SetAction {
			key: key.to_string(),
			value: value.to_string(),
		};
		let result = action.apply(input).unwrap();
		assert_eq!(result, expected);
	}

	#[test_case("/nonexistent", "42", json!({"a": 1}) ; "nonexistent path")]
	#[test_case("/nested/nonexistent", "42", json!({"nested": {}}) ; "nonexistent nested path")]
	fn test_set_action_error(key: &str, value: &str, input: Value) {
		let mut action = SetAction {
			key: key.to_string(),
			value: value.to_string(),
		};
		let result = action.apply(input);
		assert!(result.is_err());
		if let Err(err) = result {
			match err {
				ConfigEditError::Action(ActionError::PointerNotFound(_)) => {}
				_ => panic!("Expected PointerNotFound error, got: {:?}", err),
			}
		}
	}

	// AppendAction tests
	#[test_case("/arr", "42", json!({"arr": [1, 2, 3]}), json!({"arr": [1, 2, 3, 42]}) ; "append to existing array")]
	#[test_case("/arr", "\"hello\"", json!({"arr": [1, 2, 3]}), json!({"arr": [1, 2, 3, "hello"]}) ; "append string to array")]
	#[test_case("/nested/arr", "42", json!({"nested": {"arr": [1, 2]}}), json!({"nested": {"arr": [1, 2, 42]}}) ; "append to nested array")]
	#[test_case("/null", "42", json!({"null": null}), json!({"null": [42]}) ; "append to null creates array")]
	fn test_append_action_success(key: &str, value: &str, input: Value, expected: Value) {
		let mut action = AppendAction {
			key: key.to_string(),
			value: value.to_string(),
		};
		let result = action.apply(input).unwrap();
		assert_eq!(result, expected);
	}

	#[test_case("/nonexistent", "42", json!({"a": 1}) ; "nonexistent path")]
	#[test_case("/nested/nonexistent", "42", json!({"nested": {}}) ; "nonexistent nested path")]
	fn test_append_action_not_found_error(key: &str, value: &str, input: Value) {
		let mut action = AppendAction {
			key: key.to_string(),
			value: value.to_string(),
		};
		let result = action.apply(input);
		assert!(result.is_err());
		if let Err(err) = result {
			match err {
				ConfigEditError::Action(ActionError::PointerNotFound(_)) => {}
				_ => panic!("Expected PointerNotFound error, got: {:?}", err),
			}
		}
	}

	#[test_case("/notarray", "42", json!({"notarray": "string"}) ; "append to string")]
	#[test_case("/notarray", "42", json!({"notarray": 123}) ; "append to number")]
	#[test_case("/notarray", "42", json!({"notarray": {}}) ; "append to object")]
	fn test_append_action_type_error(key: &str, value: &str, input: Value) {
		let mut action = AppendAction {
			key: key.to_string(),
			value: value.to_string(),
		};
		let result = action.apply(input);
		assert!(result.is_err());
		if let Err(err) = result {
			match err {
				ConfigEditError::Action(ActionError::ApplyError(_)) => {}
				_ => panic!("Expected ApplyError error, got: {:?}", err),
			}
		}
	}
}
