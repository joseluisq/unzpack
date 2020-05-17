use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

/// Persist ZIP content bytes into a file and extract it into a specific directory on file system.
///
/// For example, zip bytes content can be provided using `include_bytes!` macro.
pub struct Unzpack;

impl Unzpack {
    /// Persists zip content into a zip file on file system.
    pub fn persist<P: AsRef<Path>>(zip_bytes: &'static [u8], filepath: P) -> std::io::Result<()> {
        let mut file = File::create(filepath)?;
        file.write_all(zip_bytes)?;
        Ok(())
    }

    /// Extracts a zip file into a specific directory.
    pub fn extract<P: AsRef<Path>>(filepath: P, outdir: P) -> std::io::Result<()> {
        let filepath = filepath.as_ref().canonicalize()?;
        let outdir = outdir.as_ref().canonicalize()?;
        let outdir = Path::new(&outdir).to_path_buf();

        let file = fs::File::open(filepath).unwrap();
        let mut archive = zip::ZipArchive::new(file).unwrap();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let outpath = outdir.join(file.sanitized_name());

            if (&*file.name()).ends_with('/') {
                fs::create_dir_all(&outpath).unwrap();
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(&p).unwrap();
                    }
                }

                let mut outfile = fs::File::create(&outpath).unwrap();
                io::copy(&mut file, &mut outfile).unwrap();
            }
        }

        Ok(())
    }

    /// Persists and extracts zip content into specific file system directory.
    ///
    /// It persists zip bytes content into file system as a zip file,
    /// then extract it into a specific directory path and finally the zip file is deleted.
    pub fn unpack<P: AsRef<Path>>(
        zip_bytes: &'static [u8],
        out_filepath: P,
        out_dirpath: P,
    ) -> std::io::Result<()> {
        Unzpack::persist(zip_bytes, out_filepath.as_ref())?;
        Unzpack::extract(out_filepath.as_ref(), out_dirpath.as_ref())?;
        fs::remove_file(out_filepath)?;

        Ok(())
    }
}
