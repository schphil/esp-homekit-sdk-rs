use std::ffi::CString;

use core::ptr;
use log::*;

use crate::*;

pub const HAP_SUCCESS_: i32 = 0;

pub struct Config {
    pub name: CString,
    pub model: CString,
    pub manufacturer: CString,
    pub serial_num: CString,
    pub fw_rev: CString,
    pub hw_rev: CString,
    pub pv: CString,
    pub cid: accessory::Category,
}
impl From<&Config> for hap_acc_cfg_t {
    fn from(cfg: &Config) -> hap_acc_cfg_t {
        hap_acc_cfg_t {
            name: cfg.name.as_ptr() as *mut i8,
            model: cfg.model.as_ptr() as *mut i8,
            manufacturer: cfg.manufacturer.as_ptr() as *mut i8,
            serial_num: cfg.serial_num.as_ptr() as *mut i8,
            fw_rev: cfg.fw_rev.as_ptr() as *mut i8,
            hw_rev: cfg.hw_rev.as_ptr() as *mut i8,
            pv: cfg.pv.as_ptr() as *mut i8,
            cid: hap_cid_t::from(cfg.cid),
            identify_routine: Some(identify),
        }
    }
}

//impl Into<hap_acc_cfg_t> for Config {
//    fn into(self) -> hap_acc_cfg_t {
//        hap_acc_cfg_t {
//            name: self.name.as_ptr() as *mut i8,
//            model: self.model.as_ptr() as *mut i8,
//            manufacturer: self.manufacturer.as_ptr() as *mut i8,
//            serial_num: self.serial_num.as_ptr() as *mut i8,
//            fw_rev: self.fw_rev.as_ptr() as *mut i8,
//            hw_rev: self.hw_rev.as_ptr() as *mut i8,
//            pv: self.pv.as_ptr() as *mut i8,
//            cid: self.cid.into(),
//            identify_routine: Some(identify),
//        }
//    }
//}

unsafe extern "C" fn identify(acc: *mut hap_acc_t) -> i32 {
    HAP_SUCCESS_
}

pub fn init() {
    info!("Initializig HAP Framework");

    unsafe {
        hap_init(hap_transport_t_HAP_TRANSPORT_WIFI);
    }
}

pub fn start() {
    info!("Starting HAP webserver");

    unsafe {
        hap_start();
    }
}

pub fn add_service_to_accessory(accessory: *mut hap_acc_t, service: *mut hap_serv_t) {
    unsafe {
        hap_acc_add_serv(accessory, service);
    }
}

pub fn add_accessory(accessory: *mut hap_acc_t) {
    unsafe {
        hap_add_accessory(accessory);
    }
}

pub fn secret(code: CString, id: CString) {
    unsafe {
        hap_set_setup_code(code.as_ptr());
        hap_set_setup_id(id.as_ptr());
    }
}
