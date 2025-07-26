#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// Prelude module for RKNN (Rockchip Neural Network) related functionality.
///
/// This module contains commonly used types and functions for interacting with RKNN, making it convenient for use in other modules.
pub mod prelude {
    use std::{
        ffi::{CStr, CString}, mem::{self, ManuallyDrop}, os::raw::c_void, ptr::null_mut, rc::Weak, slice, str, sync::Arc
    };

    use bytemuck::Pod;

    /// RKNN tensor attributes.
    ///
    /// This struct describes the attributes of a tensor in an RKNN model, including dimensions, name, type, etc.
    #[derive(Debug, Copy, Clone)]
    pub struct _rknn_tensor_attr {
        /// Tensor index.
        pub index: u32,
        /// Number of dimensions.
        pub n_dims: u32,
        /// Tensor dimensions.
        pub dims: [u32; 16usize],
        /// Tensor name.
        pub name: [::std::os::raw::c_char; 256usize],
        /// Number of elements in the tensor.
        pub n_elems: u32,
        /// Size of the tensor in bytes.
        pub size: u32,
        /// Tensor format.
        pub fmt: super::rknn_tensor_format,
        /// Tensor type.
        pub type_: super::rknn_tensor_type,
        /// Tensor quantization type.
        pub qnt_type: super::rknn_tensor_qnt_type,
        /// Quantization parameter fl.
        pub fl: i8,
        /// Quantization parameter zp.
        pub zp: i32,
        /// Quantization parameter scale.
        pub scale: f32,
        /// Width stride.
        pub w_stride: u32,
        /// Tensor size with stride.
        pub size_with_stride: u32,
        /// Whether to pass through.
        pub pass_through: u8,
        /// Height stride.
        pub h_stride: u32,
    }
    impl Default for _rknn_tensor_attr {
        fn default() -> Self {
            _rknn_tensor_attr {
                index: 0,
                n_dims: 0,
                dims: [0; 16],
                name: [0; 256],
                n_elems: 0,
                size: 0,
                fmt: super::rknn_tensor_format::default(),
                type_: super::rknn_tensor_type::default(),
                qnt_type: super::rknn_tensor_qnt_type::default(),
                fl: 0,
                zp: 0,
                scale: 0.0,
                w_stride: 0,
                size_with_stride: 0,
                pass_through: 0,
                h_stride: 0,
            }
        }
    }

    /// RKNN input structure.
    ///
    /// This struct defines the input parameters for an RKNN model.
    #[derive(Debug, Copy, Clone)]
    pub struct _rknn_input {
        /// Input index.
        pub index: u32,
        /// Pointer to the input data buffer.
        pub buf: *mut ::std::os::raw::c_void,
        /// Size of the input data in bytes.
        pub size: u32,
        /// Whether to pass through.
        pub pass_through: u8,
        /// Input data type.
        pub type_: super::rknn_tensor_type,
        /// Input data format.
        pub fmt: super::rknn_tensor_format,
    }

    impl Default for _rknn_input {
        fn default() -> Self {
            _rknn_input {
                index: 0,
                buf: null_mut(),
                size: 0,
                pass_through: 1,
                fmt: super::rknn_tensor_format::default(),
                type_: super::rknn_tensor_type::default(),
            }
        }
    }

    /// Generic RKNN input structure.
    ///
    /// This struct provides a generic way to define inputs for an RKNN model.
    #[derive(Debug, Clone)]
    pub struct RknnInput<T> {
        /// Input index.
        pub index: usize,
        /// Input data buffer.
        pub buf: Vec<T>,
        /// Whether to pass through.
        pub pass_through: bool,
        /// Input data type.
        pub type_: RknnTensorType,
        /// Input data format.
        pub fmt: RknnTensorFormat,
    }

    impl<T> Default for RknnInput<T> {
        fn default() -> Self {
            Self {
                index: Default::default(),
                buf: Default::default(),
                pass_through: Default::default(),
                type_: RknnTensorType::Float32,
                fmt: RknnTensorFormat::Undefined,
            }
        }
    }

