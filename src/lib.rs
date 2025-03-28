// Copyright 2018-2019 German Research Center for Artificial Intelligence (DFKI)
// Author: Clemens Lutz <clemens.lutz@dfki.de>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

pub mod error;
pub mod event_set;

#[cfg(feature = "criterion")]
pub mod criterion;

use crate::error::Result;

use error::ErrorKind;
use papi_sys as ffi;

use serde_derive::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::io::Read;
use std::path;
use std::sync::Mutex;

static INIT: Mutex<bool> = Mutex::new(false);

#[derive(Debug)]
pub struct Papi {
    config: Option<Config>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    presets: Option<BTreeMap<String, Vec<String>>>,
}

/// PAPI library wrapper
impl Papi {
    /// Initialize PAPI library with parallelism support
    ///
    ///     # extern crate papi;
    ///     # use papi::Papi;
    ///     assert!(Papi::init().is_ok());
    ///
    pub fn init() -> Result<Self> {
        let mut init_m = INIT.lock().unwrap();
        if *init_m {
            return Ok(Papi { config: None });
        }
        if unsafe { ffi::PAPI_is_initialized() } != ffi::PAPI_LOW_LEVEL_INITED as i32 {
            let ver_or_err = unsafe {  ffi::PAPI_library_init(ffi::PAPI_VER_CURRENT) };
            if ver_or_err != ffi::PAPI_VER_CURRENT  {
                return Err(ErrorKind::PapiError(ver_or_err).into());
            }
        }
        // should be called only once
        let err = unsafe {  ffi::PAPI_thread_init(Some(libc::pthread_self)) };
        if err != ffi::PAPI_OK as i32 {
           return Err(ErrorKind::PapiError(err).into());
        }
        *init_m = true;
        Ok(Papi { config: None })
    }

    pub fn init_with_config(config: Config) -> Result<Self> {
        let mut papi = Self::init()?;
        papi.config = Some(config);
        Ok(papi)
    }
}

impl Config {
    /// Load configuration file in TOML format
    ///
    ///     # use std::error::Error;
    ///     # use std::result::Result;
    ///     # use std::path::Path;
    ///     use papi::{Config, Papi};
    ///
    ///     # fn main() -> Result<(), Box<dyn Error>> {
    ///     let path = Path::new("resources/configuration.toml");
    ///     let config = Config::parse_file(path)?;
    ///     let papi = Papi::init_with_config(config)?;
    ///     #
    ///     # Ok(())
    ///     # }
    ///
    pub fn parse_file(config: &path::Path) -> Result<Self> {
        let mut input = String::new();

        fs::File::open(config).and_then(|mut f| f.read_to_string(&mut input))?;

        Self::parse_str(&input)
    }

    /// Load configuration from a string in TOML format
    ///
    ///     # use std::error::Error;
    ///     # use std::result::Result;
    ///     use papi::Config;
    ///
    ///     # fn main() -> Result<(), Box<dyn Error>> {
    ///     let config_str = r#"
    ///     [presets]
    ///     Test1 = ["UOPS_RETIRED:ALL", "UOPS_RETIRED:STALL_CYCLES"]
    ///     Test2 = ["UOPS_EXECUTED:CORE", "UOPS_EXECUTED:STALL_CYCLES"]
    ///     Test3 = ["UOPS_EXECUTED:THREAD"]
    ///     "#;
    ///
    ///     let config = Config::parse_str(&config_str)?;
    ///     #
    ///     # Ok(())
    ///     # }
    ///
    pub fn parse_str(config: &str) -> Result<Self> {
        let deserialized: Self = toml::from_str(&config)?;
        Ok(deserialized)
    }
}
