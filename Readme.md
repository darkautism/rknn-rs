# RKNN for rust [![dependency status](https://deps.rs/repo/github/darkautism/rknn-rs/status.svg)](https://deps.rs/repo/github/darkautism/rknn-rs)

## Features

  * **Rusty API**: Encapsulates the C-based `rknn_api` into safe Rust structs and methods.
  * **Resource Management**: Implements resource release mechanisms to prevent memory leaks.

## Version compatibility

| Component | Version |
| --- | --- |
| rknn-rs | 0.2.1 |
| rknn-sys-rs | 0.1.1 |
| RKNN Toolkit | 2.3.2 |

## Changelog

Migrating to version 0.2.x involves API changes. Please refer to the Change log for details.

[Changelog](CHANGELOG.md)

## rknnmrt support

Just add the `rknnmrt` feature gate into your `Cargo.toml`.

## Example

```rust
use rknn_rs::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rknn = Rknn::rknn_init("/home/kautism/SenseVoiceSmall-RKNN2/sense-voice-encoder.rknn")?;
    rknn.input_set_slice(
        0,                    // Set according to your input index
        &flattened_input,     // Borrowed input slice (no extra clone)
        false,                // Usually false, unless the model requires special handling
        RknnTensorType::Float32,
        RknnTensorFormat::NCHW,
    )?;

    let asr_output = rknn.outputs_get::<f32>()?;
    // Do something with the data
    Ok(())
}
```

## LICENSE

MIT

## Contributing

Issues and Pull Requests are welcome\! If you find any missing API bindings or have ideas for better implementations, please feel free to share. Any contributions will be automatically covered under the MIT LICENSE.

## Example project

![Sense Voice Small rknn using rust](https://github.com/darkautism/sensevoice-rs)
