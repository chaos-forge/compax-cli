use std::fs::File;
use std::path::Path;
use std::error::Error;
use tar::Builder;
use zstd::stream::Encoder;
use xz2::write::XzEncoder;
use flate2::write::GzEncoder;
use flate2::Compression as GzCompression; 
use bzip2::write::BzEncoder;
use bzip2::Compression as BzCompression; 
use sevenz_rust::SevenZWriter;



pub fn xz(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    let file_output = File::create(output)?;
    let xzencode = XzEncoder::new(file_output, 9);
    let mut tar = Builder::new(xzencode);
    tar.append_dir_all("", input)?;
    let xzencode = tar.into_inner()?;
    xzencode.finish()?;
    Ok(())
}

pub fn zst(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    let file_output = File::create(output)?;
    let encode = Encoder::new(file_output, 22)?;
    let mut tar = Builder::new(encode);
    tar.append_dir_all("", input)?;
    let encode = tar.into_inner()?;
    encode.finish()?;
    Ok(())
}

pub fn gz(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    let file_output = File::create(output)?;
    let gzencode = GzEncoder::new(file_output, GzCompression::best());
    let mut tar = Builder::new(gzencode);
    tar.append_dir_all("", input)?;
    let gzencode = tar.into_inner()?;
    gzencode.finish()?;
    Ok(())
}

pub fn bz2(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    let file_output = File::create(output)?;
    let bz2encode = BzEncoder::new(file_output, BzCompression::best());
    let mut tar = Builder::new(bz2encode);
    tar.append_dir_all("", input)?;
    let bz2encode = tar.into_inner()?;
    bz2encode.finish()?;
    Ok(())
}

pub fn sevenz(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    let mut sz = SevenZWriter::create(output)?;
    sz.push_source_path(Path::new(input), |_| true)?;
    sz.finish()?;
    Ok(())
}