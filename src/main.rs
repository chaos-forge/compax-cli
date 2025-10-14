use clap::Parser;
use std::path::Path;

mod pack;

#[derive(Parser, Debug)]
#[command(name = "compax", author, version, about = "Compress and Decompress in all format", long_about = None)]
struct Args {
    /// Directory to archive
    #[arg(short)]
    input: String,

    /// Output archive file
    #[arg(short)]
    output: String,
}

fn main() {
    let args = Args::parse();

    // Determine compression format based on file extension
    let result = match Path::new(&args.output)
        .extension()
        .and_then(|ext| ext.to_str())
    {
        Some("xz") | Some("txz") => pack::xz(&args.input, &args.output),
        Some("zst") | Some("tzst") => pack::zst(&args.input, &args.output),
        Some("gz") | Some("tgz") => pack::gz(&args.input, &args.output),
        Some("bz2") | Some("tbz2") => pack::bz2(&args.input, &args.output),
        Some("7z") => pack::sevenz(&args.input, &args.output),
        Some(ext) => {
            eprintln!("Unsupported compression format: {}", ext);
            return;
        }
        None => {
            eprintln!("Output file must have a valid extension (e.g., .tar.xz, .tar.gz, .tar.zst, .tar.bz2)");
            return;
        }
    };

    match result {
        Ok(()) => println!("Successfully created {}", &args.output),
        Err(e) => eprintln!("Error creating archive: {}", e),
    }
}