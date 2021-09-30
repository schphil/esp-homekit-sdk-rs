use log::info;

use crate::*;

#[derive(Clone, Copy)]
pub enum Category {
    NONE,
    OTHER,
    BRIDGE,
    FAN,
    GARAGE_DOOR_OPENER,
    LIGHTING,
    LOCK,
    OUTLET,
    SWITCH,
    THERMOSTAT,
    SENSOR,
    SECURITY_SYSTEM,
    DOOR,
    WINDOW,
    WINDOW_COVERING,
    PROGRAMMABLE_SWITCH,
    RESERVED,
    IP_CAMERA,
    VIDEO_DOORBELL,
    AIR_PURIFIER,
    HEATER,
    AIR_CONDITIONER,
    HUMIDIFIER,
    DEHUMIDIFIER,
    MAX,
}

impl From<Category> for hap_cid_t {
    fn from(category: Category) -> hap_cid_t {
        match category {
            Category::NONE => hap_cid_t_HAP_CID_NONE,
            Category::OTHER => hap_cid_t_HAP_CID_OTHER,
            Category::BRIDGE => hap_cid_t_HAP_CID_BRIDGE,
            Category::FAN => hap_cid_t_HAP_CID_FAN,
            Category::GARAGE_DOOR_OPENER => hap_cid_t_HAP_CID_GARAGE_DOOR_OPENER,
            Category::LIGHTING => hap_cid_t_HAP_CID_LIGHTING,
            Category::LOCK => hap_cid_t_HAP_CID_LOCK,
            Category::OUTLET => hap_cid_t_HAP_CID_OUTLET,
            Category::SWITCH => hap_cid_t_HAP_CID_SWITCH,
            Category::THERMOSTAT => hap_cid_t_HAP_CID_THERMOSTAT,
            Category::SENSOR => hap_cid_t_HAP_CID_SENSOR,
            Category::SECURITY_SYSTEM => hap_cid_t_HAP_CID_SECURITY_SYSTEM,
            Category::DOOR => hap_cid_t_HAP_CID_DOOR,
            Category::WINDOW => hap_cid_t_HAP_CID_WINDOW,
            Category::WINDOW_COVERING => hap_cid_t_HAP_CID_WINDOW_COVERING,
            Category::PROGRAMMABLE_SWITCH => hap_cid_t_HAP_CID_PROGRAMMABLE_SWITCH,
            Category::RESERVED => hap_cid_t_HAP_CID_RESERVED,
            Category::IP_CAMERA => hap_cid_t_HAP_CID_IP_CAMERA,
            Category::VIDEO_DOORBELL => hap_cid_t_HAP_CID_VIDEO_DOORBELL,
            Category::AIR_PURIFIER => hap_cid_t_HAP_CID_AIR_PURIFIER,
            Category::HEATER => hap_cid_t_HAP_CID_HEATER,
            Category::AIR_CONDITIONER => hap_cid_t_HAP_CID_AIR_CONDITIONER,
            Category::HUMIDIFIER => hap_cid_t_HAP_CID_HUMIDIFIER,
            Category::DEHUMIDIFIER => hap_cid_t_HAP_CID_DEHUMIDIFIER,
            Category::MAX => hap_cid_t_HAP_CID_MAX,
        }
    }
}


//impl Into<hap_cid_t> for Category {
//    fn into(self) -> hap_cid_t {
//        match self {
//            Category::NONE => hap_cid_t_HAP_CID_NONE,
//            Category::OTHER => hap_cid_t_HAP_CID_OTHER,
//            Category::BRIDGE => hap_cid_t_HAP_CID_BRIDGE,
//            Category::FAN => hap_cid_t_HAP_CID_FAN,
//            Category::GARAGE_DOOR_OPENER => hap_cid_t_HAP_CID_GARAGE_DOOR_OPENER,
//            Category::LIGHTING => hap_cid_t_HAP_CID_LIGHTING,
//            Category::LOCK => hap_cid_t_HAP_CID_LOCK,
//            Category::OUTLET => hap_cid_t_HAP_CID_OUTLET,
//            Category::SWITCH => hap_cid_t_HAP_CID_SWITCH,
//            Category::THERMOSTAT => hap_cid_t_HAP_CID_THERMOSTAT,
//            Category::SENSOR => hap_cid_t_HAP_CID_SENSOR,
//            Category::SECURITY_SYSTEM => hap_cid_t_HAP_CID_SECURITY_SYSTEM,
//            Category::DOOR => hap_cid_t_HAP_CID_DOOR,
//            Category::WINDOW => hap_cid_t_HAP_CID_WINDOW,
//            Category::WINDOW_COVERING => hap_cid_t_HAP_CID_WINDOW_COVERING,
//            Category::PROGRAMMABLE_SWITCH => hap_cid_t_HAP_CID_PROGRAMMABLE_SWITCH,
//            Category::RESERVED => hap_cid_t_HAP_CID_RESERVED,
//            Category::IP_CAMERA => hap_cid_t_HAP_CID_IP_CAMERA,
//            Category::VIDEO_DOORBELL => hap_cid_t_HAP_CID_VIDEO_DOORBELL,
//            Category::AIR_PURIFIER => hap_cid_t_HAP_CID_AIR_PURIFIER,
//            Category::HEATER => hap_cid_t_HAP_CID_HEATER,
//            Category::AIR_CONDITIONER => hap_cid_t_HAP_CID_AIR_CONDITIONER,
//            Category::HUMIDIFIER => hap_cid_t_HAP_CID_HUMIDIFIER,
//            Category::DEHUMIDIFIER => hap_cid_t_HAP_CID_DEHUMIDIFIER,
//            Category::MAX => hap_cid_t_HAP_CID_MAX,
//        }
//    }
//}

pub fn create(cfg: &hap::Config) -> *mut hap_acc_t {
    
    info!("Creating accessory");

    unsafe {
        hap_acc_create(&mut hap_acc_cfg_t::from(cfg) as *mut hap_acc_cfg_t)
    }
}
