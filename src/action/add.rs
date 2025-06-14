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
