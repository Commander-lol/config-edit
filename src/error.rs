use std::io;
use std::path::PathBuf;
use thiserror::Error;

/// Error type for the config-edit application
#[derive(Error, Debug)]
pub enum ConfigEditError {
	#[error("{0}")]
	Io(#[from] io::Error),

	#[error("{0}")]
	Format(#[from] FormatError),

	#[error("{0}")]
	Action(#[from] ActionError),

	#[error("{0}")]
	Serialization(#[from] SerializationError),
}

/// Error type for format-related errors
#[derive(Error, Debug)]
pub enum FormatError {
	#[error("Failed to determine format for file: {path:?}")]
	UnknownFormat { path: PathBuf },

	#[error("Failed to transcode from {from} to {to}: {source}")]
	TranscodeError {
		from: String,
		to: String,
		source: Box<dyn std::error::Error + Send + Sync>,
	},
}

/// Error type for action-related errors
#[derive(Error, Debug)]
pub enum ActionError {
	#[error("Failed to apply action: {0}")]
	ApplyError(String),

	#[error("JSON pointer not found: {0}")]
	PointerNotFound(String),
}

/// Error type for serialization-related errors
#[derive(Error, Debug)]
pub enum SerializationError {
	#[error("Failed to serialize to {format}: {source}")]
	SerializeError {
		format: String,
		source: Box<dyn std::error::Error + Send + Sync>,
	},

	#[error("Failed to deserialize from {format}: {source}")]
	DeserializeError {
		format: String,
		source: Box<dyn std::error::Error + Send + Sync>,
	},
}

// Implement From traits for common error conversions
impl From<serde_json::Error> for SerializationError {
	fn from(err: serde_json::Error) -> Self {
		SerializationError::DeserializeError {
			format: "JSON".to_string(),
			source: Box::new(err),
		}
	}
}

impl From<toml::de::Error> for SerializationError {
	fn from(err: toml::de::Error) -> Self {
		SerializationError::DeserializeError {
			format: "TOML".to_string(),
			source: Box::new(err),
		}
	}
}

impl From<toml::ser::Error> for SerializationError {
	fn from(err: toml::ser::Error) -> Self {
		SerializationError::SerializeError {
			format: "TOML".to_string(),
			source: Box::new(err),
		}
	}
}

impl From<serde_yaml::Error> for SerializationError {
	fn from(err: serde_yaml::Error) -> Self {
		// Can't determine if it's serialization or deserialization, so use a generic message
		SerializationError::SerializeError {
			format: "YAML".to_string(),
			source: Box::new(err),
		}
	}
}

impl From<plist::Error> for SerializationError {
	fn from(err: plist::Error) -> Self {
		// Can't determine if it's serialization or deserialization, so use a generic message
		SerializationError::SerializeError {
			format: "Plist".to_string(),
			source: Box::new(err),
		}
	}
}
