use crate::action::ActionDef;
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Retrieve a value at the given path.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Parser)]
pub struct GetAction {
	/// The JSON Pointer (RFC6901) within the file to the value being retrieved.
	///
	/// Omit the key to fetch the entire document
	key: Option<String>,
}

impl ActionDef for GetAction {
	fn apply(&mut self, value: Value) -> Option<Value> {
		value.pointer(&self.key.clone().unwrap_or_default()).cloned()
	}
}
