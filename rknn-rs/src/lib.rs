#![allow(non_camel_case_types)]

use rknn_sys_rs as rknn_sys;

pub mod error;

/// Prelude module for RKNN (Rockchip Neural Network) related functionality.
///
/// This module contains commonly used types and functions for interacting with RKNN, making it convenient for use in other modules.
pub mod prelude {
    use super::rknn_sys;
    use std::{
        ffi::CString,
        marker::PhantomData,
        mem,
        os::raw::{c_char, c_void},
        ptr::{self, null_mut, NonNull},
        slice, str,
    };

    pub use crate::error::Error;
    use crate::rkerr;
    use bytemuck::Pod;

    fn c_char_array_to_string(chars: &[c_char]) -> String {
        let end = chars.iter().position(|&c| c == 0).unwrap_or(chars.len());
        let bytes = unsafe { slice::from_raw_parts(chars.as_ptr() as *const u8, end) };
        String::from_utf8_lossy(bytes).into_owned()
    }

    /// RKNN tensor attributes.
    ///
    /// This struct describes the attributes of a tensor in an RKNN model, including dimensions, name, type, etc.
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    struct _rknn_tensor_attr {
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
        pub fmt: rknn_sys::rknn_tensor_format,
        /// Tensor type.
        pub type_: rknn_sys::rknn_tensor_type,
        /// Tensor quantization type.
        pub qnt_type: rknn_sys::rknn_tensor_qnt_type,
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
                fmt: rknn_sys::rknn_tensor_format::default(),
                type_: rknn_sys::rknn_tensor_type::default(),
                qnt_type: rknn_sys::rknn_tensor_qnt_type::default(),
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

    impl _rknn_tensor_attr {
        fn name_string(&self) -> String {
            c_char_array_to_string(&self.name)
        }
    }

    /// RKNN input structure.
    ///
    /// This struct defines the input parameters for an RKNN model.
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    struct _rknn_input {
        /// Input index.
        pub index: u32,
        /// Pointer to the input data buffer.
        pub buf: *mut ::std::os::raw::c_void,
        /// Size of the input data in bytes.
        pub size: u32,
        /// Whether to pass through.
        pub pass_through: u8,
        /// Input data type.
        pub type_: rknn_sys::rknn_tensor_type,
        /// Input data format.
        pub fmt: rknn_sys::rknn_tensor_format,
    }

    impl Default for _rknn_input {
        fn default() -> Self {
            _rknn_input {
                index: 0,
                buf: null_mut(),
                size: 0,
                pass_through: 1,
                fmt: rknn_sys::rknn_tensor_format::default(),
                type_: rknn_sys::rknn_tensor_type::default(),
            }
        }
    }

    /// Generic RKNN input structure.
    ///
    /// This struct provides a generic way to define inputs for an RKNN model.
    #[derive(Debug)]
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

    impl RknnTensorType {
        pub fn from_int(input: u32) -> Self {
            match input {
                0 => RknnTensorType::Float32,
                1 => RknnTensorType::Float16,
                2 => RknnTensorType::Int8,
                3 => RknnTensorType::Uint8,
                4 => RknnTensorType::Int16,
                5 => RknnTensorType::Uint16,
                6 => RknnTensorType::Int32,
                7 => RknnTensorType::Uint32,
                8 => RknnTensorType::Int64,
                9 => RknnTensorType::Boolean,
                10 => RknnTensorType::Int4,
                11 => RknnTensorType::BFloat16,
                _ => RknnTensorType::TypeMax,
            }
        }
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

    #[derive(Debug, Copy, Clone)]
    pub enum RknnTensorQntType {
        None = 0,
        Dfp = 1,
        AffineAsymmetric = 2,
        QntMax,
    }

    impl RknnTensorQntType {
        pub fn from_int(input: u32) -> Self {
            match input {
                0 => RknnTensorQntType::None,
                1 => RknnTensorQntType::Dfp,
                2 => RknnTensorQntType::AffineAsymmetric,
                _ => RknnTensorQntType::QntMax,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct RknnTensorAttr {
        pub index: u32,
        pub n_dims: u32,
        pub dims: Vec<u32>,
        pub name: String,
        pub n_elems: u32,
        pub size: u32,
        pub fmt: RknnTensorFormat,
        pub type_: RknnTensorType,
        pub qnt_type: RknnTensorQntType,
        pub fl: i8,
        pub zp: i32,
        pub scale: f32,
        pub w_stride: u32,
        pub size_with_stride: u32,
        pub pass_through: bool,
        pub h_stride: u32,
    }

    impl From<_rknn_tensor_attr> for RknnTensorAttr {
        fn from(raw: _rknn_tensor_attr) -> Self {
            let dims_len = raw.n_dims.min(raw.dims.len() as u32) as usize;
            RknnTensorAttr {
                index: raw.index,
                n_dims: raw.n_dims,
                dims: raw.dims[..dims_len].to_vec(),
                name: raw.name_string(),
                n_elems: raw.n_elems,
                size: raw.size,
                fmt: RknnTensorFormat::from_int(raw.fmt),
                type_: RknnTensorType::from_int(raw.type_),
                qnt_type: RknnTensorQntType::from_int(raw.qnt_type),
                fl: raw.fl,
                zp: raw.zp,
                scale: raw.scale,
                w_stride: raw.w_stride,
                size_with_stride: raw.size_with_stride,
                pass_through: raw.pass_through != 0,
                h_stride: raw.h_stride,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct RknnSdkVersion {
        pub api_version: String,
        pub drv_version: String,
    }

    #[derive(Debug, Copy, Clone)]
    pub struct RknnInputOutputNum {
        pub n_input: u32,
        pub n_output: u32,
    }

    #[derive(Debug, Clone)]
    pub struct RknnModelInfo {
        pub io_num: RknnInputOutputNum,
        pub input_attrs: Vec<RknnTensorAttr>,
        pub output_attrs: Vec<RknnTensorAttr>,
    }

    #[derive(Debug, Copy, Clone)]
    #[repr(u32)]
    pub enum RknnCoreMask {
        Auto = 0,
        Core0 = 1,
        Core1 = 2,
        Core2 = 4,
        Core0_1 = 3,
        Core0_1_2 = 7,
        All = 0xffff,
        Undefined = 0x1_0000,
    }

    pub struct RknnMemAllocFlags;
    impl RknnMemAllocFlags {
        pub const DEFAULT: u64 = 0;
        pub const CACHEABLE: u64 = 1 << 0;
        pub const NON_CACHEABLE: u64 = 1 << 1;
        pub const TRY_ALLOC_SRAM: u64 = 1 << 2;
    }

    #[derive(Debug, Copy, Clone)]
    #[repr(u32)]
    pub enum RknnMemSyncMode {
        ToDevice = 0x1,
        FromDevice = 0x2,
        Bidirectional = 0x3,
    }

    #[derive(Debug)]
    pub struct RknnTensorMemory<'a> {
        context: rknn_sys::rknn_context,
        raw: *mut rknn_sys::rknn_tensor_mem,
        _owner: PhantomData<&'a Rknn>,
    }

    impl<'a> Drop for RknnTensorMemory<'a> {
        fn drop(&mut self) {
            if !self.raw.is_null() {
                unsafe {
                    rknn_sys::rknn_destroy_mem(self.context, self.raw);
                }
                self.raw = ptr::null_mut();
            }
        }
    }

    impl<'a> RknnTensorMemory<'a> {
        fn raw_ref(&self) -> Result<&rknn_sys::rknn_tensor_mem, Error> {
            unsafe { self.raw.as_ref() }
                .ok_or_else(|| Error("RknnTensorMemory has been released.".to_string()))
        }

        fn raw_mut_ref(&mut self) -> Result<&mut rknn_sys::rknn_tensor_mem, Error> {
            unsafe { self.raw.as_mut() }
                .ok_or_else(|| Error("RknnTensorMemory has been released.".to_string()))
        }

        fn raw_bytes_ptr(virt_addr: *mut c_void, size: usize) -> Result<*mut u8, Error> {
            if size == 0 {
                return Ok(NonNull::<u8>::dangling().as_ptr());
            }
            if virt_addr.is_null() {
                return Err(Error("Tensor memory points to a null buffer.".to_string()));
            }
            Ok(virt_addr as *mut u8)
        }

        pub fn size(&self) -> Result<u32, Error> {
            Ok(self.raw_ref()?.size)
        }

        pub fn fd(&self) -> Result<i32, Error> {
            Ok(self.raw_ref()?.fd)
        }

        pub fn as_bytes(&self) -> Result<&[u8], Error> {
            let raw = self.raw_ref()?;
            let size = raw.size as usize;
            let ptr = Self::raw_bytes_ptr(raw.virt_addr, size)?;
            Ok(unsafe { slice::from_raw_parts(ptr as *const u8, size) })
        }

        pub fn as_bytes_mut(&mut self) -> Result<&mut [u8], Error> {
            let raw = self.raw_mut_ref()?;
            let size = raw.size as usize;
            let ptr = Self::raw_bytes_ptr(raw.virt_addr, size)?;
            Ok(unsafe { slice::from_raw_parts_mut(ptr, size) })
        }

        pub fn as_slice<T: Pod>(&self) -> Result<&[T], Error> {
            let bytes = self.as_bytes()?;
            bytemuck::try_cast_slice(bytes).map_err(|_| {
                Error(format!(
                    "Tensor memory cannot be viewed as {}",
                    std::any::type_name::<T>()
                ))
            })
        }

        pub fn as_mut_slice<T: Pod>(&mut self) -> Result<&mut [T], Error> {
            let bytes = self.as_bytes_mut()?;
            bytemuck::try_cast_slice_mut(bytes).map_err(|_| {
                Error(format!(
                    "Tensor memory cannot be viewed as mutable {}",
                    std::any::type_name::<T>()
                ))
            })
        }

        pub fn write_slice<T: Pod>(&mut self, data: &[T]) -> Result<(), Error> {
            let dst = self.as_mut_slice::<T>()?;
            if data.len() > dst.len() {
                return Err(Error(format!(
                    "Input data is too large: {} elements > {} elements",
                    data.len(),
                    dst.len()
                )));
            }
            dst[..data.len()].copy_from_slice(data);
            Ok(())
        }

        pub fn sync(&self, mode: RknnMemSyncMode) -> Result<(), Error> {
            let result = unsafe {
                rknn_sys::rknn_mem_sync(
                    self.context,
                    self.raw,
                    mode as rknn_sys::rknn_mem_sync_mode,
                )
            };
            if result != 0 {
                return rkerr!("rknn_mem_sync failed.", result);
            }
            Ok(())
        }
    }

    /// RKNN output structure.
    ///
    /// This struct holds the output data of an RKNN model and includes internal structures for resource release.
    /// It implements `Drop` to automatically release resources.
    pub struct RknnOutput<'a, T> {
        context: rknn_sys::rknn_context,
        memory: &'a [T],
        // Holds ALL output structs from the rknn_outputs_get call.
        // The RKNN runtime accesses all model outputs regardless of n_outputs,
        // so we must allocate the full array and release it together.
        all_raws: Vec<rknn_sys::rknn_output>,
    }

    impl<'a, T> Drop for RknnOutput<'a, T> {
        fn drop(&mut self) {
            if !self.all_raws.is_empty() {
                unsafe {
                    rknn_sys::rknn_outputs_release(
                        self.context,
                        self.all_raws.len() as u32,
                        self.all_raws.as_mut_ptr(),
                    );
                }
            }
        }
    }

    impl<'a, T> std::ops::Deref for RknnOutput<'a, T> {
        type Target = [T];
        fn deref(&self) -> &Self::Target {
            self.memory
        }
    }

    impl<'a, T: std::fmt::Debug> std::fmt::Debug for RknnOutput<'a, T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("RknnOutput")
                .field("context", &self.context)
                .field("memory", &self.memory)
                .finish()
        }
    }

    /// RKNN model.
    ///
    /// This struct encapsulates the context of an RKNN model, providing methods to load the model, set inputs, run inference, and retrieve outputs.
    ///
    /// # Examples
    ///
    /// Here’s a simple example of how to use the `Rknn` struct:
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use rknn_rs::prelude::*;
    ///
    /// fn main() -> Result<(), Error> {
    ///     // Initialize the model
    ///     let model_path = Path::new("model.rknn");
    ///     let rknn = Rknn::rknn_init(model_path)?;
    ///
    ///     // Set input
    ///     let input = RknnInput::<f32> {
    ///         index: 0,
    ///         buf: vec![0.0; 100],
    ///         pass_through: false,
    ///         type_: RknnTensorType::Float32,
    ///         fmt: RknnTensorFormat::NCHW,
    ///     };
    ///     rknn.input_set(&input)?;
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
    #[derive(Debug)]
    pub struct Rknn {
        context: rknn_sys::rknn_context,
    }

    impl Drop for Rknn {
        fn drop(&mut self) {
            if self.context != 0 {
                unsafe { rknn_sys::rknn_destroy(self.context) };
            }
        }
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
            let path_cstr = CString::new(path_str.as_ref())
                .map_err(|e| Error(format!("Invalid model path: {}", e)))?;

            unsafe {
                let result = rknn_sys::rknn_init(
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

        pub fn new<P: AsRef<std::path::Path>>(model_path: P) -> Result<Self, Error> {
            Self::rknn_init(model_path)
        }

        /// Set the model's input.
        ///
        /// # Parameters
        ///
        /// - `input`: A reference to the generic input structure `RknnInput<T>`.
        ///
        /// # Returns
        ///
        /// If successful, returns `Ok(()`; otherwise, returns an `Error`.
        pub fn input_set<T: Pod + 'static>(&self, input: &RknnInput<T>) -> Result<(), Error> {
            self.input_set_slice(
                input.index,
                &input.buf,
                input.pass_through,
                input.type_,
                input.fmt,
            )
        }

        pub fn input_set_slice<T: Pod + 'static>(
            &self,
            index: usize,
            buf: &[T],
            pass_through: bool,
            type_: RknnTensorType,
            fmt: RknnTensorFormat,
        ) -> Result<(), Error> {
            let total_bytes = (buf.len() * mem::size_of::<T>()) as u32;
            let mut c_input = rknn_sys::rknn_input {
                index: index as u32,
                buf: buf.as_ptr() as *mut c_void,
                size: total_bytes,
                pass_through: if pass_through { 1 } else { 0 },
                type_: type_ as u32,
                fmt: fmt as u32,
            };

            let result = unsafe { rknn_sys::rknn_inputs_set(self.context, 1, &mut c_input) };
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
            let result = unsafe { rknn_sys::rknn_run(self.context, null_mut()) };
            if result != 0 {
                return rkerr!("rknn_run faild.", result);
            }
            Ok(())
        }

        pub fn set_batch_core_num(&self, core_num: i32) -> Result<(), Error> {
            let result = unsafe { rknn_sys::rknn_set_batch_core_num(self.context, core_num) };
            if result != 0 {
                return rkerr!("rknn_set_batch_core_num failed.", result);
            }
            Ok(())
        }

        pub fn set_core_mask(&self, core_mask: RknnCoreMask) -> Result<(), Error> {
            let result = unsafe {
                rknn_sys::rknn_set_core_mask(self.context, core_mask as rknn_sys::rknn_core_mask)
            };
            if result != 0 {
                return rkerr!("rknn_set_core_mask failed.", result);
            }
            Ok(())
        }

        pub fn sdk_version(&self) -> Result<RknnSdkVersion, Error> {
            let mut version = rknn_sys::_rknn_sdk_version {
                api_version: [0; 256],
                drv_version: [0; 256],
            };
            let result = unsafe {
                rknn_sys::rknn_query(
                    self.context,
                    rknn_sys::_rknn_query_cmd_RKNN_QUERY_SDK_VERSION,
                    &mut version as *mut rknn_sys::_rknn_sdk_version as *mut c_void,
                    mem::size_of::<rknn_sys::_rknn_sdk_version>() as u32,
                )
            };
            if result != 0 {
                return rkerr!("rknn_query sdk_version failed.", result);
            }
            Ok(RknnSdkVersion {
                api_version: c_char_array_to_string(&version.api_version),
                drv_version: c_char_array_to_string(&version.drv_version),
            })
        }

        pub fn io_num(&self) -> Result<RknnInputOutputNum, Error> {
            let mut io_num = rknn_sys::_rknn_input_output_num {
                n_input: 0,
                n_output: 0,
            };
            let result = unsafe {
                rknn_sys::rknn_query(
                    self.context,
                    rknn_sys::_rknn_query_cmd_RKNN_QUERY_IN_OUT_NUM,
                    &mut io_num as *mut rknn_sys::_rknn_input_output_num as *mut c_void,
                    mem::size_of::<rknn_sys::_rknn_input_output_num>() as u32,
                )
            };
            if result != 0 {
                return rkerr!("rknn_query in_out_num failed.", result);
            }
            Ok(RknnInputOutputNum {
                n_input: io_num.n_input,
                n_output: io_num.n_output,
            })
        }

        fn query_tensor_attr(
            &self,
            index: u32,
            query_cmd: rknn_sys::rknn_query_cmd,
        ) -> Result<RknnTensorAttr, Error> {
            let mut attr = _rknn_tensor_attr::default();
            attr.index = index;
            let result = unsafe {
                rknn_sys::rknn_query(
                    self.context,
                    query_cmd,
                    &mut attr as *mut _rknn_tensor_attr as *mut c_void,
                    mem::size_of::<_rknn_tensor_attr>() as u32,
                )
            };
            if result != 0 {
                return rkerr!("rknn_query tensor_attr failed.", result);
            }
            Ok(attr.into())
        }

        fn query_tensor_attrs(
            &self,
            query_cmd: rknn_sys::rknn_query_cmd,
            count: u32,
        ) -> Result<Vec<RknnTensorAttr>, Error> {
            let mut attrs = Vec::with_capacity(count as usize);
            for i in 0..count {
                attrs.push(self.query_tensor_attr(i, query_cmd)?);
            }
            Ok(attrs)
        }

        pub fn input_attrs(&self) -> Result<Vec<RknnTensorAttr>, Error> {
            let io_num = self.io_num()?;
            self.query_tensor_attrs(
                rknn_sys::_rknn_query_cmd_RKNN_QUERY_INPUT_ATTR,
                io_num.n_input,
            )
        }

        pub fn output_attrs(&self) -> Result<Vec<RknnTensorAttr>, Error> {
            let io_num = self.io_num()?;
            self.query_tensor_attrs(
                rknn_sys::_rknn_query_cmd_RKNN_QUERY_OUTPUT_ATTR,
                io_num.n_output,
            )
        }

        pub fn model_info(&self) -> Result<RknnModelInfo, Error> {
            let io_num = self.io_num()?;
            let input_attrs = self.query_tensor_attrs(
                rknn_sys::_rknn_query_cmd_RKNN_QUERY_INPUT_ATTR,
                io_num.n_input,
            )?;
            let output_attrs = self.query_tensor_attrs(
                rknn_sys::_rknn_query_cmd_RKNN_QUERY_OUTPUT_ATTR,
                io_num.n_output,
            )?;
            Ok(RknnModelInfo {
                io_num,
                input_attrs,
                output_attrs,
            })
        }

        pub fn create_mem<'a>(&'a self, size: u32) -> Result<RknnTensorMemory<'a>, Error> {
            let raw = unsafe { rknn_sys::rknn_create_mem(self.context, size) };
            if raw.is_null() {
                return Err(Error("rknn_create_mem failed.".to_string()));
            }
            Ok(RknnTensorMemory {
                context: self.context,
                raw,
                _owner: PhantomData,
            })
        }

        pub fn create_mem2<'a>(
            &'a self,
            size: u64,
            alloc_flags: u64,
        ) -> Result<RknnTensorMemory<'a>, Error> {
            let raw = unsafe { rknn_sys::rknn_create_mem2(self.context, size, alloc_flags) };
            if raw.is_null() {
                return Err(Error("rknn_create_mem2 failed.".to_string()));
            }
            Ok(RknnTensorMemory {
                context: self.context,
                raw,
                _owner: PhantomData,
            })
        }

        /// Retrieve input/output information of the model.
        ///
        /// This method queries the model's input and output tensor attributes and prints them.
        ///
        /// # Returns
        ///
        /// If successful, returns `Ok(()`; otherwise, returns an `Error`.
        pub fn info(&self) -> Result<(), Error> {
            let info = self.model_info()?;
            println!("{:?}", info.io_num);
            for attr in &info.input_attrs {
                println!("input: {:?}", attr);
            }
            for attr in &info.output_attrs {
                println!("output: {:?}", attr);
            }
            Ok(())
        }

        /// Get the model's output (raw version).
        ///
        /// This method returns raw output data (zero-copy) and delegates resource management to `RknnOutput<T>`.
        /// The returned `RknnOutput` automatically releases resources when dropped.
        ///
        /// # Arguments
        ///
        /// * `index` - Output tensor index (default 0 for single-output models).
        /// * `want_float` - If true, ask the runtime to convert the output to float32.
        ///
        /// # Returns
        ///
        /// If successful, returns a `RknnOutput<'a, T>`; otherwise, returns an `Error`.
        pub fn outputs_get_by_index<'a, T: Pod + Copy + 'static>(
            &'a self,
            index: u32,
            want_float: bool,
        ) -> Result<RknnOutput<'a, T>, Error> {
            // IMPORTANT: The RKNN 2.3.x runtime internally iterates ALL model outputs
            // regardless of the n_outputs argument. We must allocate a full array
            // (size = n_model_output) so that the runtime never reads past the end.
            let n_total = self.io_num()?.n_output;
            if index >= n_total {
                return Err(Error(format!(
                    "output index {} out of range (model has {} outputs)",
                    index, n_total
                )));
            }

            // Zero all structs (including padding) to ensure the runtime reads clean data.
            let mut all_raws: Vec<rknn_sys::rknn_output> = (0..n_total)
                .map(|i| {
                    let mut o: rknn_sys::rknn_output = unsafe { mem::zeroed() };
                    o.want_float = if want_float { 1 } else { 0 };
                    o.is_prealloc = 0;
                    o.index = i;
                    o.buf = std::ptr::null_mut();
                    o.size = 0;
                    o
                })
                .collect();

            let result = unsafe {
                rknn_sys::rknn_outputs_get(
                    self.context,
                    n_total,
                    all_raws.as_mut_ptr(),
                    std::ptr::null_mut(),
                )
            };
            if result != 0 {
                return rkerr!("rknn_outputs_get faild.", result);
            }

            let desired = &all_raws[index as usize];
            if desired.buf.is_null() {
                // Release all before returning error
                unsafe {
                    rknn_sys::rknn_outputs_release(
                        self.context,
                        n_total,
                        all_raws.as_mut_ptr(),
                    );
                }
                return Err(Error(format!(
                    "rknn_outputs_get returned null buffer for output index {}",
                    index
                )));
            }
            let element_size = mem::size_of::<T>();
            let num_elements = desired.size as usize / element_size;
            let t_slice =
                unsafe { slice::from_raw_parts(desired.buf as *const T, num_elements) };

            Ok(RknnOutput {
                context: self.context,
                memory: t_slice,
                all_raws,
            })
        }

        /// Get the model's first output as float32.
        ///
        /// Convenience wrapper around [`outputs_get_by_index`] for single-output models.
        /// Asks the runtime to convert the output to float32 (`want_float = true`).
        pub fn outputs_get<'a, T: Pod + Copy + 'static>(
            &'a self,
        ) -> Result<RknnOutput<'a, T>, Error> {
            self.outputs_get_by_index(0, true)
        }
    }
}
