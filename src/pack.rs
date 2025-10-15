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

#[derive(Debug, Clone, Copy)]
pub enum CompressionFormat {
    Xz,
    Gz,
    Zst,
    Bz2,
    SevenZ,
}

impl CompressionFormat {
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "xz" | "txz" => Some(Self::Xz),
            "gz" | "tgz" => Some(Self::Gz),
            "zst" | "tzst" => Some(Self::Zst),
            "bz2" | "tbz2" => Some(Self::Bz2),
            "7z" => Some(Self::SevenZ),
            _ => None,
        }
    }

    pub fn compress(&self, input: &str, output: &str) -> Result<(), Box<dyn Error>> {
        match self {
            Self::Xz => xz_compress(input, output),
            Self::Gz => gz_compress(input, output),
            Self::Zst => zst_compress(input, output),
            Self::Bz2 => bz2_compress(input, output),
            Self::SevenZ => sevenz_compress(input, output),
        }
    }

    pub fn default_extension(&self) -> &'static str {
        match self {
            Self::Xz => "tar.xz",
            Self::Gz => "tar.gz",
            Self::Zst => "tar.zst",
            Self::Bz2 => "tar.bz2",
            Self::SevenZ => "7z",
        }
    }
}

// Private implementation functions
fn xz_compress(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    let file_output = File::create(output)?;
    let xz_encode = XzEncoder::new(file_output, 9);
    let mut tar = Builder::new(xz_encode);
    tar.append_dir_all("", input)?;
    let xz_encode = tar.into_inner()?;
    xz_encode.finish()?;
    Ok(())
}

fn gz_compress(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    let file_output = File::create(output)?;
    let gz_encode = GzEncoder::new(file_output, GzCompression::best());
    let mut tar = Builder::new(gz_encode);
    tar.append_dir_all("", input)?;
    let gz_encode = tar.into_inner()?;
    gz_encode.finish()?;
    Ok(())
}

fn zst_compress(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    let file_output = File::create(output)?;
    let encode = Encoder::new(file_output, 19)?;
    let mut tar = Builder::new(encode);
    tar.append_dir_all("", input)?;
    let encode = tar.into_inner()?;
    encode.finish()?;
    Ok(())
}

fn bz2_compress(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    let file_output = File::create(output)?;
    let bz2_encode = BzEncoder::new(file_output, BzCompression::best());
    let mut tar = Builder::new(bz2_encode);
    tar.append_dir_all("", input)?;
    let bz2_encode = tar.into_inner()?;
    bz2_encode.finish()?;
    Ok(())
}

fn sevenz_compress(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    let mut sz = SevenZWriter::create(output)?;
    sz.push_source_path(Path::new(input), |_| true)?;
    sz.finish()?;
    Ok(())
}

// Public API 
pub fn xz(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    CompressionFormat::Xz.compress(input, output)
}

pub fn gz(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    CompressionFormat::Gz.compress(input, output)
}

pub fn zst(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    CompressionFormat::Zst.compress(input, output)
}

pub fn bz2(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    CompressionFormat::Bz2.compress(input, output)
}

pub fn sevenz(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    CompressionFormat::SevenZ.compress(input, output)
}