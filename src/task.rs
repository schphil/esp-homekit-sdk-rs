use core::ptr;

use crate::*;

pub struct Task {}

impl Task {
    pub fn create(f: fn(*mut c_types::c_void), name: &str, stacksize: u32, priority: u32) {
        let ptr = &f(0 as *mut c_types::c_void) as *const ();

        unsafe {
            let f = std::mem::transmute::<
                *const (),
                Option<unsafe extern "C" fn(*mut c_types::c_void) -> ()>,
            >(ptr);

            xTaskCreatePinnedToCore(
                f,
                name.as_ptr() as *mut i8,
                stacksize,
                ptr::null_mut(),
                priority,
                ptr::null_mut(),
                0,
            );
        }
    }
}
