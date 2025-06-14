use crate::action::{ActionDef, read_file};
use crate::error::{ConfigEditError, FormatError, SerializationError};
use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::path::PathBuf;
use std::process;

mod action;
mod error;

/// Perform simple edits to toml, yaml, and json files
#[derive(Parser, Serialize, Deserialize, Debug, Clone)]
#[clap(name = "config-edit")]
#[clap(author = "Louis Capitanchik <louis@microhacks.co.uk>")]
#[clap(version = "0.1.0")]
#[clap(about, long_about = None)]
pub struct Cli {
	#[command(subcommand)]
	action: action::Action,
	/// Override file extensions detection for the output file. Only applies when writing to a different
	/// file path
	#[clap(long = "of")]
	output_format: Option<SupportedFormats>,
	/// Override file extensions detection for the input file
	#[clap(long = "if")]
	input_format: Option<SupportedFormats>,
	/// Modify the input file in place. Ignored when specifying an output file
	#[clap(short, long, default_value_t = false)]
	write: bool,
	/// Path to the file being modified
	#[clap(short, long)]
	input: PathBuf,
	/// Write output to a different file path
	#[clap(short, long)]
	output: Option<PathBuf>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum SupportedFormats {
	Toml,
	Yaml,
	Json,
	Plist,
}

fn main() {
	if let Err(err) = run() {
		eprintln!("Error: {}", err);
		process::exit(1);
	}
}

fn run() -> Result<(), ConfigEditError> {
	let mut opts = Cli::parse();
	let input_format = determine_format(Some(&opts.input), opts.input_format)?;
	let output_format =
		determine_format(opts.output.as_ref(), opts.output_format).unwrap_or(input_format);

	let input_value = read_file(&opts.input, input_format)?;
	let output_value = opts.action.apply(input_value)?;

	let mut stream: Box<dyn io::Write> = match (opts.write, &opts.output) {
		(true, _) => Box::new(std::fs::File::create(&opts.input)?),
		(false, Some(output_path)) => Box::new(std::fs::File::create(output_path)?),
		(false, None) => Box::new(io::stdout()),
	};

	match output_format {
		SupportedFormats::Toml => {
			let toml_string = toml::to_string(&output_value).map_err(|err| {
				SerializationError::SerializeError {
					format: "TOML".to_string(),
					source: Box::new(err),
				}
			})?;
			write!(stream, "{}", toml_string).map_err(ConfigEditError::Io)?;
		}
		SupportedFormats::Yaml => {
			serde_yaml::to_writer(stream, &output_value).map_err(|err| {
				SerializationError::SerializeError {
					format: "YAML".to_string(),
					source: Box::new(err),
				}
			})?;
		}
		SupportedFormats::Json => {
			serde_json::to_writer_pretty(stream, &output_value).map_err(|err| {
				SerializationError::SerializeError {
					format: "JSON".to_string(),
					source: Box::new(err),
				}
			})?;
		}
		SupportedFormats::Plist => {
			plist::to_writer_xml(stream, &output_value).map_err(|err| {
				SerializationError::SerializeError {
					format: "Plist".to_string(),
					source: Box::new(err),
				}
			})?;
		}
	}

	Ok(())
}

fn determine_format(
	path: Option<&PathBuf>,
	explicit_format: Option<SupportedFormats>,
) -> Result<SupportedFormats, ConfigEditError> {
	match (path, explicit_format) {
		(Some(path), _) => match path.extension() {
			Some(ext) if ext == "toml" => Ok(SupportedFormats::Toml),
			Some(ext) if ext == "yaml" => Ok(SupportedFormats::Yaml),
			Some(ext) if ext == "yml" => Ok(SupportedFormats::Yaml),
			Some(ext) if ext == "json" => Ok(SupportedFormats::Json),
			Some(ext) if ext == "plist" => Ok(SupportedFormats::Plist),
			_ => explicit_format
				.ok_or_else(|| FormatError::UnknownFormat { path: path.clone() }.into()),
		},
		(_, Some(format)) => Ok(format),
		_ => Err(FormatError::UnknownFormat {
			path: PathBuf::from("<unknown>"),
		}
		.into()),
	}
}
