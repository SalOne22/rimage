use std::{num::NonZeroUsize, thread};

pub fn num_threads() -> usize {
    thread::available_parallelism()
        .unwrap_or(NonZeroUsize::new(4).unwrap())
        .get()
}
