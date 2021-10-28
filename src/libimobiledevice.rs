#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]
#![allow(unaligned_references)]

pub use crate::bindings_libimobiledevice as unsafe_bindings;
use crate::bindings_libimobiledevice::idevice_info_t;

// The end goal here is to create a safe library that can wrap the unsafe C code

/// Returns a vector of devices found by usbmuxd.
pub fn idevice_get_device_list_extended() -> Option<(Vec<idevice_info>, i32)> {
    // I have no idea how this whole function works, sb wrote it so complain to him.

    // get list of idevice_info_t
    let mut device_list: *mut idevice_info_t = std::ptr::null_mut();
    let mut device_count: i32 = 0;
    let result = unsafe {
        unsafe_bindings::idevice_get_device_list_extended(&mut device_list, &mut device_count)
    };

    // idevice_get_device_list_extended returns an error status code, return None if it's not 0s
    if result < 0 {
        return None;
    }

    // Create slice of mutable references to idevice_info_t from device_list and device_count
    let device_list_slice =
        unsafe { std::slice::from_raw_parts_mut(device_list, device_count as usize) };

    // Package up the found devices into a vector of idevice_info so Rust can manage the memory
    let mut to_return: Vec<idevice_info> = vec![];
    for i in device_list_slice.iter_mut() {
        unsafe {
            to_return.push(idevice_info::new(
                // What the heck C
                std::ffi::CStr::from_ptr((*(*i)).udid)
                    .to_string_lossy()
                    .to_string(),
                (*(*i)).conn_type,
            ));
        }
    }

    // Drop the memory that the C library allocated
    let device_list_ptr = device_list as *mut *mut std::os::raw::c_char;
    unsafe {
        unsafe_bindings::idevice_device_list_free(device_list_ptr);
    }

    // All other variables are dropped at return

    Some((to_return, device_count))
}

pub struct idevice_info {
    pub udid: String,
    pub conn_type: u32, // I have no idea what to do with conn_data
}

impl idevice_info {
    fn new(udid: String, conn_type: u32) -> Self {
        idevice_info { udid, conn_type }
    }
}
