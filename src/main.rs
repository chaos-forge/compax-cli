use clap::Parser;

mod pack;

#[derive(Parser, Debug)]
#[command(name = "compax", author, version, about = "Compress and Decompress in all format", long_about = None)]
struct Args {
    ///Directory to archive
    #[arg(short)]
    input: String,

    ///Output archive file
    #[arg(short)]
    output: String
}
fn main() {
    let args = Args::parse();

    if let Err(e) = pack::tar(&args.input, &args.output) {
        eprintln!("tar creating error {}", e);
        return;
    }

    if let Err(e) = pack::zst(&args.input, &args.output) {
        eprintln!("zst creating error {}", e);
        return;
    }

    println!("Successfully create {}", &args.output);
}