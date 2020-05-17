# Unzpack

> Persist ZIP content bytes into a file and extract it into a specific directory on file system.

**Unzpack** is just a helper which persists ZIP bytes content on file system, then extract its content into a specific directory path and finally deletes current ZIP file.

For example, it can be useful when ZIP content is provided via [include_bytes!](https://doc.rust-lang.org/std/macro.include_bytes.html) macro.

## Usage

```toml
[dependencies]
unzpack = "0.1"
```

```rust
use unzpack::Unzpack;

fn main() -> std::io::Result<()> {
    Unzpack::unpack(include_bytes!("../dist/public.zip"), "./assets.zip", "./dist")?;

    Ok(())
}
```

View [code example](./examples).

## Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in current work by you, as defined in the Apache-2.0 license, shall be dual licensed as described below, without any additional terms or conditions.

Feel free to send some [Pull request](https://github.com/joseluisq/unzpack/pulls) or [issue](https://github.com/joseluisq/unzpack/issues).

## License

This work is primarily distributed under the terms of both the [MIT license](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE).

© 2020 [Jose Quintana](https://git.io/joseluisq)
