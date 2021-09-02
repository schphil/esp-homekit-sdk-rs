#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use esp_idf_sys::c_types;

include!(env!("EMBUILD_GENERATED_BINDINGS_FILE"));