    /// RKNN tensor type.
    ///
    /// This enum defines the supported tensor data types in an RKNN model.
    #[derive(Debug, Copy, Clone)]
    pub enum RknnTensorType {
        /// 32-bit floating point.
        Float32 = 0,
        /// 16-bit floating point.
        Float16,
        /// 8-bit signed integer.
        Int8,
        /// 8-bit unsigned integer.
        Uint8,
        /// 16-bit signed integer.
        Int16,
        /// 16-bit unsigned integer.
        Uint16,
        /// 32-bit signed integer.
        Int32,
        /// 32-bit unsigned integer.
        Uint32,
        /// 64-bit signed integer.
        Int64,
        /// Boolean.
        Boolean,
        /// 4-bit integer.
        Int4,
        /// 16-bit brain floating point.
        BFloat16,
        /// Maximum type value (for boundary checking).
        TypeMax,
    }

    /// RKNN tensor format.
    ///
    /// This enum defines the supported tensor data formats in an RKNN model.
    #[derive(Debug, Copy, Clone)]
    pub enum RknnTensorFormat {
        /// NCHW format (batch-channel-height-width).
        NCHW = 0,
        /// NHWC format (batch-height-width-channel).
        NHWC,
        /// NC1HWC2 format.
        NC1HWC2,
        /// Undefined format.
        Undefined,
        /// Maximum format value (for boundary checking).
        FormatMax,
    }

    impl RknnTensorFormat {
        /// Convert an integer value to a tensor format.
        ///
        /// # Parameters
        ///
        /// - `input`: The integer value representing the format.
        ///
        /// # Returns
        ///
        /// The corresponding `RknnTensorFormat` value.
        pub fn from_int(input: u32) -> Self {
            match input {
                0 => RknnTensorFormat::NCHW,
                1 => RknnTensorFormat::NHWC,
                2 => RknnTensorFormat::NC1HWC2,
                3 => RknnTensorFormat::Undefined,
                _ => RknnTensorFormat::FormatMax,
            }
        }
    }

