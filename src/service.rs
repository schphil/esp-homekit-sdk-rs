use core::ptr;
use log::*;

use crate::*;

pub fn create() -> *mut hap_serv_t {
    
    info!("Creating service");

    unsafe {
        hap_serv_outlet_create(false, false)
    }
}

pub fn add_name(service: *mut hap_serv_t, name: &str) {

    unsafe {
        hap_serv_add_char(service, hap_char_name_create(name.as_ptr() as *mut i8));
    }
}

pub fn get_service_by_uuid(service: *mut hap_serv_t) -> *mut hap_char_t {

    unsafe {
        hap_serv_get_char_by_uuid(service, HAP_CHAR_UUID_OUTLET_IN_USE.as_ptr() as *mut i8)
    }
}

pub fn set_write_cb(service: *mut hap_serv_t, write: hap_serv_write_t) {

    unsafe {
        hap_serv_set_write_cb(service, write)
    }
}
