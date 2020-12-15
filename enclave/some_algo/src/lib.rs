#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

use std::prelude::v1::*;
use std::sync::{Arc, SgxMutex};

extern crate rand;
extern crate sgx_rand;
extern crate rustc_serialize as serialize;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate prost;
extern crate prost_types;
extern crate bytes;
extern crate anyhow;
extern crate mio;

use anyhow::Result;

pub(crate) mod xgb_proto {
    include!(concat!(env!("OUT_DIR"), "/some_algo.rs"));
}
