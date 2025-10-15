use clap::{Arg, Command};
use std::error::Error;
use std::path::Path;

mod pack;
use pack::{CompressionFormat, xz, gz, zst, bz2, sevenz};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("compax")
        .version("0.1.0")
        .author("JP-Makers, Magiyaazh")
        .about("File compression tool")
        .subcommand(
            Command::new("c")
                .about("Compress a directory")
                .arg(
                    Arg::new("input")
                        .help("Input directory to compress")
                        .required(true)
                        .index(1)
                )
                .arg(
                    Arg::new("output")
                        .help("Output file or format")
                        .required(true)
                        .index(2)
                )
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("c") {
        let input = matches.get_one::<String>("input").unwrap();
        let output = matches.get_one::<String>("output").unwrap();
        
        // Check if input directory exists
        if !Path::new(input).exists() {
            return Err(format!("Input directory '{}' does not exist", input).into());
        }

        // Determine output path and format
        let (output_path, format) = if output.contains('.') {
            // If output has an extension, use it to determine format
            let path = Path::new(output);
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if let Some(fmt) = CompressionFormat::from_extension(ext) {
                    (output.to_string(), fmt)
                } else {
                    return Err(format!("Unsupported file extension: {}", ext).into());
                }
            } else {
                return Err("Output file must have an extension".into());
            }
        } else {
            // If output is just a format name, generate output filename
            let format = CompressionFormat::from_extension(output)
                .ok_or_else(|| format!("Unsupported compression format: {}", output))?;
            
            let input_path = Path::new(input);
            let dir_name = input_path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("archive");
            
            let output_filename = format!("{}.{}", dir_name, format.default_extension());
            (output_filename, format)
        };

        println!("Compressing '{}' to '{}' using {:?}", input, output_path, format);
        
        match format {
            CompressionFormat::Xz => xz(input, &output_path)?,
            CompressionFormat::Gz => gz(input, &output_path)?,
            CompressionFormat::Zst => zst(input, &output_path)?,
            CompressionFormat::Bz2 => bz2(input, &output_path)?,
            CompressionFormat::SevenZ => sevenz(input, &output_path)?,
        }

        println!("Successfully created '{}'", output_path);
    }

    Ok(())
}