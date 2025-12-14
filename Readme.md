# RKNN for rust [![dependency status](https://deps.rs/repo/github/darkautism/rknn-rs/status.svg)](https://deps.rs/repo/github/darkautism/rknn-rs)

唉，為什麼又要發明輪子

遷移至 0.2.0會遇到API變更，詳情請見Change log

[Changelog](CHANGELOG.md)

# rknnmrt support

Just add rknnmrt feature gate into your Cargo.toml.

# Example

``` Rust
use rknn_rs::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rknn = Rknn::rknn_init("/home/kautism/SenseVoiceSmall-RKNN2/sense-voice-encoder.rknn")?;
    rknn.input_set(&mut RknnInput {
        index: 0,             // 根據您的輸入索引設定
        buf: flattened_input, /* 您的數據 */
        pass_through: false,  // 通常設為 false，除非模型需要特殊處理
        type_: RknnTensorType::Float32,
        fmt: RknnTensorFormat::NCHW,
    })?;

    let mut asr_output = rknn.outputs_get::<f32>()?;
    // Use data d something
    Ok(())
}

```

# Example project

![Sense Voice Small rknn using rust](https://github.com/darkautism/sensevoice-rs)
