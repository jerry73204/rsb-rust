use crate::{common::*, transpose::Transpose};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpmmOptions {
    pub num_threads: Option<usize>,
    pub max_rounds: usize,
    pub limit: Limit,
    pub transpose: Transpose,
}

impl Default for SpmmOptions {
    fn default() -> Self {
        Self {
            num_threads: None,
            max_rounds: 0,
            limit: Default::default(),
            transpose: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Limit {
    Default,
    Time(Duration),
    Iterations(usize),
}

impl Limit {
    fn to_value(&self) -> sys::rsb_time_t {
        match *self {
            Self::Default => 0.0,
            Self::Time(dur) => dur.as_secs_f64(),
            Self::Iterations(count) => -(count as f64),
        }
    }
}

impl Default for Limit {
    fn default() -> Self {
        Self::Default
    }
}
