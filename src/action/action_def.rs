use crate::SupportedFormats;
use crate::error::{ConfigEditError, FormatError, SerializationError};
use std::path::PathBuf;

pub trait ActionDef {
	fn apply(&mut self, value: serde_json::Value) -> Result<serde_json::Value, ConfigEditError>;
}

pub fn read_file(
	input_path: &PathBuf,
	input_format: SupportedFormats,
) -> Result<serde_json::Value, ConfigEditError> {
	let file_content = std::fs::read_to_string(input_path)?;
	let mut json_content = Vec::with_capacity(file_content.len());

	match input_format {
		SupportedFormats::Toml => {
			let de = toml::Deserializer::new(&file_content);
			serde_transcode::transcode(de, &mut serde_json::Serializer::new(&mut json_content))
				.map_err(|err| FormatError::TranscodeError {
					from: "TOML".to_string(),
					to: "JSON".to_string(),
					source: Box::new(err),
				})?;
		}
		SupportedFormats::Yaml => {
			let de = serde_yaml::Deserializer::from_str(&file_content);
			serde_transcode::transcode(de, &mut serde_json::Serializer::new(&mut json_content))
				.map_err(|err| FormatError::TranscodeError {
					from: "YAML".to_string(),
					to: "JSON".to_string(),
					source: Box::new(err),
				})?;
		}
		SupportedFormats::Plist => {
			let value: plist::Value =
				plist::from_bytes(file_content.as_bytes()).map_err(|err| {
					SerializationError::DeserializeError {
						format: "Plist".to_string(),
						source: Box::new(err),
					}
				})?;
			json_content =
				serde_json::to_vec(&value).map_err(|err| SerializationError::SerializeError {
					format: "JSON".to_string(),
					source: Box::new(err),
				})?;
		}
		SupportedFormats::Json => {
			json_content = file_content.into_bytes();
		}
	}

	serde_json::from_slice(&json_content).map_err(|err| {
		SerializationError::DeserializeError {
			format: "JSON".to_string(),
			source: Box::new(err),
		}
		.into()
	})
}
