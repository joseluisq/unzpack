use unzpack::Unzpack;

fn main() -> std::io::Result<()> {
    Unzpack::unpack(include_bytes!("../dist/public.zip"), "./assets.zip", "./dist")?;

    Ok(())
}
