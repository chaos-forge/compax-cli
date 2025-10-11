use std::fs::File;

use tar::Builder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create("foo.tar")?;
    let mut tar = Builder::new(file);
    tar.append_dir_all("", "./src")?;
    tar.finish()?;
    
    Ok(())
}
