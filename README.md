# 🔑 Credent

[![Crates.io](https://img.shields.io/crates/v/credent.svg)](https://crates.io/crates/credent)
[![docs.rs](https://img.shields.io/docsrs/credent)](https://docs.rs/credent)
[![CI](https://github.com/azriel91/credent/workflows/CI/badge.svg)](https://github.com/azriel91/credent/actions/workflows/ci.yml)
[![Coverage Status](https://codecov.io/gh/azriel91/credent/branch/main/graph/badge.svg)](https://codecov.io/gh/azriel91/credent)

Manages `~/.config/<app>/credentials`.

![](demo.png)

## Usage

Add the following to Cargo.toml:

```toml
credent = { version = "0.4.1", features = ["backend-smol"] } # or "backend-tokio"
```

Example code:

```rust
use credent::{
    cli::CredentialsCliReader,
    fs::{model::AppName, CredentialsFile, CredentialsFileStorer},
    model::Credentials,
};

/// Application name
const CREDENT: AppName<'_> = AppName("credent");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    smol::run(async {
        let credentials = CredentialsCliReader::<Credentials>::read_from_tty().await?;
        println!("credentials: {}", credentials);

        CredentialsFileStorer::<Credentials>::store(CREDENT, &credentials).await?;

        println!(
            "credentials written to: {}",
            CredentialsFile::<Credentials>::path(CREDENT)?.display()
        );

        Result::<(), Box<dyn std::error::Error>>::Ok(())
    })
}
```

More examples can be seen in the [examples](examples).

```bash
# Use either "backend-smol" or "backend-tokio"
cargo run --features "backend-smol" --example simple
cargo run --features "backend-smol" --example demo
cargo run --features "backend-smol" --example profiles
cargo run --features "backend-smol" --example profiles -- --profile development
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