    /// Get the string representation of a tensor format.
    ///
    /// # Parameters
    ///
    /// - `fmt`: The tensor format.
    ///
    /// # Returns
    ///
    /// The string representation of the format.
    pub fn get_format_string(fmt: RknnTensorFormat) -> &'static str {
        match fmt {
            RknnTensorFormat::NCHW => "NCHW",
            RknnTensorFormat::NHWC => "NHWC",
            RknnTensorFormat::NC1HWC2 => "NC1HWC2",
            RknnTensorFormat::Undefined => "Undefined",
            RknnTensorFormat::FormatMax => "FormatMax",
        }
    }

    /// Error type for RKNN operations.
    #[derive(Debug)]
    pub struct Error(String);
    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    impl std::error::Error for Error {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            None
        }

        fn description(&self) -> &str {
            "description() is deprecated; use Display"
        }

        fn cause(&self) -> Option<&dyn std::error::Error> {
            self.source()
        }
    }

    macro_rules! rkerr {
        ($msg:expr, $code:expr) => {
            Err(Error(format!("{} exit code:{}", $msg, $code)))
        };
    }

    /// RKNN output structure.
    ///
    /// This struct holds the output data of an RKNN model and includes internal structures for resource release.
    pub struct RknnOutput<T> {
        /// Output data.
        pub data: Box<ManuallyDrop<Vec<T>>>,
        for_release: [super::rknn_output;1],
    }

    /// RKNN model.
    ///
    /// This struct encapsulates the context of an RKNN model, providing methods to load the model, set inputs, run inference, and retrieve outputs.
    /// 
    /// # Examples
    ///
    /// Here’s a simple example of how to use the `Rknn` struct:
    ///
    /// ```rust
    /// use std::path::Path;
    /// use crate::prelude::*;
    ///
    /// fn main() -> Result<(), Error> {
    ///     // Initialize the model
    ///     let model_path = Path::new("model.rknn");
    ///     let rknn = Rknn::rknn_init(model_path)?;
    ///
    ///     // Set input
    ///     let mut input = RknnInput::<f32> {
    ///         index: 0,
    ///         buf: vec![0.0; 100],
    ///         pass_through: false,
    ///         type_: RknnTensorType::Float32,
    ///         fmt: RknnTensorFormat::NCHW,
    ///     };
    ///     rknn.input_set(&mut input)?;
    ///
    ///     // Run the model
    ///     rknn.run()?;
    ///
    ///     // Get output
    ///     let output = rknn.outputs_get::<f32>()?;
    ///     println!("Output: {:?}", output);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[doc = "Rknn model"]
    #[derive(Debug, Clone)]
    pub struct Rknn {
        context: super::rknn_context,
    }
    impl Rknn {
        /// Initialize an RKNN model.
        ///
        /// # Parameters
        ///
        /// - `model_path`: The path to the model file.
        ///
        /// # Returns
        ///
        /// If successful, returns an `Rknn` instance; otherwise, returns an `Error`.
        pub fn rknn_init<P: AsRef<std::path::Path>>(model_path: P) -> Result<Self, Error> {
            let mut ret = Rknn { context: 0 };
            let path_str = model_path.as_ref().to_string_lossy();
            let path_cstr = CString::new(path_str.as_ref()).unwrap();

            unsafe {
                let result = super::rknn_init(
                    &mut ret.context,
                    path_cstr.as_ptr() as *mut std::ffi::c_void,
                    0,
                    0,
                    null_mut(),
                );
                if result != 0 {
                    return rkerr!("rknn_init faild.", result);
                }
            }
            Ok(ret)
        }

        /// Destroy an RKNN model.
        ///
        /// # Returns
        ///
        /// If successful, returns `Ok(()`; otherwise, returns an `Error`.
        pub fn destroy(&self) -> Result<(), Error> {
            let result = unsafe { super::rknn_destroy(self.context) };
            if result != 0 {
                return rkerr!("rknn_destroy faild.", result);
            }
            Ok(())
        }

        /// Set the model's input.
        ///
        /// # Parameters
        ///
        /// - `input`: A mutable reference to the generic input structure `RknnInput<T>`.
        ///
        /// # Returns
        ///
        /// If successful, returns `Ok(()`; otherwise, returns an `Error`.
        pub fn input_set<T: Pod + 'static>(&self, input: &mut RknnInput<T>) -> Result<(), Error> {
            let total_bytes = (input.buf.len() * mem::size_of::<T>()) as u32;
            let mut c_input = super::rknn_input {
                index: input.index as u32,
                buf: input.buf.as_mut_ptr() as *mut c_void,
                size: total_bytes,
                pass_through: if input.pass_through { 1 } else { 0 },
                type_: input.type_ as u32,
                fmt: input.fmt as u32,
            };

            let result = unsafe { super::rknn_inputs_set(self.context, 1, &mut c_input) };
            if result != 0 {
                return rkerr!("rknn_inputs_set failed.", result);
            }
            Ok(())
        }

        /// Run the RKNN model.
        ///
        /// # Returns
        ///
        /// If successful, returns `Ok(()`; otherwise, returns an `Error`.
        pub fn run(&self) -> Result<(), Error> {
            let result = unsafe { super::rknn_run(self.context, null_mut()) };
            if result != 0 {
                return rkerr!("rknn_run faild.", result);
            }
            Ok(())
        }

        /// Retrieve input/output information of the model.
        ///
        /// This method queries the model's input and output tensor attributes and prints them.
        ///
        /// # Returns
        ///
        /// If successful, returns `Ok(()`; otherwise, returns an `Error`.
        pub fn info(&self) -> Result<(), Error> {
            let mut io_num = super::_rknn_input_output_num {
                n_input: 0,
                n_output: 0,
            };

            let result = unsafe {
                super::rknn_query(
                    self.context,
                    super::_rknn_query_cmd_RKNN_QUERY_IN_OUT_NUM,
                    &mut io_num as *mut super::_rknn_input_output_num as *mut std::ffi::c_void,
                    mem::size_of::<super::_rknn_input_output_num>() as u32,
                )
            };

            if result != 0 {
                return rkerr!("rknn_query  faild.", result);
            }

            for i in 0..io_num.n_input {
                let mut rknn_tensor_attr = _rknn_tensor_attr::default();
                rknn_tensor_attr.index = i;
                let result = unsafe {
                    super::rknn_query(
                        self.context,
                        super::_rknn_query_cmd_RKNN_QUERY_INPUT_ATTR,
                        &mut rknn_tensor_attr as *mut _rknn_tensor_attr as *mut std::ffi::c_void,
                        mem::size_of::<super::_rknn_tensor_attr>() as u32,
                    )
                };
                println!("{:?}", rknn_tensor_attr);
                if result != 0 {
                    return rkerr!("rknn_query faild.", result);
                }
            }

            for i in 0..io_num.n_input {
                let mut rknn_tensor_attr = _rknn_tensor_attr::default();
                rknn_tensor_attr.index = i;
                let result = unsafe {
                    super::rknn_query(
                        self.context,
                        super::_rknn_query_cmd_RKNN_QUERY_OUTPUT_ATTR,
                        &mut rknn_tensor_attr as *mut _rknn_tensor_attr as *mut std::ffi::c_void,
                        mem::size_of::<super::_rknn_tensor_attr>() as u32,
                    )
                };
                println!("{:?}", rknn_tensor_attr);
                if result != 0 {
                    return rkerr!("rknn_query faild.", result);
                }
            }

            Ok(())
        }

        /// Get the model's output (copy version).
        ///
        /// This method includes built-in output resource release but copies the output data. For zero-copy, use `outputs_get_raw`.
        ///
        /// # Returns
        ///
        /// If successful, returns a `Vec<T>` containing the output data; otherwise, returns an `Error`.
        pub fn outputs_get<T: Pod + Copy + 'static>(&self) -> Result<Vec<T>, Error> {
            let mut out: [super::rknn_output; 1] = [super::rknn_output {
                want_float: 1,
                is_prealloc: 0,
                index: 0,
                buf: std::ptr::null_mut(),
                size: 0,
            }];
            let out_ptr = out.as_mut_ptr();
            let result =
                unsafe { super::rknn_outputs_get(self.context, 1, out_ptr, std::ptr::null_mut()) };
            if result != 0 {
                return rkerr!("rknn_outputs_get faild.", result);
            }
            let element_size = mem::size_of::<T>();
            let num_elements = out[0].size as usize / element_size;

            let t_slice = unsafe { slice::from_raw_parts(out[0].buf as *const T, num_elements) };

            // 這個動作已經copy了
            let ret = t_slice.to_vec();

            let result = unsafe { super::rknn_outputs_release(self.context, 1, out_ptr) };
            if result != 0 {
                return rkerr!("rknn_outputs_release faild.", result);
            }
            Ok(ret)
        }

        /// Get the model's output (raw version).
        ///
        /// This method returns raw output data (zero-copy) and delegates resource management to `RknnOutput<T>`. You must manually call `outputs_release` to free resources.
        ///
        /// # Returns
        ///
        /// If successful, returns a `ManuallyDrop<RknnOutput<T>>`; otherwise, returns an `Error`.
        pub fn outputs_get_raw<T: Pod + Copy + 'static>(&self) -> Result<ManuallyDrop<RknnOutput<T>>, Error> {
            let mut out: [super::rknn_output; 1] = [super::rknn_output {
                want_float: 1,
                is_prealloc: 0,
                index: 0,
                buf: std::ptr::null_mut(),
                size: 0,
            }];
            
            let out_ptr = out.as_mut_ptr();
            let result =
                unsafe { super::rknn_outputs_get(self.context, 1, out_ptr, std::ptr::null_mut()) };
            if result != 0 {
                return rkerr!("rknn_outputs_get faild.", result);
            }
            let element_size = mem::size_of::<T>();
            let num_elements = out[0].size as usize / element_size;
            let t_slice = unsafe { Vec::from_raw_parts(out[0].buf as *mut T, num_elements, num_elements) };
            let ret = ManuallyDrop::new(RknnOutput {
                data: Box::new(ManuallyDrop::new(t_slice)),
                for_release: out,
            });
            Ok(ret)
        }

        /// Release the model's output resources.
        ///
        /// This method releases resources associated with the output returned by `outputs_get_raw`.
        ///
        /// # Parameters
        ///
        /// - `rknnoutput`: A mutable reference to `RknnOutput<T>`.
        ///
        /// # Returns
        ///
        /// If successful, returns `Ok(()`; otherwise, returns an `Error`.
        pub fn outputs_release<T>(&self, rknnoutput: &mut ManuallyDrop<RknnOutput<T>>) -> Result<(), Error> {
            let mut out = rknnoutput.for_release;
            let out_ptr = out.as_mut_ptr();
            let result = unsafe { super::rknn_outputs_release(self.context, 1, out_ptr) };
            unsafe { ManuallyDrop::drop(rknnoutput) };
            if result != 0 {
                return rkerr!("rknn_outputs_release faild.", result);
            }
            Ok(())
        }
    }
}
