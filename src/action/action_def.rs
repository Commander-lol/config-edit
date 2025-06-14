use crate::SupportedFormats;
use std::path::PathBuf;

pub trait ActionDef {
	fn apply(&mut self, value: serde_json::Value) -> Option<serde_json::Value>;
}

pub fn read_file(
	input_path: &PathBuf,
	input_format: SupportedFormats,
) -> Option<serde_json::Value> {
	let file_content = std::fs::read_to_string(input_path).expect("Failed to read file");
	let mut json_content = Vec::with_capacity(file_content.len());

	match input_format {
		SupportedFormats::Toml => {
			let de = toml::Deserializer::new(&file_content);
			serde_transcode::transcode(de, &mut serde_json::Serializer::new(&mut json_content))
				.expect("Failed to transcode");
		}
		SupportedFormats::Yaml => {
			let de = toml::Deserializer::new(&file_content);
			serde_transcode::transcode(de, &mut serde_json::Serializer::new(&mut json_content))
				.expect("Failed to transcode");
		}
		SupportedFormats::Plist => {
			let value: plist::Value =
				plist::from_bytes(file_content.as_bytes()).expect("Failed to parse Plist");
			json_content = serde_json::to_vec(&value).expect("Failed to serialize Plist to JSON");
		}
		SupportedFormats::Json => {
			json_content = file_content.into_bytes();
		}
	}

	serde_json::from_slice(&json_content).ok()
}
