// Copyright 2019 German Research Center for Artificial Intelligence (DFKI)
// Author: Clemens Lutz <clemens.lutz@dfki.de>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use criterion::measurement::ValueFormatter;
use criterion::Throughput;

/// An adapter for Criterion that formats PAPI samples
#[derive(Clone, Debug)]
pub(crate) struct SampleFormatter {
    event_name: &'static str,
}

impl SampleFormatter {
    /// Creates a new SampleFormatter containing an event name
    pub(crate) fn new(event_name: &'static str) -> Self {
        Self { event_name }
    }
}

impl SampleFormatter {
    /// Calculates throughput in bytes per event
    fn bytes_per_event(&self, bytes: f64, values: &mut [f64]) -> &'static str {
        values.iter_mut().for_each(|val| *val = bytes / *val);
        "Bytes/event"
    }

    /// Calculates throughput in elements per event
    fn elements_per_event(&self, elems: f64, values: &mut [f64]) -> &'static str {
        values.iter_mut().for_each(|val| *val = elems / *val);
        "elems/event"
    }
}

impl ValueFormatter for SampleFormatter {
    fn scale_values(&self, _typical_value: f64, _values: &mut [f64]) -> &'static str {
        self.event_name
    }

    fn scale_throughputs(
        &self,
        _typical_value: f64,
        throughput: &Throughput,
        values: &mut [f64],
    ) -> &'static str {
        match *throughput {
            Throughput::Bytes(bytes) => self.bytes_per_event(bytes as f64, values),
            Throughput::BytesDecimal(bytes) => self.bytes_per_event(bytes as f64, values),
            Throughput::Elements(elems) => self.elements_per_event(elems as f64, values),
        }
    }

    fn scale_for_machines(&self, _values: &mut [f64]) -> &'static str {
        self.event_name
    }
}
