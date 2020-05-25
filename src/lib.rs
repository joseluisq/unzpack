use std::error;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

/// A library to persist ZIP content bytes into a file and extract its content on file system.
///
/// Unzpack is just a helper library which persists ZIP bytes content on file system, then extract its content into a specific directory path and finally deletes current ZIP file.
/// For example, it can be useful when ZIP content is provided via [`include_bytes!`](https://doc.rust-lang.org/std/macro.include_bytes.html) macro.
pub struct Unzpack;

impl Unzpack {
    /// Persists Zip content bytes into a `.zip` file.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use unzpack::Unzpack;
    ///
    /// const ZIP_BYTES: &[u8] = include_bytes!("data/assets.zip");
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     Unzpack::persist(ZIP_BYTES, "/tmp/out-file.zip")?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn persist<P: AsRef<Path>>(
        zip_bytes: &'static [u8],
        filepath: P,
    ) -> Result<(), Box<dyn error::Error>> {
        let mut file = fs::File::create(filepath)?;
        file.write_all(zip_bytes)?;
        Ok(())
    }

    /// Extracts a Zip file into a specific directory. Output directory will be created if doesn't exist.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use unzpack::Unzpack;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     Unzpack::extract("/tmp/src-file.zip", "/tmp/out-dir")?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn extract<P: AsRef<Path>>(filepath: P, outdir: P) -> Result<(), Box<dyn error::Error>> {
        let filepath = filepath.as_ref().canonicalize()?;

        let file = fs::File::open(filepath)?;
        let mut archive = zip::ZipArchive::new(file).unwrap();

        let outdir = outdir.as_ref();

        // Create output directory if doesn't exist
        if !outdir.exists() {
            fs::create_dir_all(&outdir)?;
        }

        let outdir = outdir.canonicalize()?;

        // Check if output path is a directory
        if !outdir.is_dir() {
            return Result::Err(From::from(format!(
                "path \"{:?}\" is not a directory",
                &outdir
            )));
        }

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

    /// Persists and extracts Zip content into specific file system directory.
    ///
    /// It persists Zip bytes content into file system as a `.zip` file,
    /// then extract it into a specific directory path and finally the Zip file is deleted.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use unzpack::Unzpack;
    ///
    /// const ZIP_BYTES: &[u8] = include_bytes!("data/assets.zip");
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     Unzpack::unpack(
    ///         ZIP_BYTES,          // Zip bytes
    ///         "./out-file.zip",   // Output Zip file
    ///         "./out-dir",        // Output extraction directory
    ///     )?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn unpack<P: AsRef<Path>>(
        zip_bytes: &'static [u8],
        out_filepath: P,
        out_dirpath: P,
    ) -> Result<(), Box<dyn error::Error>> {
        Unzpack::persist(zip_bytes, out_filepath.as_ref())?;
        Unzpack::extract(out_filepath.as_ref(), out_dirpath.as_ref())?;
        fs::remove_file(out_filepath)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ZIP_BYTES: &[u8] = include_bytes!("/tmp/static-web-server.zip");

    #[test]
    fn check_zip_file() {
        let filepath = Path::new("/tmp/static-web-server.zip");
        assert_eq!(filepath.exists(), true);
        assert_eq!(filepath.is_file(), true);
    }

    #[test]
    fn test_persist() -> Result<(), Box<dyn error::Error>> {
        let out_filepath = Path::new("/tmp/static-web-server.persist.zip");

        Unzpack::persist(ZIP_BYTES, out_filepath)?;

        assert_eq!(out_filepath.exists(), true);
        assert_eq!(out_filepath.is_file(), true);

        Ok(())
    }

    #[test]
    fn test_extract() -> Result<(), Box<dyn error::Error>> {
        let out_filepath = Path::new("/tmp/static-web-server.persist.zip");

        Unzpack::persist(ZIP_BYTES, out_filepath)?;

        assert_eq!(out_filepath.exists(), true);
        assert_eq!(out_filepath.is_file(), true);

        let out_dirpath = Path::new("/tmp/static-web-server.extract");

        Unzpack::extract(out_filepath, out_dirpath)?;

        assert_eq!(out_dirpath.exists(), true);
        assert_eq!(out_dirpath.is_dir(), true);

        Ok(())
    }

    #[test]
    fn test_unpack() -> Result<(), Box<dyn error::Error>> {
        let out_filepath = Path::new("/tmp/static-web-server.unpack.zip");
        let out_dirpath = Path::new("/tmp/static-web-server.unpack");

        Unzpack::unpack(ZIP_BYTES, out_filepath, out_dirpath)?;

        assert_eq!(out_dirpath.exists(), true);
        assert_eq!(out_dirpath.is_dir(), true);

        let out_filepath_content = out_dirpath.join("static-web-server-1.9.0");
        assert_eq!(out_filepath_content.exists(), true);
        assert_eq!(out_filepath_content.is_dir(), true);

        Ok(())
    }
}
