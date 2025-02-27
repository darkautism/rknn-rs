#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]


include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod prelude {
    use std::{
        ffi::{CStr, CString}, mem::{self, ManuallyDrop}, os::raw::c_void, ptr::null_mut, rc::Weak, slice, str, sync::Arc
    };

    use bytemuck::Pod;

    #[derive(Debug, Copy, Clone)]
    pub struct _rknn_tensor_attr {
        pub index: u32,
        pub n_dims: u32,
        pub dims: [u32; 16usize],
        pub name: [::std::os::raw::c_char; 256usize],
        pub n_elems: u32,
        pub size: u32,
        pub fmt: super::rknn_tensor_format,
        pub type_: super::rknn_tensor_type,
        pub qnt_type: super::rknn_tensor_qnt_type,
        pub fl: i8,
        pub zp: i32,
        pub scale: f32,
        pub w_stride: u32,
        pub size_with_stride: u32,
        pub pass_through: u8,
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

    #[derive(Debug, Copy, Clone)]
    pub struct _rknn_input {
        pub index: u32,
        pub buf: *mut ::std::os::raw::c_void,
        pub size: u32,
        pub pass_through: u8,
        pub type_: super::rknn_tensor_type,
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

    #[derive(Debug, Clone)]
    pub struct RknnInput<T> {
        pub index: usize,
        pub buf: Vec<T>,
        pub pass_through: bool,
        pub type_: RknnTensorType,
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

    #[derive(Debug, Copy, Clone)]
    pub enum RknnTensorType {
        Float32 = 0,
        Float16,
        Int8,
        Uint8,
        Int16,
        Uint16,
        Int32,
        Uint32,
        Int64,
        Boolean,
        Int4,
        BFloat16,
        TypeMax,
    }

    #[derive(Debug, Copy, Clone)]
    pub enum RknnTensorFormat {
        NCHW = 0,
        NHWC,
        NC1HWC2,
        Undefined,
        FormatMax,
    }

    impl RknnTensorFormat {
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

    pub fn get_format_string(fmt: RknnTensorFormat) -> &'static str {
        match fmt {
            RknnTensorFormat::NCHW => "NCHW",
            RknnTensorFormat::NHWC => "NHWC",
            RknnTensorFormat::NC1HWC2 => "NC1HWC2",
            RknnTensorFormat::Undefined => "Undefined",
            RknnTensorFormat::FormatMax => "FormatMax",
        }
    }

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

    pub struct RknnOutput<T> {
        pub data: Box<ManuallyDrop<Vec<T>>>,
        for_release: [super::rknn_output;1],
    }

    #[doc = "Rknn model"]
    pub struct Rknn {
        context: super::rknn_context,
    }
    impl Rknn {
        pub fn rknn_init(model_path: &str) -> Result<Self, Error> {
            let mut ret = Rknn { context: 0 };
            let model_path_cstr = CString::new(model_path).unwrap();
            let model_path_cstr_ptr = model_path_cstr.as_ptr();

            unsafe {
                let result = super::rknn_init(
                    &mut ret.context,
                    model_path_cstr_ptr as *mut std::ffi::c_void,
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

        pub fn destroy(&self) -> Result<(), Error> {
            let result = unsafe { super::rknn_destroy(self.context) };
            if result != 0 {
                return rkerr!("rknn_destroy faild.", result);
            }
            Ok(())
        }

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

        pub fn run(&self) -> Result<(), Error> {
            let result = unsafe { super::rknn_run(self.context, null_mut()) };
            if result != 0 {
                return rkerr!("rknn_run faild.", result);
            }
            Ok(())
        }

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

        // 這個output版本是懶人版，它內置了rknn_outputs_release，代價是必須複製輸出，如果為了效率要zerocopy，請使用outputs_get_raw
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
