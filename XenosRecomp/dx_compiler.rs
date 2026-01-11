use std::ffi::{c_char, c_void, CStr};
use std::ptr;

pub type HRESULT = i32;

const S_OK: HRESULT = 0;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

#[repr(C)]
pub struct DxcBuffer {
    pub ptr: *const c_void,
    pub size: usize,
    pub encoding: u32,
}

#[repr(C)]
pub struct IDxcCompiler3 {
    pub vtbl: *const IDxcCompiler3VTable,
}

#[repr(C)]
pub struct IDxcCompiler3VTable {
    pub query_interface: unsafe extern "system" fn(
        *mut IDxcCompiler3,
        *const Guid,
        *mut *mut c_void,
    ) -> HRESULT,
    pub add_ref: unsafe extern "system" fn(*mut IDxcCompiler3) -> u32,
    pub release: unsafe extern "system" fn(*mut IDxcCompiler3) -> u32,
    pub compile: unsafe extern "system" fn(
        *mut IDxcCompiler3,
        *const DxcBuffer,
        *const *const u16,
        u32,
        *mut c_void,
        *const Guid,
        *mut *mut c_void,
    ) -> HRESULT,
}

#[repr(C)]
pub struct IDxcResult {
    pub vtbl: *const IDxcResultVTable,
}

#[repr(C)]
pub struct IDxcResultVTable {
    pub query_interface: unsafe extern "system" fn(
        *mut IDxcResult,
        *const Guid,
        *mut *mut c_void,
    ) -> HRESULT,
    pub add_ref: unsafe extern "system" fn(*mut IDxcResult) -> u32,
    pub release: unsafe extern "system" fn(*mut IDxcResult) -> u32,
    pub get_status: unsafe extern "system" fn(*mut IDxcResult, *mut HRESULT) -> HRESULT,
    pub get_result: unsafe extern "system" fn(*mut IDxcResult, *mut *mut IDxcBlob) -> HRESULT,
    pub get_error_buffer:
        unsafe extern "system" fn(*mut IDxcResult, *mut *mut IDxcBlobUtf8) -> HRESULT,
    pub has_output: unsafe extern "system" fn(*mut IDxcResult, u32) -> bool,
    pub get_output: unsafe extern "system" fn(
        *mut IDxcResult,
        u32,
        *const Guid,
        *mut *mut c_void,
        *mut *mut c_void,
    ) -> HRESULT,
}

#[repr(C)]
pub struct IDxcBlob {
    pub vtbl: *const IDxcBlobVTable,
}

#[repr(C)]
pub struct IDxcBlobVTable {
    pub query_interface: unsafe extern "system" fn(
        *mut IDxcBlob,
        *const Guid,
        *mut *mut c_void,
    ) -> HRESULT,
    pub add_ref: unsafe extern "system" fn(*mut IDxcBlob) -> u32,
    pub release: unsafe extern "system" fn(*mut IDxcBlob) -> u32,
    pub get_buffer_pointer: unsafe extern "system" fn(*mut IDxcBlob) -> *mut c_void,
    pub get_buffer_size: unsafe extern "system" fn(*mut IDxcBlob) -> usize,
}

#[repr(C)]
pub struct IDxcBlobUtf8 {
    pub vtbl: *const IDxcBlobUtf8VTable,
}

#[repr(C)]
pub struct IDxcBlobUtf8VTable {
    pub query_interface: unsafe extern "system" fn(
        *mut IDxcBlobUtf8,
        *const Guid,
        *mut *mut c_void,
    ) -> HRESULT,
    pub add_ref: unsafe extern "system" fn(*mut IDxcBlobUtf8) -> u32,
    pub release: unsafe extern "system" fn(*mut IDxcBlobUtf8) -> u32,
    pub get_buffer_pointer: unsafe extern "system" fn(*mut IDxcBlobUtf8) -> *mut c_void,
    pub get_buffer_size: unsafe extern "system" fn(*mut IDxcBlobUtf8) -> usize,
    pub get_string_pointer: unsafe extern "system" fn(*mut IDxcBlobUtf8) -> *const c_char,
    pub get_string_length: unsafe extern "system" fn(*mut IDxcBlobUtf8) -> usize,
}

pub const DXC_OUT_OBJECT: u32 = 0;
pub const DXC_OUT_ERRORS: u32 = 1;

extern "system" {
    pub fn DxcCreateInstance(
        clsid: *const Guid,
        iid: *const Guid,
        out: *mut *mut c_void,
    ) -> HRESULT;
}

pub struct DxcCompiler {
    dxc_compiler: *mut IDxcCompiler3,
}

