use std::fs::File;
use std::io;
use tar::Builder;
use zstd::stream::Encoder;

pub fn tar(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(input)?;
    let mut tar = Builder::new(file);
    tar.append_dir_all("", output)?;
    tar.finish()?;
    Ok(())
}

pub fn zst(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file_input = File::open(input)?;
    let file_output = File::create(output)?;
    let mut encode = Encoder::new(file_output, 19)?;
    io::copy(&mut file_input, &mut encode)?;
    encode.finish()?;
    Ok(())
}
