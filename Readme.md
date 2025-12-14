# RKNN for rust [![dependency status](https://deps.rs/repo/github/darkautism/rknn-rs/status.svg)](https://deps.rs/repo/github/darkautism/rknn-rs)

## Features

  * **Rusty API**: Encapsulates the C-based `rknn_api` into safe Rust structs and methods.
  * **Resource Management**: Implements resource release mechanisms to prevent memory leaks.

## Changelog

Migrating to version 0.2.0 involves API changes. Please refer to the Change log for details.

[Changelog](CHANGELOG.md)

## rknnmrt support

Just add the `rknnmrt` feature gate into your `Cargo.toml`.

## Example

```rust
use rknn_rs::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rknn = Rknn::rknn_init("/home/kautism/SenseVoiceSmall-RKNN2/sense-voice-encoder.rknn")?;
    rknn.input_set(&mut RknnInput {
        index: 0,                     // Set according to your input index
        buf: flattened_input,         /* Your data */
        pass_through: false,          // Usually false, unless the model requires special handling
        type_: RknnTensorType::Float32,
        fmt: RknnTensorFormat::NCHW,
    })?;

    let mut asr_output = rknn.outputs_get::<f32>()?;
    // Do something with the data
    Ok(())
}
```

## LICENSE

MIT

## Contributing

Issues and Pull Requests are welcome\! If you find any missing API bindings or have ideas for better implementations, please feel free to share. Any contributions will be automatically covered under the MIT LICENSE.
