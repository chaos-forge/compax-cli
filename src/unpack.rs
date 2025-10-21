use std::fs::File;
use std::path::Path;
use flate2::read::GzDecoder;
use xz2::read::XzDecoder;
use zstd::stream::Decoder;
use bzip2::read::BzDecoder;
use tar::Archive;
use std::error::Error;
use sevenz_rust::decompress_file;

#[derive(Debug, Clone, Copy)]
pub enum DeCompressionFormat {
    Xz,
    Gz,
    Zst,
    Bz2,
    SevenZ,
}

impl DeCompressionFormat {
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

    pub fn decompress(&self, input: &str, output: &str) -> Result<(), Box<dyn Error>> {
        match self {
            Self::Xz => xz_decompress(input, output),
            Self::Gz => gz_decompress(input, output),
            Self::Zst => zst_decompress(input, output),
            Self::Bz2 => bz2_decompress(input, output),
            Self::SevenZ => sevenz_decompress(input, output),
        }
    }
}

fn gz_decompress(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    let file_input = File::open(input)?;
    let gz_decoder = GzDecoder::new(file_input);
    let mut archive = Archive::new(gz_decoder);
    archive.unpack(output)?;
    Ok(())
}

fn xz_decompress(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    let file_input = File::open(input)?;
    let xz_decoder = XzDecoder::new(file_input);
    let mut archive = Archive::new(xz_decoder);
    archive.unpack(output)?;
    Ok(())
}

fn zst_decompress(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    let file_input = File::open(input)?;
    let zst_decoder = Decoder::new(file_input)?;
    let mut archive = Archive::new(zst_decoder);
    archive.unpack(output)?;
    Ok(())
}

fn bz2_decompress(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    let file_input = File::open(input)?;
    let bzdecoder = BzDecoder::new(file_input);
    let mut archive = Archive::new(bzdecoder);
    archive.unpack(output)?;
    Ok(())
}

fn sevenz_decompress(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    decompress_file(input, output)?;
    Ok(())
}

/// Automatically detect compression format and decompress
pub fn decompress_any(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(input);
    let extension = path.extension()
        .and_then(|ext| ext.to_str())
        .ok_or("Unable to determine file extension")?;

    let format = DeCompressionFormat::from_extension(extension)
        .ok_or_else(|| format!("Unsupported compression format: {}", extension))?;

    format.decompress(input, output)
}