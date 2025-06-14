use std::io::{self, Write};
use crate::action::{ActionDef, read_file};
use clap::{Args, Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

mod action;

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
	let mut opts = Cli::parse();
	let input_format = determine_format(Some(&opts.input), opts.input_format).expect("Failed to determine input format");
	let output_format = determine_format(opts.output.as_ref(), opts.output_format).unwrap_or(input_format);

	let input_value = read_file(&opts.input, input_format).expect("Failed to read input file");
	let output_value = opts.action.apply(input_value).expect("Failed to apply action");

	let mut stream: Box<dyn io::Write> = match (opts.write, &opts.output) {
		(true, _) => {
			Box::new(std::fs::File::create(&opts.input).expect("Failed to open input file for writing"))
		},
		(false, Some(output_path)) => {
			Box::new(std::fs::File::create(output_path).expect("Failed to open output file for writing"))
		},
		(false, None) => {
			Box::new(io::stdout())
		},
	};

	match output_format {
		SupportedFormats::Toml => {
			let toml_string = toml::to_string(&output_value).expect("Failed to serialize to TOML");
			write!(stream, "{}", toml_string).expect("Failed to write TOML to output");
		},
		SupportedFormats::Yaml => {
			serde_yaml::to_writer(stream, &output_value).expect("Failed to serialize to YAML");
		},
		SupportedFormats::Json => {
			serde_json::to_writer_pretty(stream, &output_value).expect("Failed to serialize to JSON");
		},
		SupportedFormats::Plist => {
			plist::to_writer_xml(stream, &output_value).expect("Failed to serialize to Plist");
		},
	}
}

fn determine_format(path: Option<&PathBuf>, explicit_format: Option<SupportedFormats>) -> Option<SupportedFormats> {
	match (path, explicit_format) {
        (Some(path), _) => {
            match path.extension() {
                Some(ext) if ext == "toml" => Some(SupportedFormats::Toml),
                Some(ext) if ext == "yaml" => Some(SupportedFormats::Yaml),
                Some(ext) if ext == "yml" => Some(SupportedFormats::Yaml),
                Some(ext) if ext == "json" => Some(SupportedFormats::Json),
                Some(ext) if ext == "plist" => Some(SupportedFormats::Plist),
                _ => explicit_format,
            }
        }
        (_, Some(format)) => Some(format),
        _ => None,
    }
}
