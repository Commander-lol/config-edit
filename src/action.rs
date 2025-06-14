use crate::error::ConfigEditError;
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use serde_json::Value;

mod action_def;
mod add;
mod get;

pub use action_def::*;

#[derive(Debug, PartialEq, Eq, Clone, Subcommand, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Action {
	Set(add::SetAction),
	Append(add::AppendAction),
	Get(get::GetAction),
}

impl ActionDef for Action {
	fn apply(&mut self, value: Value) -> Result<Value, ConfigEditError> {
		match self {
			Action::Set(action) => action.apply(value),
			Action::Append(action) => action.apply(value),
			Action::Get(action) => action.apply(value),
		}
	}
}