impl DxcCompiler {
    pub fn new(clsid: &Guid, iid: &Guid) -> Result<Self, HRESULT> {
        let mut compiler_ptr: *mut c_void = ptr::null_mut();
        let hr = unsafe { DxcCreateInstance(clsid, iid, &mut compiler_ptr) };
        if succeeded(hr) {
            Ok(Self {
                dxc_compiler: compiler_ptr as *mut IDxcCompiler3,
            })
        } else {
            Err(hr)
        }
    }

    pub fn compile(
        &self,
        shader_source: &str,
        compile_pixel_shader: bool,
        compile_library: bool,
        compile_spirv: bool,
        result_iid: &Guid,
        blob_iid: &Guid,
        blob_utf8_iid: &Guid,
    ) -> Option<*mut IDxcBlob> {
        if self.dxc_compiler.is_null() {
            return None;
        }

        if compile_library && compile_spirv {
            return None;
        }

        let source = DxcBuffer {
            ptr: shader_source.as_ptr() as *const c_void,
            size: shader_source.len(),
            encoding: 0,
        };

        let target = if compile_library {
            "-T lib_6_3"
        } else if compile_pixel_shader {
            "-T ps_6_0"
        } else {
            "-T vs_6_0"
        };

        let mut args = Vec::new();
        args.push(wide(target));
        args.push(wide("-HV 2021"));
        args.push(wide("-all-resources-bound"));

        if compile_spirv {
            args.push(wide("-spirv"));
            args.push(wide("-fvk-use-dx-layout"));

            if !compile_pixel_shader {
                args.push(wide("-fvk-invert-y"));
            }
        } else {
            args.push(wide("-Wno-ignored-attributes"));
            args.push(wide("-Qstrip_reflect"));
        }

        args.push(wide("-Qstrip_debug"));

        #[cfg(feature = "unleashed_recomp")]
        {
            args.push(wide("-DUNLEASHED_RECOMP"));
        }

        let arg_ptrs: Vec<*const u16> = args.iter().map(|arg| arg.as_ptr()).collect();

        let mut result_ptr: *mut c_void = ptr::null_mut();
        let hr = unsafe {
            ((*(*self.dxc_compiler).vtbl).compile)(
                self.dxc_compiler,
                &source,
                arg_ptrs.as_ptr(),
                arg_ptrs.len() as u32,
                ptr::null_mut(),
                result_iid,
                &mut result_ptr,
            )
        };

        if !succeeded(hr) {
            return None;
        }

        let result = result_ptr as *mut IDxcResult;
        if result.is_null() {
            return None;
        }

        let mut status: HRESULT = S_OK;
        let status_hr = unsafe { ((*(*result).vtbl).get_status)(result, &mut status) };
        if !succeeded(status_hr) {
            unsafe { ((*(*result).vtbl).release)(result) };
            return None;
        }

        if !succeeded(status) {
            let has_errors = unsafe { ((*(*result).vtbl).has_output)(result, DXC_OUT_ERRORS) };
            if has_errors {
                let mut errors_ptr: *mut c_void = ptr::null_mut();
                let errors_hr = unsafe {
                    ((*(*result).vtbl).get_output)(
                        result,
                        DXC_OUT_ERRORS,
                        blob_utf8_iid,
                        &mut errors_ptr,
                        ptr::null_mut(),
                    )
                };

                if succeeded(errors_hr) {
                    let errors = errors_ptr as *mut IDxcBlobUtf8;
                    if !errors.is_null() {
                        unsafe {
                            let text_ptr = ((*(*errors).vtbl).get_string_pointer)(errors);
                            if !text_ptr.is_null() {
                                let c_str = CStr::from_ptr(text_ptr);
                                eprintln!("{}", c_str.to_string_lossy());
                            }
                            ((*(*errors).vtbl).release)(errors);
                        }
                    }
                }
            }

            unsafe { ((*(*result).vtbl).release)(result) };
            return None;
        }

        let mut object_ptr: *mut c_void = ptr::null_mut();
        let object_hr = unsafe {
            ((*(*result).vtbl).get_output)(
                result,
                DXC_OUT_OBJECT,
                blob_iid,
                &mut object_ptr,
                ptr::null_mut(),
            )
        };

        let object = if succeeded(object_hr) {
            let blob = object_ptr as *mut IDxcBlob;
            if blob.is_null() {
                None
            } else {
                Some(blob)
            }
        } else {
            None
        };

        unsafe { ((*(*result).vtbl).release)(result) };
        object
    }
}

impl Drop for DxcCompiler {
    fn drop(&mut self) {
        if !self.dxc_compiler.is_null() {
            unsafe {
                ((*(*self.dxc_compiler).vtbl).release)(self.dxc_compiler);
            }
        }
    }
}

fn succeeded(hr: HRESULT) -> bool {
    hr >= 0
}

fn wide(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}
