use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;
use std::time::{Duration, Instant};

#[derive(Default, Debug)]
pub struct MeasureContext {
    measurements: IndexMap<&'static str, Duration, FxBuildHasher>,
}

impl MeasureContext {
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    pub fn measure<T>(&mut self, label: &'static str, f: impl FnOnce() -> T) -> T {
        let start = Instant::now();

        let result = f();

        let duration = start.elapsed();
        *self.measurements.entry(label).or_insert(Duration::ZERO) += duration;
        result
    }

    pub fn measurements(&self) -> impl Iterator<Item = (&'static str, Duration)> {
        self.measurements
            .iter()
            .map(|(label, duration)| (*label, *duration))
    }
}
