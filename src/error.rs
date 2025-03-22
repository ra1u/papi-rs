// Copyright 2018-2019 German Research Center for Artificial Intelligence (DFKI)
// Author: Clemens Lutz <clemens.lutz@dfki.de>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.


use std::os::raw::c_int;
use super::ffi;
use core::result;
use std::error::Error;

pub type Result<T> = result::Result<T, Box<dyn Error>>;
//
pub fn check(code: c_int) -> Result<()> {
    match code as u32 {
        ffi::PAPI_OK => Ok(()),
        _ => Err(ErrorKind::PapiError(code).into()),
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    PapiError(c_int),
    InvalidEvent(&'static str),
    InvalidArgument(String),
    OutOfHardwareCounters(&'static str),
}

impl Error for ErrorKind {}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ErrorKind::PapiError(ref e) => write!(f, "PAPI error: {}", e),
            ErrorKind::InvalidEvent(ref e) => write!(f, "Invalid event: {}", e),
            ErrorKind::InvalidArgument(ref e) => write!(f, "Invalid argument: {}", e),
            ErrorKind::OutOfHardwareCounters(ref e) => write!(f, "Out of hardware counters: {}", e),
        }
    }
}
