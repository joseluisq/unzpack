use unzpack::Unzpack;

const BYTES: &[u8] = include_bytes!("../dist/public.zip");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Unzpack::unpack(BYTES, "./assets.zip", "./dist")?;

    Ok(())
}
