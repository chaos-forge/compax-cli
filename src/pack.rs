use std::fs::File;
use tar::Builder;
use zstd::stream::Encoder;

pub fn tar(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(output)?;
    let mut tar = Builder::new(file);
    tar.append_dir_all("", input)?;
    tar.finish()?;
    Ok(())
}

pub fn zst(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file_output = File::create(output)?;
    let encode = Encoder::new(file_output, 19)?;
    let mut tar = Builder::new(encode);
    tar.append_dir_all("", input)?;
    let encode = tar.into_inner()?;
    encode.finish()?;
    Ok(())
}
