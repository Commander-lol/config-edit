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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_action_set() {
        let mut action = Action::Set(add::SetAction {
            key: "/a".to_string(),
            value: "42".to_string(),
        });
        let input = json!({"a": 1});
        let expected = json!({"a": 42});
        let result = action.apply(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_action_append() {
        let mut action = Action::Append(add::AppendAction {
            key: "/arr".to_string(),
            value: "42".to_string(),
        });
        let input = json!({"arr": [1, 2, 3]});
        let expected = json!({"arr": [1, 2, 3, 42]});
        let result = action.apply(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_action_get() {
        let mut action = Action::Get(get::GetAction::new("/a".to_string()));
        let input = json!({"a": 1});
        let expected = json!(1);
        let result = action.apply(input).unwrap();
        assert_eq!(result, expected);
    }
}
