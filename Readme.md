# RKNN for rust [![dependency status](https://deps.rs/repo/github/darkautism/rknn-rs/status.svg)](https://deps.rs/repo/github/darkautism/rknn-rs)

唉，為什麼又要發明輪子

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

    let mut asr_output = rknn.outputs_get_raw::<f32>()?;
    // Use data d something
    rknn.outputs_release(&mut asr_output)?; // 資料會被丟棄，不可再用asr_output

    // 或者：你很懶直接拿，這個內置release，代價是data copy
    let mut asr_output = rknn.outputs_get::<f32>()?;

    
    rknn.destroy()?;
    Ok(())
}

```

# Example project

![Sense Voice Small rknn using rust](https://github.com/darkautism/sensevoice-rs)
