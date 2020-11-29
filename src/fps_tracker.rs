use std::collections::VecDeque;
use std::time::Duration;

use crate::time;

pub(crate) struct FpsTracker {
    samples: VecDeque<f64>,
}

impl FpsTracker {
    pub fn new() -> Self {
        let mut samples = VecDeque::with_capacity(200);
        samples.resize(200, 1.0 / 60.0);

        Self { samples }
    }

    pub fn fps(&self) -> f64 {
        1.0 / (self.samples.iter().sum::<f64>() / self.samples.len() as f64)
    }

    pub fn tick(&mut self, elapsed_time: Duration) {
        self.samples.pop_front();
        self.samples.push_back(time::duration_to_f64(elapsed_time));
    }
}
